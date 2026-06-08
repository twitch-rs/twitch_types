use crate::{DisplayName, UserId, UserName};

pub use crate::{BroadcasterType, UserType};

/// A broadcaster
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Broadcaster {
    /// ID of the broadcaster
    #[cfg_attr(feature = "serde", serde(alias = "broadcaster_user_id"))]
    pub broadcaster_user_id: UserId,
    /// Login name of the broadcaster, not capitalized
    #[cfg_attr(feature = "serde", serde(alias = "broadcaster_user_name"))]
    pub broadcaster_user_name: UserName,
    /// Display name of the broadcaster
    #[cfg_attr(feature = "serde", serde(alias = "broadcaster_display_name"))]
    pub broadcaster_display_name: DisplayName,
}
