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
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum IError {
    NotOwner,
    IdNotMatch,
    ChainMessageNotFound,
    IdOutOfBound,
    AlreadyExecuted,
    InterfaceNotFound,
}

impl scale_info::TypeInfo for IError {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IError", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("NotOwner", |v| v.index(0))
                                .variant("IdNotMatch", |v| v.index(1))
                                .variant("ChainMessageNotFound", |v| v.index(2))
                                .variant("IdOutOfBound", |v| v.index(3))
                                .variant("AlreadyExecuted", |v| v.index(4))
                                .variant("InterfaceNotFound", |v| v.index(5))
                        )
    }
}

/// Content structure
#[derive(Clone, Decode, Encode)]
// #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct IContent {
    contract: String,
    action: String,
    data: Bytes,
}

impl IContent {
    pub fn new(contract: String, action: String, data: Bytes) -> Self {
        Self {
            contract: contract,
            action: action,
            data: data,
        }
    }
}

impl scale_info::TypeInfo for IContent {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IContent", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<String>().name("contract").type_name("String"))
                        .field(|f| f.ty::<String>().name("action").type_name("String"))
                        .field(|f| f.ty::<Bytes>().name("data").type_name("Bytes"))
                    )
    }
}

/// SQOS structure
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum ISQoSType{
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

impl ::scale_info::TypeInfo for ISQoSType {
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

#[derive(Debug, Clone, Decode, Encode)]
// #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct ISQoS {
    pub t: ISQoSType,
    pub v: String,
}

impl scale_info::TypeInfo for ISQoS {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("ISQoS", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<ISQoSType>().name("t").type_name("ISQoSType"))
                        .field(|f| f.ty::<String>().name("v").type_name("String"))
                    )
    }
}

/// Session Structure
#[derive(Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct ISession {
    pub msg_type: u8,
    pub id: u128,
}

/// Received message structure
#[derive(Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct IReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub contract: AccountId,
    pub action: String,
    pub data: Bytes,
    pub session: ISession,
    pub executed: bool,
    pub error_code: u16,
}

impl IReceivedMessage {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: ink_prelude::vec::Vec<ISQoS>,
        contract: AccountId, action: String, data: Bytes, session: ISession) -> Self {
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
#[derive(Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SentMessage {
    pub id: u128,
    pub from_chain: String,
    pub to_chain: String,
    pub sender: AccountId,
    pub signer: AccountId,
    pub sqos: ISQoS,
    pub content: IContent,
    pub session: ISession,
}

impl SentMessage {
    pub fn new(id: u128, from_chain: String, to_chain: String, sender: AccountId, signer: AccountId,
        sqos: ISQoS, content: IContent, session: ISession) -> Self {
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

    pub fn new_sending_message(to_chain: String, sqos: ISQoS, session: ISession, content: IContent) -> Self {
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
#[derive(Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct IContext {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub contract: AccountId,
    pub action: String,
}

impl IContext {
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