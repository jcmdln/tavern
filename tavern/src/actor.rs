use crate::{core::Object, traits::ObjectTrait};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Application {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Group {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Person {
    #[serde(flatten)]
    pub extends: Object,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Service {
    #[serde(flatten)]
    pub extends: Object,
}

macro_rules! actor_impl {
    ($($t:ty)*) => ($(
        impl Default for $t {
            fn default() -> Self {
                Self {
                    extends: $crate::traits::StreamTrait::kind(
                        &mut $crate::core::Object::default(),
                        $crate::property::Type::String(stringify!($t).into()),
                    )
                    .clone(),
                }
            }
        }

        impl $crate::traits::StreamTrait for $t {
            fn as_stream(&mut self) -> &mut $crate::core::Stream {
                &mut self.as_object().extends
            }
        }

        impl $crate::traits::ObjectTrait for $t {
            fn as_object(&mut self) -> &mut $crate::core::Object {
                &mut self.extends
            }
        }
    )*)
}

actor_impl! { Actor Application Group Organization Person Service }
