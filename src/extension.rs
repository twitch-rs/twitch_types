manual_braid! {
    /// An Extension ID
    pub struct ExtensionId;
    pub struct ExtensionIdRef;
}
impl_extra!(ExtensionId, ExtensionIdRef);

manual_braid!{
    /// An Extension Client ID
    pub struct ExtensionClientId;
    pub struct ExtensionClientIdRef;
}
impl_extra!(ExtensionClientId, ExtensionClientIdRef);

manual_braid!{
    /// A Bits Transaction ID
    pub struct BitsTransactionId;
    pub struct BitsTransactionIdref;
}
impl_extra!(BitsTransactionId, BitsTransactionIdref);

/// An extension product
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ExtensionProduct {
    /// The Display name of the purchased product
    pub name: String,

    /// The sku of the purchased product
    pub sku: String,

    /// The amount of bits paid for the product
    pub bits: i64,

    /// If the product is in development (bits will always be 0 if this is true)
    pub in_development: bool,
}