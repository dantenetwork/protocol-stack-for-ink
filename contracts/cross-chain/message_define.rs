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
    DecodeDataFailed,
    CrossContractCallFailed(ink_env::Error),
}

/// Content structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Content {
    contract: String,
    action: String,
    data: Bytes,
}

impl Content {
    pub fn new(contract: String, action: String, data: Bytes) -> Self {
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
    pub action: Bytes,
    pub data: Bytes,
    pub session: Session,
    pub executed: bool,
    pub error_code: u16,
}

impl ReceivedMessage {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: SQOS,
        contract: AccountId, action: Bytes, data: Bytes, session: Session) -> Self {
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
    pub id: u128,
    pub from_chain: String,
    pub to_chain: String,
    pub sender: AccountId,
    pub signer: AccountId,
    pub sqos: SQOS,
    pub content: Content,
    pub session: Session,
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

    pub fn new_sending_message(to_chain: String, sqos: SQOS, session: Session, content: Content) -> Self {
        Self {
            id: 0,
            from_chain: String::try_from("").unwrap(),
            to_chain,
            sender: AccountId::default(),
            signer: AccountId::default(),
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
    pub action: Bytes,
}

impl Context {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, contract: AccountId, action: Bytes) -> Self {
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