include!("message_protocol.rs");

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
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
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

impl ::scale_info::TypeInfo for SQoSType {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("SQoSType", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("Reveal", |v| v.index(0))
                                .variant("Challenge", |v| v.index(1))
                                .variant("Threshold", |v| v.index(2))
                                .variant("Priority", |v| v.index(3))
                                .variant("ExceptionRollback", |v| v.index(4))
                                .variant("Anonymous", |v| v.index(5))
                                .variant("Identity", |v| v.index(6))
                                .variant("Isolation", |v| v.index(7))
                                .variant("CrossVerify", |v| v.index(8))
                        )
    }
}


#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct SQOS {
    pub reveal: u8,
    pub haha: MsgType,
}

impl SQOS {
    pub fn new(reveal: u8) -> Self {
        Self {
            reveal,
            haha: MsgType::InkString,
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
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
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
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: SQOS,
        contract: AccountId, action: String, data: Bytes, session: Session) -> Self {
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