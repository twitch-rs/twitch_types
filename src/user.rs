use serde::{Deserialize, Serialize};

use crate::{DisplayName, UserId, UserName};

/// Broadcaster types: "partner", "affiliate", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum BroadcasterType {
    /// Partner
    #[serde(rename = "partner")]
    Partner,
    /// Affiliate
    #[serde(rename = "affiliate")]
    Affiliate,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for BroadcasterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            BroadcasterType::Partner => "partner",
            BroadcasterType::Affiliate => "affiliate",
            BroadcasterType::None => "",
        })
    }
}

/// User types: "staff", "admin", "global_mod", or "".
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum UserType {
    /// Staff
    #[serde(rename = "staff")]
    Staff,
    /// Admin
    #[serde(rename = "admin")]
    Admin,
    /// Global Moderator
    #[serde(rename = "global_mod")]
    GlobalMod,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            UserType::Staff => "staff",
            UserType::Admin => "admin",
            UserType::GlobalMod => "global_mod",
            UserType::None => "",
        })
    }
}

/// A user according to many endpoints
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct User {
    /// ID of the user
    #[serde(alias = "user_id")]
    pub id: UserId,
    /// Login name of the user, not capitalized
    #[serde(alias = "user_login")]
    pub login: UserName,
    /// Display name of user
    #[serde(alias = "user_display_name", alias = "user_name")]
    pub display_name: DisplayName,
    #[serde(default)]
    /// URL of the user's profile
    pub profile_image_url: Option<String>,
}
