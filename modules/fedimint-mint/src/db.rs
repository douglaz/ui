use std::time::SystemTime;

use fedimint_api::db::DatabaseKeyPrefixConst;
use fedimint_api::encoding::{Decodable, Encodable};
use fedimint_api::impl_db_prefix_const;
use fedimint_api::{Amount, OutPoint, PeerId};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::{MintOutputBlindSignatures, MintOutputSignatureShare, Nonce};

#[repr(u8)]
#[derive(Clone, EnumIter, Debug)]
pub enum DbKeyPrefix {
    NoteNonce = 0x10,
    ProposedPartialSig = 0x11,
    ReceivedPartialSig = 0x12,
    OutputOutcome = 0x13,
    MintAuditItem = 0x14,
    EcashBackup = 0x15,
}

impl std::fmt::Display for DbKeyPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize)]
pub struct NonceKey(pub Nonce);

#[derive(Debug, Encodable, Decodable)]
pub struct NonceKeyPrefix;

impl_db_prefix_const!(NonceKey, NonceKeyPrefix, (), DbKeyPrefix::NoteNonce);

#[derive(Debug, Encodable, Decodable, Serialize)]
pub struct ProposedPartialSignatureKey {
    pub out_point: OutPoint, // tx + output idx
}

#[derive(Debug, Encodable, Decodable)]
pub struct ProposedPartialSignaturesKeyPrefix;

impl_db_prefix_const!(
    ProposedPartialSignatureKey,
    ProposedPartialSignaturesKeyPrefix,
    MintOutputSignatureShare,
    DbKeyPrefix::ProposedPartialSig
);

#[derive(Debug, Encodable, Decodable, Serialize)]
pub struct ReceivedPartialSignatureKey {
    pub request_id: OutPoint, // tx + output idx
    pub peer_id: PeerId,
}

#[derive(Debug, Encodable, Decodable)]
pub struct ReceivedPartialSignaturesKeyPrefix;

impl_db_prefix_const!(
    ReceivedPartialSignatureKey,
    ReceivedPartialSignaturesKeyPrefix,
    MintOutputSignatureShare,
    DbKeyPrefix::ReceivedPartialSig
);

#[derive(Debug, Encodable, Decodable)]
pub struct ReceivedPartialSignatureKeyOutputPrefix {
    pub request_id: OutPoint, // tx + output idx
}

impl DatabaseKeyPrefixConst for ReceivedPartialSignatureKeyOutputPrefix {
    const DB_PREFIX: u8 = DbKeyPrefix::ReceivedPartialSig as u8;
    type Key = ReceivedPartialSignatureKey;
    type Value = MintOutputSignatureShare;
}

/// Transaction id and output index identifying an output outcome
#[derive(Debug, Clone, Copy, Encodable, Decodable, Serialize)]
pub struct OutputOutcomeKey(pub OutPoint);

#[derive(Debug, Encodable, Decodable)]
pub struct OutputOutcomeKeyPrefix;

impl_db_prefix_const!(
    OutputOutcomeKey,
    OutputOutcomeKeyPrefix,
    MintOutputBlindSignatures,
    DbKeyPrefix::OutputOutcome
);

/// Represents the amounts of issued (signed) and redeemed (verified) notes for auditing
#[derive(Debug, Clone, Encodable, Decodable, Serialize)]
pub enum MintAuditItemKey {
    Issuance(OutPoint),
    IssuanceTotal,
    Redemption(NonceKey),
    RedemptionTotal,
}

#[derive(Debug, Encodable, Decodable)]
pub struct MintAuditItemKeyPrefix;

impl_db_prefix_const!(
    MintAuditItemKey,
    MintAuditItemKeyPrefix,
    Amount,
    DbKeyPrefix::MintAuditItem
);

/// Key used to store user's ecash backups
#[derive(Debug, Clone, Copy, Encodable, Decodable, Serialize)]
pub struct EcashBackupKey(pub secp256k1_zkp::XOnlyPublicKey);

#[derive(Debug, Encodable, Decodable)]
pub struct EcashBackupKeyPrefix;

impl_db_prefix_const!(
    EcashBackupKey,
    EcashBackupKeyPrefix,
    ECashUserBackupSnapshot,
    DbKeyPrefix::EcashBackup
);

/// User's backup, received at certain time, containing encrypted payload
#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable, Serialize, Deserialize)]
pub struct ECashUserBackupSnapshot {
    pub timestamp: SystemTime,
    #[serde(with = "fedimint_api::hex::serde")]
    pub data: Vec<u8>,
}
