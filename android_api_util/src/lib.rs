use jni::descriptors::Desc;
use jni::errors::Error;
use jni::objects::{JClass, JObject, JValue};
use jni::strings::JNIString;
use jni::{AttachGuard, JNIEnv};
use ndk_glue::native_activity;
use std::ops::{Deref, DerefMut};

pub mod resources;

#[derive(Copy, Clone)]
pub struct CompatEnv<'a> {
    pub env: JNIEnv<'a>,
    pub context: JObject<'a>,
    pub class_not_found_exception: JClass<'a>,
    pub no_such_field_exception: JClass<'a>,
    pub no_such_method_exception: JClass<'a>,
    pub no_class_def_found_error: JClass<'a>,
    pub no_such_field_error: JClass<'a>,
    pub no_such_method_error: JClass<'a>,
}

impl<'a> CompatEnv<'a> {
    pub fn new(guard: &'a AttachGuard<'a>) -> Result<Self, Error> {
        let env = **guard;

        // Should all be available.
        let class = env.find_class("java/lang/ClassNotFoundException")?;
        let field = env.find_class("java/lang/NoSuchFieldException")?;
        let method = env.find_class("java/lang/NoSuchMethodException")?;
        let class_err = env.find_class("java/lang/NoClassDefFoundError")?;
        let field_err = env.find_class("java/lang/NoSuchFieldError")?;
        let method_err = env.find_class("java/lang/NoSuchMethodError")?;

        Ok(Self {
            env,
            context: JObject::from(native_activity().activity()),
            class_not_found_exception: class,
            no_such_field_exception: field,
            no_such_method_exception: method,
            no_class_def_found_error: class_err,
            no_such_field_error: field_err,
            no_such_method_error: method_err,
        })
    }

    fn try_do<T>(
        &self,
        val: Result<T, Error>,
        ignore: &'_ [JClass<'a>],
    ) -> Result<Option<T>, Error> {
        match val {
            Ok(x) => Ok(Some(x)),
            Err(Error::JavaException) => {
                let exception = self.env.exception_occurred()?;

                self.env.exception_clear()?;

                for i in ignore {
                    if self.env.is_instance_of(*exception, *i)? {
                        return Ok(None);
                    }
                }

                self.env.throw(exception)?;

                Err(Error::JavaException)
            }
            Err(e) => Err(e),
        }
    }

    pub fn try_find_class<S>(&self, s: S) -> Result<Option<JClass<'a>>, Error>
    where
        S: Into<JNIString>,
    {
        self.try_do(
            self.env.find_class(s),
            &[
                self.class_not_found_exception,
                self.no_class_def_found_error,
            ],
        )
    }

    pub fn try_get_field<O, S, T>(
        &self,
        obj: O,
        name: S,
        ty: T,
    ) -> Result<Option<JValue<'a>>, Error>
    where
        O: Into<JObject<'a>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        self.try_do(
            self.env.get_field(obj, name, ty),
            &[self.no_such_field_exception, self.no_such_field_error],
        )
    }

    pub fn try_get_static_field<'c, T, U, V>(
        &self,
        class: T,
        field: U,
        sig: V,
    ) -> Result<Option<JValue<'a>>, Error>
    where
        T: Desc<'a, JClass<'c>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        self.try_do(
            self.env.get_static_field(class, field, sig),
            &[self.no_such_field_exception, self.no_such_field_error],
        )
    }

    pub fn try_call_method<O, S, T>(
        &self,
        obj: O,
        name: S,
        sig: T,
        args: &[JValue],
    ) -> Result<Option<JValue<'a>>, Error>
    where
        O: Into<JObject<'a>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        self.try_do(
            self.env.call_method(obj, name, sig, args),
            &[self.no_such_method_exception, self.no_such_method_error],
        )
    }

    pub fn try_call_static_method<'c, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
        args: &[JValue],
    ) -> Result<Option<JValue<'a>>, Error>
    where
        T: Desc<'a, JClass<'c>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        self.try_do(
            self.env.call_static_method(class, name, sig, args),
            &[self.no_such_method_exception, self.no_such_method_error],
        )
    }

    pub fn try_new_object<'c, T, U>(
        &self,
        class: T,
        ctor_sig: U,
        ctor_args: &[JValue],
    ) -> Result<Option<JObject<'a>>, Error>
    where
        T: Desc<'a, JClass<'c>>,
        U: Into<JNIString> + AsRef<str>,
    {
        self.try_do(
            self.env.new_object(class, ctor_sig, ctor_args),
            &[self.no_such_method_exception, self.no_such_method_error],
        )
    }
}

impl<'a> Deref for CompatEnv<'a> {
    type Target = JNIEnv<'a>;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl<'a> DerefMut for CompatEnv<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}
