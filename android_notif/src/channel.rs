use android_api_util::CompatEnv;
use jni::errors::Error;
use jni::objects::JValue;
use jni::sys::jint;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Importance {
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_DEFAULT
    Default,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_HIGH
    High,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_LOW
    Low,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_MAX
    Max,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_MIN
    Min,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_NONE
    None,
    /// API 24: https://developer.android.com/reference/android/app/NotificationManager#IMPORTANCE_UNSPECIFIED
    Unspecified,
}

impl Importance {
    fn internal_name(&self) -> &'static str {
        match self {
            Importance::Default => "IMPORTANCE_DEFAULT",
            Importance::High => "IMPORTANCE_HIGH",
            Importance::Low => "IMPORTANCE_LOW",
            Importance::Max => "IMPORTANCE_MAX",
            Importance::Min => "IMPORTANCE_MIN",
            Importance::None => "IMPORTANCE_NONE",
            Importance::Unspecified => "IMPORTANCE_UNSPECIFIED",
        }
    }

    /// API 24
    pub fn internal_value(&self, env: CompatEnv<'_>) -> Result<jint, Error> {
        // API 1: https://developer.android.com/reference/android/app/NotificationManager
        let class = env.find_class("android/app/NotificationManager")?;
        // API 24 as per all possible fields for Self
        let value = env.get_static_field(class, self.internal_name(), "I")?;
        Ok(value.i()?)
    }
}

pub type NotificationChannelID<'a> = &'a str;

pub struct NotificationChannel<'a> {
    pub id: NotificationChannelID<'a>,
    pub name: String,
    pub desc: Option<String>,
    pub importance: Importance,
}

/// Supports API 1
///
/// API 4
fn notification_channel_available(env: CompatEnv<'_>) -> Result<bool, Error> {
    // API 1: https://developer.android.com/reference/android/os/Build.VERSION
    let version_class = env.find_class("android/os/Build$VERSION")?;
    // API 4: https://developer.android.com/reference/android/os/Build.VERSION_CODES
    let version_codes = env.try_find_class("android/os/Build$VERSION_CODES")?;

    let version_codes = if let Some(x) = version_codes {
        x
    } else {
        return Ok(false);
    };

    // API 4: https://developer.android.com/reference/android/os/Build.VERSION#SDK_INT
    let version_value = env.try_get_static_field(version_class, "SDK_INT", "I")?;

    let version_value = if let Some(x) = version_value {
        x.i()?
    } else {
        return Ok(false);
    };

    let o_version = env.try_get_static_field(version_codes, "O", "I")?;

    Ok(o_version
        .map(|x| x.i())
        .transpose()?
        .map(|x| version_value >= x)
        .unwrap_or(false))
}

/// Supports API 1
///
/// API 26
pub fn create_notification_channel(
    channel_cfg: NotificationChannel<'_>,
    env: CompatEnv<'_>,
) -> Result<(), Error> {
    if !notification_channel_available(env)? {
        return Ok(());
    }

    let name = env.new_string(&channel_cfg.name)?;
    let desc = channel_cfg
        .desc
        .as_ref()
        .map(|x| env.new_string(&**x))
        .transpose()?;
    let importance = channel_cfg.importance.internal_value(env)?;
    let id = env.new_string(channel_cfg.id)?;

    // API 26: https://developer.android.com/reference/android/app/NotificationChannel?hl=en#NotificationChannel(java.lang.String,%20java.lang.CharSequence,%20int)
    let channel = env.new_object(
        "android/app/NotificationChannel",
        "(Ljava/lang/String;Ljava/lang/CharSequence;I)V",
        &[
            JValue::Object(*id),
            JValue::Object(*name),
            JValue::Int(importance),
        ],
    )?;

    if let Some(desc) = desc {
        // API 26: https://developer.android.com/reference/android/app/NotificationChannel?hl=en#setDescription(java.lang.String)
        env.call_method(
            channel,
            "setDescription",
            "(Ljava/lang/String;)V",
            &[JValue::Object(*desc)],
        )?;
    }

    // API 1: https://developer.android.com/reference/android/content/Context#NOTIFICATION_SERVICE
    let notif_manager = env
        .get_static_field(
            "android/content/Context",
            "NOTIFICATION_SERVICE",
            "Ljava/lang/String;",
        )?
        .l()?;

    // API 1: https://developer.android.com/reference/android/content/Context#getSystemService(java.lang.String)
    let manager = env
        .call_method(
            env.context,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(notif_manager)],
        )?
        .l()?;

    // API 26: https://developer.android.com/reference/android/app/NotificationManager#createNotificationChannel(android.app.NotificationChannel)
    env.call_method(
        manager,
        "createNotificationChannel",
        "(Landroid/app/NotificationChannel;)V",
        &[JValue::Object(channel)],
    )?;

    Ok(())
}
