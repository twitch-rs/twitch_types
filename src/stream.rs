#[cfg(feature = "serde")]
use serde::Serialize;

manual_braid! {
    /// A Stream ID
    pub struct StreamId;
    pub struct StreamIdRef;
}
impl_extra!(StreamId, StreamIdRef);

manual_braid! {
    /// A game or category ID
    pub struct CategoryId;
    pub struct CategoryIdRef;
}
impl_extra!(CategoryId, CategoryIdRef);

manual_braid! {
    /// A tag ID
    pub struct TagId;
    pub struct TagIdRef;
}
impl_extra!(TagId, TagIdRef);

manual_braid! {
    /// A Team ID
    pub struct TeamId;
    pub struct TeamIdRef;
}
impl_extra!(TeamId, TeamIdRef);

manual_braid! {
    /// A video ID
    pub struct VideoId;
    pub struct VideoIdRef;
}
impl_extra!(VideoId, VideoIdRef);

manual_braid! {
    /// A clip ID
    pub struct ClipId;
    pub struct ClipIdRef;
}
impl_extra!(ClipId, ClipIdRef);

manual_braid! {
    /// A Stream Segment ID.
    pub struct StreamSegmentId;
    pub struct StreamSegmentIdRef;
}
impl_extra!(StreamSegmentId, StreamSegmentIdRef);

manual_braid! {
    /// A Hype Train ID
    pub struct HypeTrainId;
    pub struct HypeTrainIdRef;
}
impl_extra!(HypeTrainId, HypeTrainIdRef);

manual_braid! {
    /// A Charity Campaign ID
    pub struct CharityCampaignId;
    pub struct CharityCampaignIdRef;
}
impl_extra!(CharityCampaignId, CharityCampaignIdRef);

manual_braid! {
    /// A Charity Donation ID
    pub struct CharityDonationId;
    pub struct CharityDonationIdRef;
}
impl_extra!(CharityDonationId, CharityDonationIdRef);

manual_braid! {
    /// A [IGDB](https://www.igdb.com/) ID
    pub struct IgdbId;
    pub struct IgdbIdRef;
}
impl_extra!(IgdbId, IgdbIdRef);

manual_braid! {
    /// A Guest Star Session ID
    pub struct GuestStarSessionId;
    pub struct GuestStarSessionIdRef;
}
impl_extra!(GuestStarSessionId, GuestStarSessionIdRef);

manual_braid! {
    /// A Guest Star Slot ID
    pub struct GuestStarSlotId;
    pub struct GuestStarSlotIdRef;
}
impl_extra!(GuestStarSlotId, GuestStarSlotIdRef);

manual_braid! {
    /// A stream marker ID
    pub struct StreamMarkerId;
    pub struct StreamMarkerIdRef;
}
impl_extra!(StreamMarkerId, StreamMarkerIdRef);

manual_braid! {
    redact("stream key");

    /// A Stream Key (hidden [Debug] output)
    pub struct StreamKey;
    pub struct StreamKeyRef;
}
impl_extra!(StreamKey, StreamKeyRef);

/// A game or category as defined by Twitch
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TwitchCategory {
    /// Template URL for the game’s box art.
    pub box_art_url: String,
    /// Game or category ID.
    pub id: CategoryId,
    /// Game name.
    pub name: String,
    /// The ID that IGDB uses to identify this game.
    ///
    /// An empty value may indicate the endpoint does not return an id or that the category/game is not available on IGDB
    #[cfg_attr(
        feature = "serde",
        serde(
            deserialize_with = "crate::deserialize_none_from_empty_string",
            default
        )
    )]
    pub igdb_id: Option<IgdbId>,
}

/// Subscription tiers
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(field_identifier))]
pub enum SubscriptionTier {
    /// Tier 1. $4.99
    #[cfg_attr(feature = "serde", serde(rename = "1000"))]
    Tier1,
    /// Tier 1. $9.99
    #[cfg_attr(feature = "serde", serde(rename = "2000"))]
    Tier2,
    /// Tier 1. $24.99
    #[cfg_attr(feature = "serde", serde(rename = "3000"))]
    Tier3,
    /// Prime subscription
    Prime,
    /// Other
    Other(String),
}

#[cfg(feature = "serde")]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Length of the commercial in seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[cfg(feature = "serde")]
impl serde::Serialize for CommercialLength {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let value: u64 = *self as u64;
        serde::Serialize::serialize(&value, serializer)
    }
}

/// TODO: macroify?
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CommercialLength {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        struct discriminant;

        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Length120: u64 = CommercialLength::Length120 as u64;
            const Length150: u64 = CommercialLength::Length150 as u64;
            const Length180: u64 = CommercialLength::Length180 as u64;
            const Length30: u64 = CommercialLength::Length30 as u64;
            const Length60: u64 = CommercialLength::Length60 as u64;
            const Length90: u64 = CommercialLength::Length90 as u64;
        }
        match <u64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Length30 => core::result::Result::Ok(CommercialLength::Length30),
            discriminant::Length60 => core::result::Result::Ok(CommercialLength::Length60),
            discriminant::Length90 => core::result::Result::Ok(CommercialLength::Length90),
            discriminant::Length120 => core::result::Result::Ok(CommercialLength::Length120),
            discriminant::Length150 => core::result::Result::Ok(CommercialLength::Length150),
            discriminant::Length180 => core::result::Result::Ok(CommercialLength::Length180),
            other => core::result::Result::Err(serde::de::Error::custom(format_args!(
                "invalid value: {}, expected one of: {}, {}, {}, {}, {}, {}",
                other,
                discriminant::Length30,
                discriminant::Length60,
                discriminant::Length90,
                discriminant::Length120,
                discriminant::Length150,
                discriminant::Length180
            ))),
        }
    }
}

impl std::fmt::Display for CommercialLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", *self as u64)
    }
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
#[derive(Debug)]
pub enum CommercialLengthParseError {
    /// invalid length of {0}
    InvalidLength(u64),
}

impl std::error::Error for CommercialLengthParseError {}

impl core::fmt::Display for CommercialLengthParseError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[allow(unused_variables)]
        match self {
            CommercialLengthParseError::InvalidLength(len) => {
                write!(formatter, "invalid length of {len}")
            }
        }
    }
}

/// IDs for [content classification labels](https://help.twitch.tv/s/article/content-classification-labels) also known as CCLs
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(field_identifier))]
pub enum ContentClassificationId {
    /// Politics and Sensitive Social Issues
    ///
    /// Discussions or debates about politics or sensitive social issues such as elections, civic integrity, military conflict, and civil rights in a polarizing manner.
    DebatedSocialIssuesAndPolitics,
    /// Drugs, Intoxication, or Excessive Tobacco Use
    ///
    /// Excessive tobacco glorification or promotion, any marijuana consumption/use, legal drug and alcohol induced intoxication, discussions of illegal drugs.
    DrugsIntoxication,
    /// Sexual Themes
    ///
    /// Content that focuses on sexualized physical attributes and activities, sexual topics, or experiences.
    SexualThemes,
    /// Violent and Graphic Depictions
    ///
    /// Simulations and/or depictions of realistic violence, gore, extreme injury, or death.
    ViolentGraphic,
    /// Gambling
    ///
    /// Participating in online or in-person gambling, poker or fantasy sports, that involve the exchange of real money.
    Gambling,
    /// Significant Profanity or Vulgarity
    ///
    /// Prolonged, and repeated use of obscenities, profanities, and vulgarities, especially as a regular part of speech.
    ProfanityVulgarity,
    /// Mature-rated game
    ///
    /// Games that are rated Mature or less suitable for a younger audience.
    MatureGame,
    /// Other
    Other(String),
}

#[cfg(feature = "serde")]
impl Serialize for ContentClassificationId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            ContentClassificationId::DebatedSocialIssuesAndPolitics => {
                "DebatedSocialIssuesAndPolitics"
            }
            ContentClassificationId::DrugsIntoxication => "DrugsIntoxication",
            ContentClassificationId::SexualThemes => "SexualThemes",
            ContentClassificationId::ViolentGraphic => "ViolentGraphic",
            ContentClassificationId::Gambling => "Gambling",
            ContentClassificationId::ProfanityVulgarity => "ProfanityVulgarity",
            ContentClassificationId::MatureGame => "MatureGame",
            ContentClassificationId::Other(o) => o,
        })
    }
}
