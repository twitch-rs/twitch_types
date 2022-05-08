use serde::{Deserialize, Serialize};

/// A Stream ID
#[aliri_braid::braid(serde)]
pub struct StreamId;

/// A game or category ID
#[aliri_braid::braid(serde)]
pub struct CategoryId;

/// A tag ID
#[aliri_braid::braid(serde)]
pub struct TagId;

/// A Team ID
#[aliri_braid::braid(serde)]
pub struct TeamId;

/// A video ID
#[aliri_braid::braid(serde)]
pub struct VideoId;

/// A Stream Segment ID.
#[aliri_braid::braid(serde)]
pub struct StreamSegmentId;

/// A Hype Train ID
#[aliri_braid::braid(serde)]
pub struct HypeTrainId;

/// A game or category as defined by Twitch
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TwitchCategory {
    /// Template URL for the game’s box art.
    pub box_art_url: String,
    /// Game or category ID.
    pub id: CategoryId,
    /// Game name.
    pub name: String,
}

/// Subscription tiers
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(field_identifier)]
pub enum SubscriptionTier {
    /// Tier 1. $4.99
    #[serde(rename = "1000")]
    Tier1,
    /// Tier 1. $9.99
    #[serde(rename = "2000")]
    Tier2,
    /// Tier 1. $24.99
    #[serde(rename = "3000")]
    Tier3,
    /// Prime subscription
    Prime,
    /// Other
    Other(String),
}

impl Serialize for SubscriptionTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            SubscriptionTier::Tier1 => "1000",
            SubscriptionTier::Tier2 => "2000",
            SubscriptionTier::Tier3 => "3000",
            SubscriptionTier::Prime => "Prime",
            SubscriptionTier::Other(o) => o,
        })
    }
}

/// Period during which the video was created
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoType {
    /// A live video
    Live,
    // FIXME: What is this?
    /// A playlist video
    Playlist,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
    /// A premiere
    Premiere,
    /// A rerun
    Rerun,
    /// A watch party
    WatchParty,
    /// A watchparty premiere,
    WatchPartyPremiere,
    /// A watchparty rerun
    WatchPartyRerun,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Length of the commercial in seconds
#[derive(
    displaydoc::Display,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[repr(u64)]
#[non_exhaustive]
pub enum CommercialLength {
    /// 30s
    Length30 = 30,
    /// 60s
    Length60 = 60,
    /// 90s
    Length90 = 90,
    /// 120s
    Length120 = 120,
    /// 150s
    Length150 = 150,
    /// 180s
    Length180 = 180,
}

impl std::convert::TryFrom<u64> for CommercialLength {
    type Error = CommercialLengthParseError;

    fn try_from(l: u64) -> Result<Self, Self::Error> {
        match l {
            30 => Ok(CommercialLength::Length30),
            60 => Ok(CommercialLength::Length60),
            90 => Ok(CommercialLength::Length90),
            120 => Ok(CommercialLength::Length120),
            150 => Ok(CommercialLength::Length150),
            180 => Ok(CommercialLength::Length180),
            other => Err(CommercialLengthParseError::InvalidLength(other)),
        }
    }
}

/// Error for the `TryFrom` on [`CommercialLength`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CommercialLengthParseError {
    /// invalid length of {0}
    InvalidLength(u64),
}
