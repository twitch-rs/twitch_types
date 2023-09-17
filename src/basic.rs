manual_braid! {
    /// A user ID.
    pub struct UserId;
    pub struct UserIdRef;
}
impl_extra!(numeric, UserId, UserIdRef);

manual_braid! {
    /// A users display name
    pub struct DisplayName;
    pub struct DisplayNameRef;
}
impl_extra!(DisplayName, DisplayNameRef);

manual_braid! {
    /// A nickname, not capitalized.
    pub struct Nickname;
    pub struct NicknameRef;
}
impl_extra!(ascii, Nickname, NicknameRef);

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A reference to a borrowed [`UserName`], also specified as login. Should not be capitalized.
pub type UserNameRef = NicknameRef;

manual_braid! {
    /// A message ID
    pub struct MsgId;
    pub struct MsgIdRef;
}
impl_extra!(MsgId, MsgIdRef);

/// Broadcaster types: "partner", "affiliate", or "".
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]
pub enum BroadcasterType {
    /// Partner
    #[cfg_attr(feature = "serde", serde(rename = "partner"))]
    Partner,
    /// Affiliate
    #[cfg_attr(feature = "serde", serde(rename = "affiliate"))]
    Affiliate,
    /// None
    #[cfg_attr(feature = "serde", serde(other))]
    #[default]
    None,
}

#[cfg(feature = "serde")]
impl serde::Serialize for BroadcasterType {
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
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]

pub enum UserType {
    /// Staff
    #[cfg_attr(feature = "serde", serde(rename = "staff"))]
    Staff,
    /// Admin
    #[cfg_attr(feature = "serde", serde(rename = "admin"))]
    Admin,
    /// Global Moderator
    #[cfg_attr(feature = "serde", serde(rename = "global_mod"))]
    GlobalMod,
    /// None
    #[cfg_attr(feature = "serde", serde(other))]
    #[default]
    None,
}

impl UserType {
    /// Parse a string into a [`UserType`]
    pub fn parse(input: &str) -> Self {
        match input {
            "admin" => Self::Admin,
            "global_mod" => Self::GlobalMod,
            "staff" => Self::Staff,
            _ => Self::None,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for UserType {
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
