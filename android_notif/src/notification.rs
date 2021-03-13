use crate::channel::NotificationChannelID;
use android_api_util::CompatEnv;
use jni::errors::Error;
use jni::objects::{JObject, JValue};
use jni::strings::JNIString;
use jni::sys::jint;
use once_cell::sync::OnceCell;

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub struct ActivityFlags {
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_BROUGHT_TO_FRONT
    pub brought_to_front:       jint,
    /// API 11: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_CLEAR_TASK
    pub clear_task:             Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_CLEAR_TOP
    pub clear_top:              jint,
    /// API 3: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_CLEAR_WHEN_TASK_RESET
    /// DEPRECATED in API 21
    pub clear_when_task_reset:  Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_EXCLUDE_FROM_RECENTS
    pub exclude_from_recents:   jint,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_FORWARD_RESULT
    pub forward_result:         jint,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_LAUNCHED_FROM_HISTORY
    pub launched_from_history:  jint,
    /// API 24: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_LAUNCH_ADJACENT
    pub launch_adjacent:        Option<jint>,
    /// API 28: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_LAUNCH_ADJACENT
    pub match_external:         Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_MULTIPLE_TASK
    pub multiple_task:          jint,
    /// API 21: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_DOCUMENT
    pub new_document:           Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_TASK
    pub new_task:               jint,
    /// API 5: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NO_ANIMATION
    pub no_animation:           Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NO_HISTORY
    pub no_history:             jint,
    /// API 3: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NO_USER_ACTION
    pub no_user_action:         Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_PREVIOUS_IS_TOP
    pub previous_is_top:        jint,
    /// API 3: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_REORDER_TO_FRONT
    pub reorder_to_front:       Option<jint>,
    /// API 30: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_REQUIRE_DEFAULT
    pub require_default:        Option<jint>,
    /// API 30: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_REQUIRE_NON_BROWSER
    pub require_non_browser:    Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_RESET_TASK_IF_NEEDED
    pub reset_task_if_needed:   jint,
    /// API 21: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_RETAIN_IN_RECENTS
    pub retain_in_recents:      Option<jint>,
    /// API 1: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_SINGLE_TOP
    pub single_top:             Option<jint>,
    /// API 11: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_TASK_ON_HOME
    pub task_on_home:           Option<jint>,
}

struct ActivityFlagLoader<'a>(CompatEnv<'a>);

impl<'a> ActivityFlagLoader<'a> {
    pub fn load(&self) -> Result<ActivityFlags, Error> {
        let env = self.0;

        // API 1: https://developer.android.com/reference/android/content/Intent
        let intent = env.find_class("android/content/Intent")?;

        let load = |name: &str| -> Result<Option<jint>, Error> {
            env.try_get_static_field(intent, name, "I")
                .transpose()
                .map(|x| x.and_then(|x| x.i()))
                .transpose()
        };

        let load_yes = |name: &str| -> Result<jint, Error> {
            load(name)?.ok_or_else(|| Error::FieldNotFound {
                sig: "I".into(),
                name: name.into(),
            })
        };

        let value = ActivityFlags {
            brought_to_front: load_yes("FLAG_ACTIVITY_BROUGHT_TO_FRONT")?,
            clear_task: load("FLAG_ACTIVITY_CLEAR_TASK")?,
            clear_top: load_yes("FLAG_ACTIVITY_CLEAR_TOP")?,
            clear_when_task_reset: load("FLAG_ACTIVITY_CLEAR_WHEN_TASK_RESET")?,
            exclude_from_recents: load_yes("FLAG_ACTIVITY_EXCLUDE_FROM_RECENTS")?,
            forward_result: load_yes("FLAG_ACTIVITY_FORWARD_RESULT")?,
            launched_from_history: load_yes("FLAG_ACTIVITY_LAUNCHED_FROM_HISTORY")?,
            launch_adjacent: load("FLAG_ACTIVITY_LAUNCH_ADJACENT")?,
            match_external: load("FLAG_ACTIVITY_MATCH_EXTERNAL")?,
            multiple_task: load_yes("FLAG_ACTIVITY_MULTIPLE_TASK")?,
            new_document: load("FLAG_ACTIVITY_NEW_DOCUMENT")?,
            new_task: load_yes("FLAG_ACTIVITY_NEW_TASK")?,
            no_animation: load("FLAG_ACTIVITY_NO_ANIMATION")?,
            no_history: load_yes("FLAG_ACTIVITY_NO_HISTORY")?,
            no_user_action: load("FLAG_ACTIVITY_NO_USER_ACTION")?,
            previous_is_top: load_yes("FLAG_ACTIVITY_PREVIOUS_IS_TOP")?,
            reorder_to_front: load("FLAG_ACTIVITY_REORDER_TO_FRONT")?,
            require_default: load("FLAG_ACTIVITY_REQUIRE_DEFAULT")?,
            require_non_browser: load("FLAG_ACTIVITY_REQUIRE_NON_BROWSER")?,
            reset_task_if_needed: load_yes("FLAG_ACTIVITY_RESET_TASK_IF_NEEDED")?,
            retain_in_recents: load("FLAG_ACTIVITY_RETAIN_IN_RECENTS")?,
            single_top: load("FLAG_ACTIVITY_SINGLE_TOP")?,
            task_on_home: load("FLAG_ACTIVITY_TASK_ON_HOME")?,
        };

        Ok(value)
    }
}

/// Supports API 1
///
/// API 30
pub fn activity_flags(env: CompatEnv<'_>) -> &'static ActivityFlags {
    static FLAGS: OnceCell<ActivityFlags> = OnceCell::new();

    FLAGS.get_or_init(move || ActivityFlagLoader(env).load().unwrap())
}

/// API 1
pub fn create_intent(env: CompatEnv<'_>, flags: jint) -> Result<JObject<'_>, Error> {
    // API 1: https://developer.android.com/reference/android/content/Intent
    let class = env.find_class("android/content/Intent")?;

    // API 1: https://developer.android.com/reference/android/content/Intent#Intent(android.content.Context,%20java.lang.Class%3C?%3E)
    let intent = env.new_object(
        class,
        "(Landroid/content/Context;Ljava/lang/Class;)V",
        &[
            JValue::Object(env.context),
            JValue::Object(*env.get_object_class(env.context)?),
        ],
    )?;

    // API 1: https://developer.android.com/reference/android/content/Intent#setFlags(int)
    env.call_method(
        intent,
        "setFlags",
        "(I)Landroid/content/Intent;",
        &[JValue::Int(flags)],
    )?;

    Ok(intent)
}

/// API 1
pub fn pending_intent<'a>(env: CompatEnv<'a>, intent: JObject<'_>) -> Result<JObject<'a>, Error> {
    // API 1: https://developer.android.com/reference/android/app/PendingIntent
    let class = env.find_class("android/app/PendingIntent")?;
    // API 1: https://developer.android.com/reference/android/app/PendingIntent#getActivity(android.content.Context,%20int,%20android.content.Intent,%20int)
    let value = env
        .call_static_method(
            class,
            "getActivity",
            "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;",
            &[
                JValue::Object(env.context),
                JValue::Int(0),
                JValue::Object(intent),
                JValue::Int(0),
            ],
        )?
        .l()?;

    Ok(value)
}

#[derive(Copy, Clone)]
pub struct NotificationBuilder<'a> {
    internal: JObject<'a>,
    env: CompatEnv<'a>,
}

impl<'a> NotificationBuilder<'a> {
    /// Supports API 11
    ///
    /// API 26
    pub fn new(env: CompatEnv<'a>, channel_id: NotificationChannelID<'_>) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder
        let class = env.find_class("android/app/Notification$Builder")?;

        // API 26: https://developer.android.com/reference/android/app/Notification.Builder#Builder(android.content.Context,%20java.lang.String)
        // Fallback API 11: https://developer.android.com/reference/android/app/Notification.Builder#Builder(android.content.Context)
        let builder = env
            .try_new_object(
                class,
                "(Landroid/content/Context;Ljava/lang/String;)V",
                &[
                    JValue::Object(env.context),
                    JValue::Object(*env.new_string(channel_id)?),
                ],
            )
            .transpose()
            .unwrap_or_else(|| {
                env.new_object(
                    class,
                    "(Landroid/content/Context;)V",
                    &[JValue::Object(env.context)],
                )
            })?;

        Ok(Self {
            internal: builder,
            env,
        })
    }

    /// API 11
    pub fn set_intent(&self, intent: JObject<'_>) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder#setContentIntent(android.app.PendingIntent)
        self.env.call_method(
            self.internal,
            "setContentIntent",
            "(Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;",
            &[JValue::Object(intent)],
        )?;

        Ok(*self)
    }

    /// API 11
    pub fn set_title(&self, title: impl Into<JNIString>) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder#setContentTitle(java.lang.CharSequence)
        self.env.call_method(
            self.internal,
            "setContentTitle",
            "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
            &[JValue::Object(*self.env.new_string(title)?)],
        )?;

        Ok(*self)
    }

    /// API 11
    pub fn set_content_text(&self, content: impl Into<JNIString>) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder#setContentText(java.lang.CharSequence)
        self.env.call_method(
            self.internal,
            "setContentText",
            "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
            &[JValue::Object(*self.env.new_string(content)?)],
        )?;

        Ok(*self)
    }

    /// API 11
    pub fn set_auto_cancel(&self, auto_cancel: bool) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder#setAutoCancel(boolean)
        self.env.call_method(
            self.internal,
            "setAutoCancel",
            "(Z)Landroid/app/Notification$Builder;",
            &[JValue::Bool(auto_cancel as u8)],
        )?;

        Ok(*self)
    }

    /// API 11
    pub fn set_small_icon(&self, icon: jint) -> Result<Self, Error> {
        // API 11: https://developer.android.com/reference/android/app/Notification.Builder#setSmallIcon(int)
        self.env.call_method(
            self.internal,
            "setSmallIcon",
            "(I)Landroid/app/Notification$Builder;",
            &[JValue::Int(icon)],
        )?;

        Ok(*self)
    }

    /// Supports API 11
    ///
    /// API 16
    fn build(&self) -> Result<JObject<'a>, Error> {
        // API 16: https://developer.android.com/reference/android/app/Notification.Builder#build()
        // Fallback API 11: https://developer.android.com/reference/android/app/Notification.Builder#getNotification()
        let x = self
            .env
            .try_call_method(self.internal, "build", "()Landroid/app/Notification;", &[])
            .transpose()
            .unwrap_or_else(|| {
                self.env.call_method(
                    self.internal,
                    "getNotification",
                    "()Landroid/app/Notification;",
                    &[],
                )
            })?;

        x.l()
    }
}

#[derive(Copy, Clone)]
pub struct NotificationManager<'a> {
    internal: JObject<'a>,
    env: CompatEnv<'a>,
}

impl<'a> NotificationManager<'a> {
    /// API 1
    pub fn new(env: CompatEnv<'a>) -> Result<Self, Error> {
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

        Ok(Self {
            internal: manager,
            env,
        })
    }

    /// Supports API 11
    ///
    /// API 16
    pub fn notify(&self, notif: &NotificationBuilder<'_>, id: jint) -> Result<(), Error> {
        // Min API 11
        let notif_obj = notif.build()?;

        // API 1: https://developer.android.com/reference/android/app/NotificationManager#notify(int,%20android.app.Notification)
        self.env.call_method(
            self.internal,
            "notify",
            "(ILandroid/app/Notification;)V",
            &[JValue::Int(id), JValue::Object(notif_obj)],
        )?;

        Ok(())
    }
}
