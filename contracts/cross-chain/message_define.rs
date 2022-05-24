use ink_storage::{
    traits::{
        SpreadLayout,
        StorageLayout,
        PackedLayout,
    },
};

use ink_env::AccountId;

use ink_prelude::{
    vec::Vec,
    string::String,
};

use scale::{
    Encode,
    Decode,
};

use crate::payload::MessagePayload;
    
pub type Bytes = Vec<u8>;
pub type Porters = Vec<AccountId>;

/// Errors for cross-chain contract
#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    IdNotMatch,
    ChainMessageNotFound,
    IdOutOfBound,
    AlreadyExecuted,
    InterfaceNotFound,
}

/// Content structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Content {
    contract: String,
    action: String,
    data: MessagePayload,
}

impl Content {
    pub fn new(contract: String, action: String, data: MessagePayload) -> Self {
        Self {
            contract: contract,
            action: action,
            data: data,
        }
    }
}

/// SQOS structure
#[derive(SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct SQOS {
    pub reveal: u8,
}

impl SQOS {
    pub fn new(reveal: u8) -> Self {
        Self {
            reveal,
        }
    }
}

/// Session Structure
#[derive(SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Session {
    pub msg_type: u8,
    pub id: u128,
}

impl Session {
    pub fn new(msg_type: u8, id: u128) -> Self {
        Self {
            msg_type,
            id,
        }
    }
}

/// Received message structure
#[derive(SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct ReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: SQOS,
    pub contract: AccountId,
    pub action: String,
    pub data: MessagePayload,
    pub session: Session,
    pub executed: bool,
    pub error_code: u16,
}

impl ReceivedMessage {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: SQOS,
        contract: AccountId, action: String, data: MessagePayload, session: Session) -> Self {
        Self {
            id,
            from_chain,
            sender,
            signer,
            sqos,
            contract,
            action,
            data,
            session,
            executed: false,
            error_code: 0,
        }
    }

    pub fn new_with_error(id: u128, from_chain: String, error_code: u16) -> Self {
        let mut m = Self::default();
        m.id = id;
        m.from_chain = from_chain;
        m.error_code = error_code;
        m
    }
}

/// Sent message structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct SentMessage {
    id: u128,
    from_chain: String,
    to_chain: String,
    sender: AccountId,
    signer: AccountId,
    sqos: SQOS,
    content: Content,
    session: Session,
}

impl SentMessage {
    pub fn new(id: u128, from_chain: String, to_chain: String, sender: AccountId, signer: AccountId,
        sqos: SQOS, content: Content, session: Session) -> Self {
        Self {
            id,
            from_chain,
            to_chain,
            sender,
            signer,
            sqos,
            content,
            session,
        }
    }
}

/// Context structure
#[derive(SpreadLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Context {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub contract: AccountId,
    pub action: String,
}

impl Context {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, contract: AccountId, action: String) -> Self {
        Self {
            id,
            from_chain,
            sender,
            signer,
            contract,
            action,
        }
    }
}