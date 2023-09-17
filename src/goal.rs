manual_braid! {
    /// A Creator Goal ID
    pub struct CreatorGoalId;
    pub struct CreatorGoalIdRef;
}
impl_extra!(CreatorGoalId, CreatorGoalIdRef);

/// Type of creator goal
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
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
