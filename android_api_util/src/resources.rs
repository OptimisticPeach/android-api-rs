use crate::CompatEnv;
use jni::errors::Error;
use jni::objects::{JObject, JValue};
use jni::strings::JNIString;
use jni::sys::jint;
use std::collections::HashMap;

pub struct ResourceManager<'a> {
    env: CompatEnv<'a>,
    resources: JObject<'a>,
    package: JObject<'a>,
    previous_resources: HashMap<String, jint>,
}

impl<'a> ResourceManager<'a> {
    pub const DRAWABLE: &'static str = "drawable";

    /// API 1
    pub fn new(env: CompatEnv<'a>, context: JObject<'a>) -> Result<Self, Error> {
        // API 1: https://developer.android.com/reference/android/content/Context#getResources()
        let resources = env
            .call_method(
                context,
                "getResources",
                "()Landroid/content/res/Resources;",
                &[],
            )?
            .l()?;

        // API 1: https://developer.android.com/reference/android/content/Context#getPackageName()
        let package = env
            .call_method(context, "getPackageName", "()Ljava/lang/String;", &[])?
            .l()?;

        let x = Self {
            env,
            resources,
            package,
            previous_resources: HashMap::new(),
        };

        Ok(x)
    }

    /// API 1
    pub fn get(
        &mut self,
        name: impl AsRef<str> + Into<JNIString>,
        kind: impl Into<JNIString>,
    ) -> Result<jint, Error> {
        match self.previous_resources.get(name.as_ref()) {
            Some(x) => Ok(*x),
            None => {
                let owned = name.as_ref().to_owned();

                // API 1: https://developer.android.com/reference/android/content/res/Resources#getIdentifier(java.lang.String,%20java.lang.String,%20java.lang.String)
                let val = self
                    .env
                    .call_method(
                        self.resources,
                        "getIdentifier",
                        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I",
                        &[
                            JValue::Object(*self.env.new_string(name)?),
                            JValue::Object(*self.env.new_string(kind)?),
                            JValue::Object(self.package),
                        ],
                    )
                    .and_then(|x| x.i());

                match val {
                    Ok(x) => {
                        self.previous_resources.insert(owned, x);
                        Ok(x)
                    }
                    x => x,
                }
            }
        }
    }
}
