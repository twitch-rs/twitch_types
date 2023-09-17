use crate::{DisplayName, UserId, UserName};

pub use crate::{BroadcasterType, UserType};

/// A user according to many endpoints
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct User {
    /// ID of the user
    #[cfg_attr(feature = "serde", serde(alias = "user_id"))]
    pub id: UserId,
    /// Login name of the user, not capitalized
    #[cfg_attr(feature = "serde", serde(alias = "user_login"))]
    pub login: UserName,
    /// Display name of user
    #[cfg_attr(
        feature = "serde",
        serde(alias = "user_display_name", alias = "user_name")
    )]
    pub display_name: DisplayName,
    #[cfg_attr(feature = "serde", serde(default))]
    /// URL of the user's profile
    pub profile_image_url: Option<String>,
}
