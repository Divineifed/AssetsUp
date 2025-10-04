use soroban_sdk::{Address, BytesN, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    AuditLog(BytesN<32>),
    AuditLogsByAsset(BytesN<32>),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditLog {
    pub id: BytesN<32>,
    pub asset_id: BytesN<32>,
    pub action: String,
    pub timestamp: u64,
    pub actor: Address,
    pub details: String,
}
