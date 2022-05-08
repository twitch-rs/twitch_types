use serde::{Deserialize, Serialize};

/// A Creator Goal ID
#[aliri_braid::braid(serde)]
pub struct CreatorGoalId;

/// Type of creator goal
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum CreatorGoalType {
    /// Creator goal is for followers
    Follower,
    /// Creator goal is for subscriptions
    Subscription,
}
