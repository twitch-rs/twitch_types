manual_braid! {
    /// An EventSub Subscription ID
    pub struct EventSubId;
    pub struct EventSubIdRef;
}
impl_extra!(EventSubId, EventSubIdRef);

manual_braid! {
    /// An ID of a Conduit
    pub struct ConduitId;
    pub struct ConduitIdRef;
}
impl_extra!(ConduitId, ConduitIdRef);

manual_braid! {
    /// An ID of a Conduit Shard
    pub struct ConduitShardId;
    pub struct ConduitShardIdRef;
}
impl_extra!(ConduitShardId, ConduitShardIdRef);
