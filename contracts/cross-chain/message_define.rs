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

use Payload::message_define::{
    ISentMessage,
    IReceivedMessage,
    IContent,
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
    CrossContractCallFailed,
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

    pub fn from(content: IContent) -> Self {
        Self {
            contract: content.contract,
            action: content.action,
            data: content.data,
        }
    }
}

/// SQOS structure
#[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum SQoSType{
    Reveal,
    Challenge,
    Threshold,
    Priority,
    ExceptionRollback,
    Anonymous,
    Identity,
    Isolation,
    CrossVerify,
}

/// SQOS structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct SQoS {
    pub t: SQoSType,
    pub v: Option<String>,
}

impl SQoS {
    pub fn new(t: SQoSType, v: Option<String>) -> Self {
        Self {
            t,
            v,
        }
    }

    pub fn from(sqos: ISQoS) -> Self {
        Self {
            t: sqos,
            v: sqos.v,
        }
    }
}

/// Session Structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
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
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct ReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub contract: AccountId,
    pub action: Bytes,
    pub data: Bytes,
    pub session: Session,
    pub executed: bool,
    pub error_code: u16,
}

impl ReceivedMessage {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: Vec<SQoS>,
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
        let m = Self {
            id,
            from_chain,
            sender: String::try_from("").unwrap(),
            signer: String::try_from("").unwrap(),
            sqos: Vec::<SQoS>::new(),
            contract: AccountId::default(),
            action: Bytes::new(),
            data: Bytes::new(),
            session: Session::new(0, 0),
            executed: false,
            error_code,
        };
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
    pub sqos: Vec<SQoS>,
    pub content: Content,
    pub session: Session,
}

impl SentMessage {
    pub fn new(id: u128, from_chain: String, sender: AccountId, signer: AccountId, message: ISentMessage) -> Self {
        Self {
            id,
            from_chain,
            to_chain: message.to_chain,
            sender,
            signer,
            sqos,
            content,
            session,
        }
    }

    pub fn new_sending_message(to_chain: String, sqos: Vec<SQoS>, session: Session, content: Content) -> Self {
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