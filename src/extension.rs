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
    pub struct BitsTransactionIdRef;
}
impl_extra!(BitsTransactionId, BitsTransactionIdRef);
