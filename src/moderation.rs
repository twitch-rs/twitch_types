manual_braid! {
    /// A blocked term ID
    pub struct BlockedTermId;
    pub struct BlockedTermIdRef;
}
impl_extra!(BlockedTermId, BlockedTermIdRef);

manual_braid! {
    /// An unban request ID
    pub struct UnbanRequestId;
    pub struct UnbanRequestIdRef;
}
impl_extra!(UnbanRequestId, UnbanRequestIdRef);

/// Status of a message that is or was in AutoMod queue
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[non_exhaustive]
pub enum AutomodStatus {
    /// Message has been caught and pending moderation
    Pending,
    /// Message has been allowed
    Allowed,
    /// Message has been denied
    Denied,
    /// Automod message expired in queue
    Expired,
}

pub use crate::basic::{MsgId, MsgIdRef};
