#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
//! Twitch types

mod basic;
// cc: https://github.com/rust-lang/rust/issues/83428, can't use glob imports and keep the modules private
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
