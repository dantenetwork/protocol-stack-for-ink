use ink_storage::{
    traits::{
        SpreadLayout,
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

use payload::message_define::{
    IContext,
    IError,
    ISQoSType,
    ISession,
    ISQoS,
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
    NotPorter,
}

impl Error {
    pub fn from(e: IError) -> Self {
        match e {
            IError::NotOwner => Error::NotOwner,
            IError::IdNotMatch => Error::IdNotMatch,
            IError::ChainMessageNotFound => Error::ChainMessageNotFound,
            IError::IdOutOfBound => Error::IdOutOfBound,
            IError::AlreadyExecuted => Error::AlreadyExecuted,
            IError::InterfaceNotFound => Error::InterfaceNotFound,
            IError::DecodeDataFailed => Error::DecodeDataFailed,
            IError::CrossContractCallFailed => Error::CrossContractCallFailed,
        }
    }
}

/// Content structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
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
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub enum SQoSType{
    Reveal,
    Challenge,
    Threshold,
    Priority,
    ExceptionRollback,
    SelectionDelay,
    Anonymous,
    Identity,
    Isolation,
    CrossVerify,
}

impl SQoSType {
    pub fn from(s: ISQoSType) -> Self {
        match s {
            ISQoSType::Reveal => SQoSType::Reveal,
            ISQoSType::Challenge => SQoSType::Challenge,
            ISQoSType::Threshold => SQoSType::Threshold,
            ISQoSType::Priority => SQoSType::Priority,
            ISQoSType::ExceptionRollback => SQoSType::ExceptionRollback,
            ISQoSType::SelectionDelay => SQoSType::SelectionDelay,
            ISQoSType::Anonymous => SQoSType::Anonymous,
            ISQoSType::Identity => SQoSType::Identity,
            ISQoSType::Isolation => SQoSType::Isolation,
            ISQoSType::CrossVerify => SQoSType::CrossVerify,
        }
    }
}

/// SQOS structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
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
            t: SQoSType::from(sqos.t),
            v: sqos.v,
        }
    }

    pub fn derive(&self) -> ISQoS {        
        let sqos_type = match self.t {
            SQoSType::Reveal => ISQoSType::Reveal,
            SQoSType::Challenge => ISQoSType::Challenge,
            SQoSType::Threshold => ISQoSType::Threshold,
            SQoSType::Priority => ISQoSType::Priority,
            SQoSType::ExceptionRollback => ISQoSType::ExceptionRollback,
            SQoSType::SelectionDelay => ISQoSType::SelectionDelay,
            SQoSType::Anonymous => ISQoSType::Anonymous,
            SQoSType::Identity => ISQoSType::Identity,
            SQoSType::Isolation => ISQoSType::Isolation,
            SQoSType::CrossVerify => ISQoSType::CrossVerify,
        };
        ISQoS::new(sqos_type, self.v.clone())
    }
}

/// Session Structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct Session {
    pub id: u128,
    pub callback: Option<Bytes>,
}

impl Session {
    pub fn new(id: u128, callback: Option<Bytes>) -> Self {
        Self {
            id,
            callback,
        }
    }

    pub fn from(session: ISession) -> Self {
        Self {
            id: session.id,
            callback: session.callback,
        }
    }
}

/// Received message structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct ReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub contract: AccountId,
    pub action: [u8;4],
    pub data: Bytes,
    pub session: Session,
    pub executed: bool,
    pub error_code: u16,
}

impl ReceivedMessage {
    pub fn new(message: IReceivedMessage) -> Self {
        let mut sqos = Vec::<SQoS>::new();
        for s in message.sqos {
            sqos.push(SQoS::from(s));
        }

        Self {
            id: message.id,
            from_chain: message.from_chain,
            sender: message.sender,
            signer: message.signer,
            sqos,
            contract: AccountId::from(message.contract),
            action: message.action,
            data: message.data,
            session: Session::from(message.session),
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
            action: [0, 0, 0, 0],
            data: Bytes::new(),
            session: Session::new(0, None),
            executed: false,
            error_code,
        };
        m
    }
}

/// Sent message structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
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
        let mut sqos = Vec::<SQoS>::new();
        for s in message.sqos {
            sqos.push(SQoS::from(s));
        }

        Self {
            id,
            from_chain,
            to_chain: message.to_chain,
            sender,
            signer,
            sqos,
            content: Content::from(message.content),
            session: Session::from(message.session),
        }
    }

    pub fn new_sending_message(to_chain: String, sqos: Vec<SQoS>, content: Content, session: Session) -> Self {
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
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct Context {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub contract: AccountId,
    pub action: [u8;4],
    pub session: Session,
}

impl Context {
    pub fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: Vec<SQoS>, contract: AccountId, action: [u8;4], session: Session) -> Self {
        Self {
            id,
            from_chain,
            sender,
            signer,
            sqos,
            contract,
            action,
            session,
        }
    }

    pub fn derive(&self) -> IContext {
        let mut sqos = Vec::<ISQoS>::new();
        
        for i in &self.sqos {
            let s = i.derive();
            sqos.push(s);
        }

        let contract: &[u8; 32] = AsRef::<[u8; 32]>::as_ref(&self.contract);

        let session = ISession::new(self.session.id, self.session.callback.clone());
        IContext::new(self.id, self.from_chain.clone(), self.sender.clone(), self.signer.clone(), sqos, contract.clone(), self.action, session)
    }
}