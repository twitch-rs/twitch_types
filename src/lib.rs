#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![allow(clippy::extra_unused_lifetimes)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
//! Twitch types

macro_rules! impl_extra {
    (validated, $owned:path, $ref:path, $error:path) => {
        impl<'a> TryFrom<&'a String> for &'a $ref {
            type Error = $error;
            fn try_from(string: &'a String) -> Result<Self, $error>  {
                <$ref>::from_str(string.as_str())
            }
        }

        impl_extra!(@all, $owned, $ref);
    };
    ($owned:path, $ref:path) => {
        impl<'a> From<&'a String> for &'a $ref {
            fn from(string: &'a String) -> Self {
                <$ref>::from_str(string.as_str())
            }
        }

        impl_extra!(@all, $owned, $ref);
    };
    (@all, $owned:path, $ref:path) => {
        impl $ref {
            /// Get a
            #[doc = concat!("[`Cow<'_, ", stringify!($ref), ">`](std::borrow::Cow::Borrowed)")]
            pub fn as_cow<'a>(&'a self) -> ::std::borrow::Cow<'a, $ref> {
                self.into()
            }
        }

        impl<'a> From<&'a $owned> for &'a $ref {
            fn from(owned: &'a $owned) -> Self {
                &*owned
            }
        }

        impl<'a> From<&'a $owned> for ::std::borrow::Cow<'a, $ref> {
            fn from(owned: &'a $owned) -> Self {
                ::std::borrow::Cow::Borrowed(&*owned)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for &'a $ref {
            fn to_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Borrowed(self)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for $owned {
            fn to_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Owned(self)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for &'a $owned {
            fn to_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Borrowed(self.as_ref())
            }
        }
    };
}

/// Convert a type into a [`Cow`](std::borrow::Cow)
pub trait IntoCow<'a, Ref: ?Sized>
where Ref: ToOwned {
    /// Make the cow with proper ownership, muu
    fn to_cow(self) -> std::borrow::Cow<'a, Ref>
    where &'a Self: 'a;
}

impl<'a, R> IntoCow<'a, R> for std::borrow::Cow<'a, R>
where
    &'a R: Into<&'a R>,
    R: ToOwned + ?Sized + 'a,
    &'a R: Into<std::borrow::Cow<'a, R>>,
    R::Owned: Into<std::borrow::Cow<'a, R>>,
{
    fn to_cow(self) -> std::borrow::Cow<'a, R> {
        match self {
            std::borrow::Cow::Borrowed(b) => b.into(),
            std::borrow::Cow::Owned(o) => o.into(),
        }
    }
}

impl<'a, R> IntoCow<'a, R> for &'a str
where
    &'a str: Into<&'a R>,
    R: ToOwned + ?Sized + 'a,
{
    fn to_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Borrowed(self.into()) }
}

impl<'a, R> IntoCow<'a, R> for &'a String
where
    &'a String: Into<&'a R>,
    R: ToOwned + ?Sized + 'a,
{
    fn to_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Borrowed(self.into()) }
}

impl<'a, R> IntoCow<'a, R> for String
where
    String: Into<R::Owned>,
    R: ToOwned + ?Sized + 'a,
{
    fn to_cow(self) -> std::borrow::Cow<'a, R> { std::borrow::Cow::Owned(self.into()) }
}

mod basic;
// cc: https://github.com/rust-lang/rust/issues/83428, can't use glob imports and keep the modules private
#[cfg(feature = "color")]
/// types for colors
pub mod color;
#[cfg(feature = "emote")]
/// types for emotes
pub mod emote;
#[cfg(feature = "eventsub")]
/// types for eventsub related things
pub mod eventsub;
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
#[cfg(feature = "timestamp")]
/// types for time
pub mod time;
#[cfg(feature = "user")]
/// types for user related things
pub mod user;

pub use basic::*;

#[cfg(feature = "color")]
pub use crate::color::*;
#[cfg(feature = "emote")]
pub use crate::emote::*;
#[cfg(feature = "eventsub")]
pub use crate::eventsub::*;
#[cfg(feature = "goal")]
pub use crate::goal::*;
#[cfg(feature = "moderation")]
pub use crate::moderation::*;
#[cfg(feature = "points")]
pub use crate::points::*;
#[cfg(feature = "stream")]
pub use crate::stream::*;
#[cfg(feature = "timestamp")]
pub use crate::time::*;
#[cfg(feature = "user")]
pub use crate::user::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lol() {
        assert!(broadcaster_id("literal"));
        assert!(!broadcaster_id(String::from("string")));
        assert!(broadcaster_id(&String::from("ref string")));
        assert!(broadcaster_id(UserIdRef::from_static("static ref")));
        assert!(!broadcaster_id(UserId::new(String::from("owned"))));
        assert!(broadcaster_id(&UserId::new(String::from("borrowed owned"))));
        assert!(broadcaster_id(&*UserId::new(String::from("deref owned"))));
        assert!(!broadcaster_id(std::borrow::Cow::Owned(UserId::new(
            String::from("cow owned")
        ))));
        assert!(broadcaster_id(std::borrow::Cow::Borrowed(
            UserIdRef::from_static("cow borrowed")
        )));
    }
    /// aa
    pub fn broadcaster_id<'a>(broadcaster_id: impl IntoCow<'a, UserIdRef> + 'a) -> bool {
        struct K<'a> {
            id: std::borrow::Cow<'a, UserIdRef>,
        }
        let k = K {
            id: broadcaster_id.to_cow(),
        };
        matches!(k.id, std::borrow::Cow::Borrowed(_))
    }
}
