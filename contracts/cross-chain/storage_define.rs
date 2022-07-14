use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

use ink_env::AccountId;

use ink_prelude::{string::String, vec::Vec};

use scale::{Decode, Encode};

use payload::message_define::{
    IContent, IContext, IError, IReceivedMessage, ISQoS, ISQoSType, ISentMessage, ISession, 
};

pub type Bytes = Vec<u8>;
pub type Routers = Vec<AccountId>;

/// Errors for cross-chain contract
#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    IdNotMatch,
    ChainMessageNotFound,
    IdOutOfBound,
    NotExecutable,
    InterfaceNotFound,
    DecodeDataFailed,
    CrossContractCallFailed,
    NotRouter,
    RouterNotExist,
    RemoveRouterError,
    AheadOfId,
    AlreadReceived,
    ReceiveCompleted,
    RouterAlreadyRegisterd,
    CreditBeyondUpLimit,
    CreditValueError,
}

impl Error {
    pub fn from(e: IError) -> Self {
        match e {
            IError::NotOwner => Error::NotOwner,
            IError::IdNotMatch => Error::IdNotMatch,
            IError::ChainMessageNotFound => Error::ChainMessageNotFound,
            IError::IdOutOfBound => Error::IdOutOfBound,
            IError::AlreadyExecuted => Error::NotExecutable,
            IError::InterfaceNotFound => Error::InterfaceNotFound,
            IError::DecodeDataFailed => Error::DecodeDataFailed,
            IError::CrossContractCallFailed => Error::CrossContractCallFailed,
        }
    }
}

/// Content structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
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
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub enum SQoSType {
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
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct SQoS {
    pub t: SQoSType,
    pub v: Option<String>,
}

impl SQoS {
    pub fn new(t: SQoSType, v: Option<String>) -> Self {
        Self { t, v }
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
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Session {
    pub id: u128,
    pub callback: Option<Bytes>,
}

impl Session {
    pub fn new(id: u128, callback: Option<Bytes>) -> Self {
        Self { id, callback }
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
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct AbandonedMessage {
    pub id: u128,
    pub error_code: u16,
}

#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Message {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub contract: AccountId,
    pub action: [u8; 4],
    pub data: Bytes,
    pub session: Session,
    pub error_code: Option<u16>,
}

impl Message {
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
            error_code: None,
        }
    }

    pub fn into_hash(&self) -> [u8; 32] {
        let mut output = [0; 32];
        ink_env::hash_encoded::<ink_env::hash::Sha2x256, _>(&self, &mut output);
        output
    }
}

#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Group {
    pub message_hash: [u8; 32],
    pub message: Message,
    pub routers: Vec<AccountId>,
    pub group_credibility_value: u64,
    pub credibility_weight: u32,
}

impl Group {
    pub fn contains(&self, router: &AccountId) -> bool {
        for r in self.routers.iter() {
            if router == r {
                return true;
            }
        }
        false
    }
}

// impl AbandonedMessage {
//     pub fn new(message: IReceivedMessage) -> Self {
//         let mut sqos = Vec::<SQoS>::new();
//         for s in message.sqos {
//             sqos.push(SQoS::from(s));
//         }

//         Self {
//             message: Message {
//                 id: message.id,
//                 from_chain: message.from_chain,
//                 sender: message.sender,
//                 signer: message.signer,
//                 sqos,
//                 contract: AccountId::from(message.contract),
//                 action: message.action,
//                 data: message.data,
//                 session: Session::from(message.session),
//             },
//             error_code: 0,
//         }
//     }

//     pub fn new_with_error(id: u128, from_chain: String, error_code: u16) -> Self {
//         let m = Self {
//             message: Message {
//                 id,
//                 from_chain,
//                 sender: String::try_from("").unwrap(),
//                 signer: String::try_from("").unwrap(),
//                 sqos: Vec::<SQoS>::new(),
//                 contract: AccountId::default(),
//                 action: [0, 0, 0, 0],
//                 data: Bytes::new(),
//                 session: Session::new(0, None),
//             },
//             error_code,
//         };
//         m
//     }
// }

/// Sent message structure
#[derive(SpreadLayout, PackedLayout, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
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
    pub fn new(
        id: u128,
        from_chain: String,
        sender: AccountId,
        signer: AccountId,
        message: ISentMessage,
    ) -> Self {
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

    pub fn new_sending_message(
        to_chain: String,
        sqos: Vec<SQoS>,
        content: Content,
        session: Session,
    ) -> Self {
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
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Context {
    pub id: u128,
    pub from_chain: String,
    pub sender: String,
    pub signer: String,
    pub sqos: Vec<SQoS>,
    pub contract: AccountId,
    pub action: [u8; 4],
    pub session: Session,
}

impl Context {
    pub fn new(
        id: u128,
        from_chain: String,
        sender: String,
        signer: String,
        sqos: Vec<SQoS>,
        contract: AccountId,
        action: [u8; 4],
        session: Session,
    ) -> Self {
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
        IContext::new(
            self.id,
            self.from_chain.clone(),
            self.sender.clone(),
            self.signer.clone(),
            sqos,
            contract.clone(),
            self.action,
            session,
        )
    }
}

// Router Evaluation
#[derive(SpreadLayout, PackedLayout, SpreadAllocate, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct EvaluationCoefficient {
    pub min_credibility: u32,
    pub max_credibility: u32,
    pub middle_credibility: u32,
    pub range_crediblility: u32,
    pub success_step: u32,
    pub do_evil_step: u32,
    pub exception_step: u32,
}

#[derive(SpreadLayout, PackedLayout, SpreadAllocate, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct CredibilitySelectionRatio {
    pub upper_limit: u32,
    pub lower_limit: u32,
}

#[derive(SpreadLayout, PackedLayout, SpreadAllocate, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Threshold {
    pub credibility_weight_threshold: u32,
    pub min_seleted_threshold: u32,
    pub trustworthy_threshold: u32,
}

#[derive(SpreadLayout, PackedLayout, SpreadAllocate, Clone, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
)]
pub struct Evaluation {
    pub threshold: Threshold,
    pub credibility_selection_ratio: CredibilitySelectionRatio,
    pub evaluation_coefficient: EvaluationCoefficient,
    pub current_routers: Vec<AccountId>,
    pub routers: Vec<(AccountId, u32)>,
    pub initial_credibility_value: u32,
    pub selected_number: u8,
}

impl Evaluation {
    pub fn get_router_credibility(&self, router: &AccountId) -> u32 {
        for r in self.routers.iter() {
            if r.0 == *router {
                return r.1;
            }
        }
        0
    }

    pub fn update_router_credibility(&mut self, router: &AccountId, credibility: u32) {
        for r in self.routers.iter_mut() {
            if r.0 == *router {
                r.1 = credibility;
            }
        }
    }

    pub fn new_default_evaluation() -> Evaluation {
        Self {
            threshold: Threshold {
                credibility_weight_threshold: 4000,
                min_seleted_threshold: 3500,
                trustworthy_threshold: 3500,
            },
            credibility_selection_ratio: CredibilitySelectionRatio {
                upper_limit: 8000,
                lower_limit: 6000,
            },
            evaluation_coefficient: EvaluationCoefficient {
                min_credibility: 0,
                max_credibility: 10_000,
                middle_credibility: (10_000 - 0) / 2,
                range_crediblility: 10_000 - 0,
                success_step: 100,
                do_evil_step: 200,
                exception_step: 100,
            },
            current_routers: Vec::new(),
            routers: Vec::new(),
            initial_credibility_value: 4000,
            selected_number: 13,
        }
    }
}
