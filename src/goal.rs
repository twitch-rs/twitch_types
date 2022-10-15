use serde::{Deserialize, Serialize};

/// A Creator Goal ID
#[aliri_braid::braid(serde)]
pub struct CreatorGoalId;

/// Type of creator goal
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CreatorGoalType {
    /// The goal is to increase followers.
    Follower,
    /// The goal is to increase subscriptions. This type shows the net increase or decrease in tier points associated with the subscriptions.
    Subscription,
    /// The goal is to increase subscriptions. This type shows the net increase or decrease in the number of subscriptions.
    SubscriptionCount,
    /// The goal is to increase subscriptions. This type shows only the net increase in tier points associated with the subscriptions (it does not account for users that unsubscribed since the goal started).
    NewSubscription,
    /// The goal is to increase subscriptions. This type shows only the net increase in the number of subscriptions (it does not account for users that unsubscribed since the goal started).
    NewSubscriptionCount,
}
