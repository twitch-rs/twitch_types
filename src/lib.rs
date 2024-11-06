#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![allow(clippy::extra_unused_lifetimes)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
//! Twitch types

#[macro_use]
#[doc(hidden)]
pub mod macros;
mod collection;

pub use collection::{Collection, CollectionIter};

/// Convert a type into a [`Cow`](std::borrow::Cow)
pub trait IntoCow<'a, Ref: ?Sized>
where Ref: ToOwned {
    /// Make the cow with proper ownership, muu
    fn into_cow(self) -> std::borrow::Cow<'a, Ref>
    where &'a Self: 'a;
}

impl<'a, R, S> IntoCow<'a, R> for std::borrow::Cow<'a, S>
where
    R: ToOwned + ?Sized + 'a,
    S: ToOwned + ?Sized + 'a,
    S::Owned: Into<R::Owned>,
    &'a R: From<&'a S>,
{
    fn into_cow(self) -> std::borrow::Cow<'a, R> {
        match self {
            std::borrow::Cow::Borrowed(b) => std::borrow::Cow::Borrowed(b.into()),
            std::borrow::Cow::Owned(o) => std::borrow::Cow::Owned(o.into()),
        }
    }
}

impl<'a, R> IntoCow<'a, R> for &'a str
where
    &'a str: Into<&'a R>,
    R: ToOwned + ?Sized + 'a,
{
    fn into_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Borrowed(self.into()) }
}

impl<'a, R> IntoCow<'a, R> for &'a String
where
    &'a String: Into<&'a R>,
    R: ToOwned + ?Sized + 'a,
{
    fn into_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Borrowed(self.into()) }
}

impl<'a, R> IntoCow<'a, R> for String
where
    String: Into<R::Owned>,
    R: ToOwned + ?Sized + 'a,
{
    fn into_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Owned(self.into()) }
}

mod basic;
// cc: https://github.com/rust-lang/rust/issues/83428, can't use glob imports and keep the modules private
#[cfg(feature = "chat")]
/// types for chat
pub mod chat;
#[cfg(feature = "color")]
/// types for colors
pub mod color;
#[cfg(feature = "emote")]
/// types for emotes
pub mod emote;
#[cfg(feature = "entitlement")]
/// types for entitlements
pub mod entitlement;
#[cfg(feature = "eventsub")]
/// types for eventsub related things
pub mod eventsub;
#[cfg(feature = "extension")]
/// types for extensions
pub mod extension;
#[cfg(feature = "goal")]
/// types for goals
pub mod goal;
#[cfg(feature = "moderation")]
/// types for moderation
pub mod moderation;
#[cfg(feature = "points")]
/// types for points
pub mod points;
#[cfg(feature = "stream")]
/// types for stream related things
pub mod stream;
#[cfg(feature = "sub")]
/// types for subscriptions
pub mod sub;
#[cfg(feature = "timestamp")]
/// types for time
pub mod time;
#[cfg(feature = "user")]
/// types for user related things
pub mod user;

pub use basic::*;

#[cfg(feature = "chat")]
pub use crate::chat::*;
#[cfg(feature = "color")]
pub use crate::color::*;
#[cfg(feature = "emote")]
pub use crate::emote::*;
#[cfg(feature = "entitlement")]
pub use crate::entitlement::*;
#[cfg(feature = "eventsub")]
pub use crate::eventsub::*;
#[cfg(feature = "extension")]
pub use crate::extension::*;
#[cfg(feature = "goal")]
pub use crate::goal::*;
#[cfg(feature = "moderation")]
pub use crate::moderation::*;
#[cfg(feature = "points")]
pub use crate::points::*;
#[cfg(feature = "stream")]
pub use crate::stream::*;
#[cfg(feature = "sub")]
pub use crate::sub::*;
#[cfg(feature = "timestamp")]
pub use crate::time::*;
#[cfg(feature = "user")]
pub use crate::user::*;

#[cfg(all(feature = "serde", feature = "stream"))]
fn deserialize_none_from_empty_string<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: serde::Deserialize<'de>, {
    use serde::de::IntoDeserializer;
    struct Inner<S>(std::marker::PhantomData<S>);
    impl<'de, S> serde::de::Visitor<'de> for Inner<S>
    where S: serde::Deserialize<'de>
    {
        type Value = Option<S>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("any string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match value {
                "" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match &*value {
                "" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: serde::de::Error {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Inner(std::marker::PhantomData))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrow)]
    use super::*;

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn lol() {
        assert!(broadcaster_id("literal"));
        assert!(!broadcaster_id(String::from("string")));
        assert!(broadcaster_id(&String::from("ref string")));
        assert!(broadcaster_id(UserIdRef::from_static("static ref")));
        assert!(!broadcaster_id(UserId::new(String::from("owned"))));
        assert!(broadcaster_id(&UserId::new(String::from("borrowed owned"))));
        assert!(broadcaster_id(&*UserId::new(String::from("deref owned"))));
        assert!(!broadcaster_id(std::borrow::Cow::Owned::<'_, UserIdRef>(
            UserId::new(String::from("cow owned"))
        )));
        assert!(broadcaster_id(std::borrow::Cow::Borrowed(
            UserIdRef::from_static("cow borrowed")
        )));

        assert!(broadcaster_id(opt(Some(std::borrow::Cow::Borrowed(
            "through fn borrow"
        )))));

        assert!(!broadcaster_id(opt(Some(std::borrow::Cow::Owned(
            "through fn owned".to_owned()
        )))));

        assert!(!broadcaster_id(opt_ref(Some(&std::borrow::Cow::Owned(
            "through fn ref owned".to_owned()
        )))));

        assert!(broadcaster_id(opt_ref(Some(&std::borrow::Cow::Borrowed(
            "through fn ref borrowed"
        )))));
    }

    fn opt(cow: Option<std::borrow::Cow<'_, str>>) -> std::borrow::Cow<'_, UserIdRef> {
        cow.map(|c| c.into_cow()).unwrap()
    }
    fn opt_ref<'a>(cow: Option<&std::borrow::Cow<'a, str>>) -> std::borrow::Cow<'a, UserIdRef> {
        cow.map(|c| c.clone().into_cow()).unwrap()
    }
    /// aa
    pub fn broadcaster_id<'a>(broadcaster_id: impl IntoCow<'a, UserIdRef> + 'a) -> bool {
        struct K<'a> {
            id: std::borrow::Cow<'a, UserIdRef>,
        }
        let k = K {
            id: broadcaster_id.into_cow(),
        };
        matches!(k.id, std::borrow::Cow::Borrowed(_))
    }

    #[test]
    fn debug_output_shown() {
        let uid = UserIdRef::from_static("my-user-id");
        let owned_uid = uid.to_owned();

        assert_eq!(format!("{uid:?}"), "\"my-user-id\"");
        assert_eq!(format!("{owned_uid:?}"), "\"my-user-id\"");
    }

    #[test]
    #[cfg(feature = "stream")]
    fn debug_output_hidden() {
        let key = StreamKey::from_static("my-stream-key");
        let owned_key = key.to_owned();

        assert_eq!(format!("{key:?}"), "[redacted stream key]");
        assert_eq!(format!("{owned_key:?}"), "[redacted stream key]");
    }
}
