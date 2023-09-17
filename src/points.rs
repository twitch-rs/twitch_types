use crate::{DisplayName, UserId, UserName};
manual_braid! {
    /// A reward ID.
    pub struct RewardId;
    pub struct RewardIdRef;
}

impl_extra!(RewardId, RewardIdRef);
manual_braid! {
    /// A reward redemption ID.
    pub struct RedemptionId;
    pub struct RedemptionIdRef;
}
impl_extra!(RedemptionId, RedemptionIdRef);

manual_braid! {
    /// A poll ID
    pub struct PollId;
    pub struct PollIdRef;
}
impl_extra!(PollId, PollIdRef);

manual_braid! {
    /// A poll choice ID
    pub struct PollChoiceId;
    pub struct PollChoiceIdRef;
}
impl_extra!(PollChoiceId, PollChoiceIdRef);

manual_braid! {
    /// A prediction ID
    pub struct PredictionId;
    pub struct PredictionIdRef;
}
impl_extra!(PredictionId, PredictionIdRef);

manual_braid! {
    /// A prediction choice ID
    pub struct PredictionOutcomeId;
    pub struct PredictionOutcomeIdRef;
}
impl_extra!(PredictionOutcomeId, PredictionOutcomeIdRef);

/// Reward redemption max
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[non_exhaustive]
pub enum Max {
    /// Max per stream
    MaxPerStream {
        /// Max per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per stream
        #[cfg_attr(feature = "serde", serde(alias = "value"))]
        max_per_stream: u32,
    },
    /// Max per user per stream
    MaxPerUserPerStream {
        /// Max per user per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per user per stream
        #[cfg_attr(feature = "serde", serde(alias = "value"))]
        max_per_user_per_stream: u32,
    },
}

/// Information about global cooldown
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalCooldown {
    /// Cooldown enabled
    pub is_enabled: bool,
    /// Cooldown amount
    #[cfg_attr(feature = "serde", serde(alias = "seconds"))]
    pub global_cooldown_seconds: u32,
}

/// Poll choice
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PollChoice {
    /// ID for the choice.
    pub id: String,
    /// Text displayed for the choice.
    pub title: String,
    /// Total number of votes received for the choice across all methods of voting.
    pub votes: Option<i64>,
    /// Number of votes received via Channel Points.
    pub channel_points_votes: Option<i64>,
    /// Number of votes received via Bits.
    pub bits_votes: Option<i64>,
}

// FIXME: Poll status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402176
/// Status of a poll
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[non_exhaustive]
pub enum PollStatus {
    /// Poll is currently in progress.
    #[cfg_attr(feature = "serde", serde(alias = "active"))]
    Active,
    /// Poll has reached its ended_at time.
    #[cfg_attr(feature = "serde", serde(alias = "completed"))]
    Completed,
    /// Poll has been manually terminated before its ended_at time.
    #[cfg_attr(feature = "serde", serde(alias = "terminated"))]
    Terminated,
    /// Poll is no longer visible on the channel.
    #[cfg_attr(feature = "serde", serde(alias = "archived"))]
    Archived,
    /// Poll is no longer visible to any user on Twitch.
    #[cfg_attr(feature = "serde", serde(alias = "moderated"))]
    Moderated,
    /// Something went wrong determining the state.
    #[cfg_attr(feature = "serde", serde(alias = "invalid"))]
    Invalid,
}

// FIXME: Prediction status has different name depending on if returned from helix or eventsub. See https://twitch.uservoice.com/forums/310213-developers/suggestions/43402197
/// Status of the Prediction
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[non_exhaustive]
pub enum PredictionStatus {
    /// A winning outcome has been chosen and the Channel Points have been distributed to the users who guessed the correct outcome.
    #[cfg_attr(feature = "serde", serde(alias = "resolved"))]
    Resolved,
    /// The Prediction is active and viewers can make predictions.
    #[cfg_attr(feature = "serde", serde(alias = "active"))]
    Active,
    /// The Prediction has been canceled and the Channel Points have been refunded to participants.
    #[cfg_attr(feature = "serde", serde(alias = "canceled"))]
    Canceled,
    /// The Prediction has been locked and viewers can no longer make predictions.
    #[cfg_attr(feature = "serde", serde(alias = "locked"))]
    Locked,
}

/// Outcome for the Prediction
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionOutcome {
    /// ID for the outcome.
    pub id: String,
    /// Text displayed for outcome.
    pub title: String,
    /// Number of unique users that chose the outcome.
    pub users: Option<i64>,
    /// Number of Channel Points used for the outcome.
    pub channel_points: Option<i64>,
    /// Array of users who were the top predictors. null if none. Top 10
    pub top_predictors: Option<Vec<PredictionTopPredictors>>,
    /// Color for the outcome. Valid values: BLUE, PINK
    pub color: String,
}

// FIXME: eventsub adds prefix `user_*`. See https://discord.com/channels/325552783787032576/326772207844065290/842359030252437514
/// Users who were the top predictors.
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PredictionTopPredictors {
    /// ID of the user.
    #[cfg_attr(feature = "serde", serde(alias = "user_id"))]
    pub id: UserId,
    /// Display name of the user.
    #[cfg_attr(feature = "serde", serde(alias = "user_name"))]
    pub name: DisplayName,
    /// Login of the user.
    #[cfg_attr(feature = "serde", serde(alias = "user_login"))]
    pub login: UserName,
    /// Number of Channel Points used by the user.
    pub channel_points_used: i64,
    /// Number of Channel Points won by the user.
    ///
    /// This value is always null in the event payload for Prediction progress and Prediction lock. This value is 0 if the outcome did not win or if the Prediction was canceled and Channel Points were refunded.
    pub channel_points_won: Option<i64>,
}
