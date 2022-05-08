use serde::{Deserialize, Serialize};

/// A blocked term ID
#[aliri_braid::braid(serde)]
pub struct BlockedTermId;

/// Status of a message that is or was in AutoMod queue
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
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

/// A message ID
#[aliri_braid::braid(serde)]
pub struct MsgId;