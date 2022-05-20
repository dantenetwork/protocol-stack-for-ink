use ink_storage::{
    traits::{
        SpreadLayout,
        SpreadAllocate,
        StorageLayout,
        PackedLayout,
    },
};

use ink_env::{
    AccountId,
};

use ink_prelude::{
    vec::Vec,
    string::String,
};

use scale::{
    Encode,
    Decode,
};
    
pub type Bytes = Vec<u8>;
pub type Porters = Vec<String>;

/// Errors for cross-chain contract
#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    IdNotMatch,
    ChainMessageNotFound,
    IdOutOfBound,
    AlreadyExecuted,
}

/// Content structure
#[derive(SpreadAllocate, SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Content {
    contract: String,
    action: String,
    data: Bytes,
}

impl Content {
    pub fn new(contract: &String, action: &String, data: &Bytes) -> Self {
        Self {
            contract: contract.clone(),
            action: action.clone(),
            data: data.clone(),
        }
    }
}

/// SQOS structure
#[derive(SpreadAllocate, SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct SQOS {
    pub reveal: u8,
}

/// Session Structure
#[derive(SpreadAllocate, SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct Session {
    pub msg_type: u8,
    pub id: u128,
}

/// Received message structure
#[derive(SpreadAllocate, SpreadLayout, PackedLayout, Default, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct ReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: SQOS,
    pub contract: AccountId,
    pub action: String,
    pub data: Bytes,
    pub session: Session,
    pub executed: bool,
    pub error_code: u16,
}

impl ReceivedMessage {
    pub fn new(id: u128, from_chain: &String, sender: &String, signer: &String, sqos: &SQOS,
        contract: AccountId, action: &String, data: &Bytes, session: &Session) -> Self {
        Self {
            id,
            from_chain: from_chain.clone(),
            sender: sender.clone(),
            signer: signer.clone(),
            sqos: sqos.clone(),
            contract,
            action: action.clone(),
            data: data.clone(),
            session: session.clone(),
            executed: false,
            error_code: 0,
        }
    }

    pub fn new_with_error(id: u128, from_chain: &String, error_code: u16) -> Self {
        let mut m = Self::default();
        m.id = id;
        m.from_chain = from_chain.clone();
        m.error_code = error_code;
        m
    }
}

/// Sent message structure
#[derive(SpreadAllocate, SpreadLayout, PackedLayout, Clone, Decode, Encode)]
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
    pub fn new(id: u128, from_chain: &String, to_chain: &String, sender: AccountId, signer: AccountId,
        sqos: &SQOS, content: &Content, session: &Session) -> Self {
        Self {
            id,
            from_chain: from_chain.clone(),
            to_chain: to_chain.clone(),
            sender: sender.clone(),
            signer: signer.clone(),
            sqos: sqos.clone(),
            content: content.clone(),
            session: session.clone(),
        }
    }
}

/// Context structure
#[derive(SpreadAllocate, SpreadLayout, Clone, Decode, Encode)]
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
    pub fn new(id: u128, from_chain: &String, sender: &String, signer: &String, contract: AccountId, action: &String) -> Self {
        Self {
            id,
            from_chain: from_chain.clone(),
            sender: sender.clone(),
            signer: signer.clone(),
            contract,
            action: action.clone(),
        }
    }
}