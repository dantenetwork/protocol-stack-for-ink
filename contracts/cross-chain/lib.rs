#![cfg_attr(not(feature = "std"), no_std)]

pub mod cross_chain_base;
pub mod evaluation;
pub mod sqos_base;
pub mod storage_define;

#[ink::contract]
pub mod cross_chain {
    pub const PRECISION: u32 = 10_000;
    use crate::cross_chain_base::CrossChainBase;
    use crate::evaluation::RoutersCore;
    use crate::sqos_base::SQoSBase;
    use crate::storage_define::{
        AbandonedMessage, Bytes, Context, CredibilitySelectionRatio, Error, Evaluation,
        EvaluationCoefficient, Group, Message, SQoS, SQoSType, SentMessage, Session, Threshold,
    };
    use ink::prelude::{string::String, vec, vec::Vec};
    use ink::storage::Mapping;
    // use String as ChainId;
    use payload::message_define::{IContent, IContext, IReceivedMessage, ISentMessage, ISession};
    use payload::message_protocol::MessagePayload;

    struct Candidate {
        id: AccountId,
        low: u32,
        high: u32,
        selected: bool,
        credit: u32,
    }

    impl Candidate {
        pub fn contains(&self, value: u32) -> bool {
            value >= self.low && value < self.high
        }
    }

    /// Trait for owner
    #[ink::trait_definition]
    pub trait Ownable {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(&self) -> Option<AccountId>;
        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), Error>;
        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), Error>;
    }

    type SQoSMessage = Mapping<(String, u128), (Vec<(AccountId, Bytes)>, bool)>;
    type AggregationResult = (Vec<AccountId>, Vec<AccountId>, Vec<(Vec<AccountId>, u32)>);
    type ReceivedMessageTable = Mapping<(String, u128), (Vec<Group>, (bool, u64))>;
    type ExecutableMessageTable = Mapping<(String, u128), [u8; 32]>;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    // #[derive(SpreadAllocate)]
    pub struct CrossChain {
        // Data for Ownable
        /// Account id of owner
        owner: Option<AccountId>,
        // Data for CrossChainBase
        /// Current chain name
        chain_name: String,
        /// Dante token contract
        /// Table of sent messages
        sent_message_table: Mapping<(String, u128), SentMessage>,
        /// latest sent message id
        latest_sent_message_id: Mapping<String, u128>,
        /// Table of received messages
        received_message_table: ReceivedMessageTable,
        /// Table of pending messages key
        pending_message_key: Vec<(String, u128)>,
        /// latest received message id
        latest_message_id: Mapping<String, u128>,
        /// router final received message id
        final_received_message_id: Mapping<(String, AccountId), u128>,
        /// Table of executable messages and executed messages
        executable_message_table: ExecutableMessageTable,
        /// executable messages
        executable_key: Vec<(String, u128)>,

        abandoned_message: Mapping<String, Vec<AbandonedMessage>>,
        /// Context of a cross-contract call
        context: Option<Context>,

        evaluation: Evaluation,
        // SQoS
        sqos_table: Mapping<AccountId, SQoS>,
        // SQoS message, key: (from_chain, id), value: (RouterId, SQoSValue)
        sqos_message: SQoSMessage,

        callback: Mapping<(String, u128), bool>,
    }

    impl CrossChain {
        /// Constructor that initializes `chain_name`.
        #[ink(constructor)]
        pub fn new_default(chain_name: String) -> Self {
            Self {
                owner: Some(Self::env().caller()),
                chain_name,
                sent_message_table: Default::default(),
                latest_sent_message_id: Default::default(),
                received_message_table: Default::default(),
                pending_message_key: Vec::new(),
                latest_message_id: Default::default(),
                final_received_message_id: Default::default(),
                executable_message_table: Default::default(),
                executable_key: Vec::new(),
                abandoned_message: Default::default(),
                context: None,
                evaluation: Evaluation::new_default_evaluation(),
                sqos_table: Default::default(),
                sqos_message: Default::default(),
                callback: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn new(
            chain_name: String,
            threshold: Threshold,
            credibility_selection_ratio: CredibilitySelectionRatio,
            evaluation_coefficient: EvaluationCoefficient,
            initial_credibility_value: u32,
            selected_number: u8,
        ) -> Self {
            let evaluation = Evaluation {
                threshold,
                credibility_selection_ratio,
                evaluation_coefficient,
                current_routers: Vec::new(),
                routers: Vec::new(),
                initial_credibility_value,
                selected_number,
            };

            Self {
                owner: Some(Self::env().caller()),
                chain_name,
                sent_message_table: Default::default(),
                latest_sent_message_id: Default::default(),
                received_message_table: Default::default(),
                pending_message_key: Vec::new(),
                latest_message_id: Default::default(),
                final_received_message_id: Default::default(),
                executable_message_table: Default::default(),
                executable_key: Vec::new(),
                abandoned_message: Default::default(),
                context: None,
                evaluation,
                sqos_table: Default::default(),
                sqos_message: Default::default(),
                callback: Default::default(),
            }
        }

        /// If the caller is the owner of the contract
        fn only_owner(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.owner.unwrap() != caller {
                return Err(Error::NotOwner);
            }

            Ok(())
        }

        /// If the caller is a port
        // fn only_router(&self) -> Result<(), Error> {
        //     let caller = self.env().caller();

        //     for router in self.evaluation.current_routers.iter() {
        //         if *router == caller {
        //             return Ok(());
        //         }
        //     }

        //     Err(Error::NotRouter)
        // }

        /// Registers SQoS
        #[ink(message)]
        pub fn set_sqos(&mut self, contract: AccountId, sqos: SQoS) -> Result<(), Error> {
            self.only_owner().unwrap();
            // if self.sqos_table.contains(&contract) {
            //     return Err(Error::AlreadyRegister);
            // }
            self.sqos_table.insert(contract, &sqos);
            Ok(())
        }

        /// Returns SQoS
        #[ink(message)]
        pub fn get_sqos(&self, contract: AccountId) -> Option<SQoS> {
            self.sqos_table.get(&contract)
        }

        /// Method flushes the current state of `Self` into storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to flush the correct state of the contract,
        /// you have to this method on storage struct.
        fn flush(&self) {
            let root_key = <Self as ::ink::storage::traits::StorageKey>::KEY;
            ::ink::env::set_contract_storage::<::ink::primitives::Key, Self>(&root_key, self);
        }

        /// Method loads the current state of `Self` from storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to load the correct state of the contract,
        /// you have to this method on storage struct.
        fn load(&mut self) {
            let root_key = <Self as ::ink::storage::traits::StorageKey>::KEY;
            let mut state = ::ink::env::get_contract_storage(&root_key)
                .unwrap()
                .unwrap();
            core::mem::swap(self, &mut state);
            let _ = core::mem::ManuallyDrop::new(state);
        }
        // /// Cross-chain receives message from chain `from_chain`, the message
        // /// will be handled by method `action` of contract `to` with data `data`
        // #[ink(message)]
        fn internal_receive_message(
            &mut self,
            router: AccountId,
            message: Message,
        ) -> Result<(), Error> {
            // let router = self.env().caller();
            let message_hash = message.into_hash();
            let id = message.id;
            let key = (message.from_chain.clone(), id);
            let latest_message_id = self.latest_message_id.get(&message.from_chain).unwrap_or(0);
            if id == latest_message_id + 1 {
                self.latest_message_id.insert(&message.from_chain, &id);
            }
            if id > latest_message_id + 1 {
                return Err(Error::AheadOfId);
            }

            let router_key = (message.from_chain.clone(), router);
            let final_received_message_id =
                self.final_received_message_id.get(&router_key).unwrap_or(0);
            // if id == final_received_message_id {
            //     return Err(Error::AlreadReceived);
            // }

            // if (id < final_received_message_id
            //     || (id < latest_message_id + 1 && final_received_message_id == 0))
            //     && (!self.received_message_table.contains(&key)
            //         || self.received_message_table.get(&key).unwrap().1 .0)
            // {
            //     return Err(Error::ReceiveCompleted);
            // }

            if self.received_message_table.contains(&key)
                && self.received_message_table.get(&key).unwrap().1 .0
            {
                for group in self.received_message_table.get(&key).unwrap().0 {
                    // for group in
                    for r in group.routers {
                        if r == router {
                            return Err(Error::AlreadReceived);
                        }
                    }
                }
                return Err(Error::ReceiveCompleted);
            }

            if id > final_received_message_id {
                self.final_received_message_id.insert(&router_key, &id);
            }
            let router_credibility = self.evaluation.get_router_credibility(&router);
            match self.received_message_table.get(&key) {
                Some((mut groups, completed)) => {
                    let mut hash_found = false;
                    for group in groups.iter_mut() {
                        if group.message_hash == message_hash {
                            group.routers.push(router);
                            group.group_credibility_value += router_credibility as u64;
                            hash_found = true;
                        }
                    }
                    if !hash_found {
                        let group = Group {
                            message_hash,
                            message: message.clone(),
                            routers: ink::prelude::vec![router],
                            group_credibility_value: router_credibility as u64,
                            credibility_weight: 0,
                        };
                        groups.push(group);
                    }
                    self.received_message_table
                        .insert(&key, &(groups, completed));
                }
                None => {
                    let groups = ink::prelude::vec![Group {
                        message_hash,
                        message: message.clone(),
                        routers: ink::prelude::vec![router],
                        group_credibility_value: router_credibility as u64,
                        credibility_weight: 0,
                    }];
                    self.received_message_table
                        .insert(&key, &(groups, (false, Self::env().block_timestamp())));
                    self.pending_message_key.push(key.clone());
                }
            }

            // let mut len: u8 = 0;
            // let mut total_credibility: u64 = 0;
            // for group in self.received_message_table.get(&key).unwrap().0 {
            //     len += group.routers.len() as u8;
            //     total_credibility += group.group_credibility_value;
            // }

            // if len as usize >= self.evaluation.current_routers.len() {
            //     let (trusted, untrusted, exeception) = self.message_verify(&key, total_credibility);
            //     self.update_validator_credibility(trusted, untrusted, exeception);
            //     // TODO remove immediate?
            //     let mut receive_message = self.received_message_table.get(&key).unwrap();
            //     receive_message.1 = true;
            //     self.received_message_table.insert(key, &receive_message);
            // }
            Ok(())
        }

        fn internal_message_evaluation(&mut self, key: (String, u128), expected_routers: u8) {
            let mut len: u8 = 0;
            let mut total_credibility: u64 = 0;
            for group in self.received_message_table.get(&key).unwrap().0 {
                len += group.routers.len() as u8;
                total_credibility += group.group_credibility_value;
            }

            if len >= expected_routers {
                // let mut receive_message = self.received_message_table.get(&key).unwrap();
                // let groups: Vec<Group> = receive_message
                //     .0
                //     .clone()
                //     .into_iter()
                //     .map(|group| {
                //         let mut reture_value = group.clone();
                //         reture_value.credibility_weight =
                //             (group.group_credibility_value * 10000 / total_credibility) as u32;
                //         reture_value
                //     })
                //     .collect();
                let (trusted, untrusted, exeception) = self.message_verify(&key, total_credibility);
                self.update_validator_credibility(trusted, untrusted, exeception);
                // TODO remove immediate?
                // receive_message.0 = groups;
                // receive_message.1 = (true, Self::env().block_timestamp());
                // self.received_message_table.insert(key, &receive_message);
            }
        }

        fn message_verify(
            &mut self,
            key: &(String, u128),
            total_credibility: u64, // groups: &mut Vec<Group>,
        ) -> AggregationResult {
            let mut receive_message = self.received_message_table.get(&key).unwrap();
            let mut groups: Vec<Group> = receive_message
                .0
                .clone()
                .into_iter()
                .map(|group| {
                    let mut reture_value = group.clone();
                    reture_value.credibility_weight =
                        (group.group_credibility_value * 10000 / total_credibility) as u32;
                    reture_value
                })
                .collect();

            receive_message.0 = groups.clone();
            groups.sort_by(|a, b| b.credibility_weight.cmp(&a.credibility_weight));
            let mut trusted: Vec<AccountId> = Vec::new();
            let mut untrusted: Vec<AccountId> = Vec::new();
            let mut exeception: Vec<(Vec<AccountId>, u32)> = Vec::new();
            if groups[0].credibility_weight
                >= self.evaluation.threshold.credibility_weight_threshold
            {
                self.executable_message_table
                    .insert(&key, &groups[0].message_hash);
                self.executable_key.push(key.clone());
                trusted = groups.remove(0).routers;
                for group in groups {
                    untrusted.extend(group.routers.clone());
                }
                receive_message.1 = (true, Self::env().block_timestamp());
                self.received_message_table.insert(key, &receive_message);
                let index = self
                    .pending_message_key
                    .iter()
                    .position(|x| *x == *key)
                    .unwrap();
                self.pending_message_key.remove(index);
                // }
            } else {
                for group in groups {
                    exeception.push((group.routers.clone(), group.credibility_weight));
                }
                self.received_message_table.remove(key);
            }
            (trusted, untrusted, exeception)
        }

        pub fn update_validator_credibility(
            &mut self,
            trusted: Vec<AccountId>,
            untrusted: Vec<AccountId>,
            exeception: Vec<(Vec<AccountId>, u32)>,
        ) {
            // assert_eq!(
            //     env::predecessor_account_id(),
            //     self.vc_contract_id,
            //     "EVALUATION: Only call by vc contract"
            // );
            let mut credibility_value: u32;
            let coefficient = self.evaluation.evaluation_coefficient.clone();
            // update current trusted validators credibility
            for router in trusted {
                let origin_router_credibility = self.evaluation.get_router_credibility(&router);
                if origin_router_credibility < coefficient.middle_credibility {
                    credibility_value = coefficient.success_step
                        * (origin_router_credibility - coefficient.min_credibility)
                        / coefficient.range_crediblility
                        + origin_router_credibility;
                } else {
                    credibility_value = coefficient.success_step
                        * (coefficient.max_credibility - origin_router_credibility)
                        / coefficient.range_crediblility
                        + origin_router_credibility;
                }
                self.evaluation
                    .update_router_credibility(&router, credibility_value);
            }

            // update current untrusted validators credibility
            for router in untrusted {
                let origin_node_credibility = self.evaluation.get_router_credibility(&router);
                credibility_value = origin_node_credibility
                    - coefficient.do_evil_step
                        * (origin_node_credibility - coefficient.min_credibility)
                        / coefficient.range_crediblility;
                self.evaluation
                    .update_router_credibility(&router, credibility_value);
            }
            // update current exeception validators credibility
            for (routers, credibility_weight) in exeception {
                for router in routers {
                    let origin_node_credibility = self.evaluation.get_router_credibility(&router);
                    credibility_value = origin_node_credibility
                        - coefficient.exception_step
                            * (origin_node_credibility - coefficient.min_credibility)
                            / coefficient.range_crediblility
                            * (10_000 - credibility_weight)
                            / 10_000;
                    self.evaluation
                        .update_router_credibility(&router, credibility_value);
                }
            }
        }

        #[ink(message)]
        pub fn get_evaluation(&self) -> Evaluation {
            self.evaluation.clone()
        }

        #[ink(message)]
        pub fn get_msg_porting_task(&self, chain_name: String, router: AccountId) -> u128 {
            let final_received_message_id = self
                .final_received_message_id
                .get(&(chain_name.clone(), router))
                .unwrap_or(0);
            for key in self.pending_message_key.iter() {
                if key.0 != chain_name || key.1 <= final_received_message_id {
                    continue;
                }
                return key.1;
            }
            self.latest_message_id.get(&chain_name).unwrap_or(0) + 1
        }

        #[ink(message)]
        pub fn is_selected(&self, router: AccountId) -> bool {
            for r in self.evaluation.current_routers.iter() {
                if *r == router {
                    return true;
                }
            }
            false
        }

        #[ink(message)]
        pub fn get_current_routers(&self) -> Vec<AccountId> {
            self.evaluation.current_routers.clone()
        }

        /// Returns messages that sent from chains `chain_names` and can be executed.
        #[ink(message)]
        pub fn get_executable_message(&self, from_chain: String, id: u128) -> Option<[u8; 32]> {
            self.executable_message_table.get(&(from_chain, id))
        }

        // #[ink(message)]
        pub fn is_router(&self, account: &AccountId) -> (bool, u32) {
            for router in self.evaluation.routers.clone() {
                if *account == router.0 {
                    return (true, router.1);
                }
            }
            (false, 0)
        }

        fn generate_random(&self, index: &u8) -> [u8; 32] {
            let mut random_bytes = Self::env().block_timestamp().to_be_bytes().to_vec();
            random_bytes.push(index.clone());
            let mut output = [0; 32];
            ink::env::hash_bytes::<ink::env::hash::Sha2x256>(
                &random_bytes,
                &mut output,
            );
            output
        }
    }

    // process call
    impl CrossChain {
        fn process_error(&mut self, from_chain: String, id: u128) {
            let session = ISession::new(id, 105, Vec::new(), Vec::new(), Vec::new());
            let content = IContent::new(Vec::new(), Vec::new(), Vec::new());
            // let message = ISentMessage::new(message.from_chain, Vec::new(), content, session);
            let message = ISentMessage {
                to_chain: from_chain,
                sqos: Vec::new(),
                content,
                session,
            };
            self.send_message(message);
        }

        fn process_remote_error(&mut self, message: Message) -> Result<(), ink::env::Error> {
            self.remove_callback(message.from_chain.clone(), message.session.id);
            let selector: [u8; 4] = message.action;
            let contract = message.contract;
            let payload = MessagePayload::new();
            crate::call!(contract, selector, payload)
                .returns::<()>()
                .fire()
        }

        fn process_remote_call(&mut self, message: Message) -> Result<(), ink::env::Error> {
            self.remove_callback(message.from_chain.clone(), message.session.id);
            let selector: [u8; 4] = message.action;
            let contract = message.contract;
            let mut data_slice = message.data.as_slice();
            let payload: MessagePayload = scale::Decode::decode(&mut data_slice).unwrap();
            crate::call!(contract, selector, payload)
                .returns::<()>()
                .fire()
        }

        fn remove_callback(&mut self, to_chain: String, id: u128) {
            let key = (to_chain, id);
            // let key = (message.from_chain.clone(), message.session.id);
            if self.callback.contains(&key) {
                self.callback.remove(&key);
            }
        }
    }

    impl Ownable for CrossChain {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.owner
        }

        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), Error> {
            self.only_owner()?;

            self.owner = None;

            Ok(())
        }

        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), Error> {
            self.only_owner()?;

            self.owner = Some(new_owner);

            Ok(())
        }
    }

    impl CrossChainBase for CrossChain {
        /// Sets DAT token contract address
        #[ink(message)]
        fn set_token_contract(&mut self, _token: AccountId) {}

        /// Cross-chain calls method `action` of contract `contract` on chain
        /// `to_chain` with data `data`
        #[ink(message)]
        fn send_message(&mut self, message: ISentMessage) -> u128 {
            let latest_sent_message_id = self
                .latest_sent_message_id
                .get(&message.to_chain)
                .unwrap_or(0);
            let id = latest_sent_message_id + 1;
            let caller = Self::env().caller();
            let signer = caller;
            let sent_message: SentMessage =
                SentMessage::new(id, self.chain_name.clone(), caller, signer, message.clone());
            // TODO: For response message, context has value, and it is necessary ensure
            // the consistency of the message to prevent that SDK has not been tampered with.
            // if self.context.is_some() {
            // }
            self.latest_sent_message_id.insert(&message.to_chain, &id);
            self.sent_message_table
                .insert(&(message.to_chain, id), &sent_message);
            if sent_message.session.session_type == 2 {
                self.callback
                    .insert(&(sent_message.from_chain, sent_message.id), &true);
            }
            id
        }

        /// Cross-chain receives message from chain `from_chain`, the message
        /// will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, message: IReceivedMessage) -> Result<(), Error> {
            let caller = self.env().caller();
            let msg = Message::new(message.clone());
            let mut expected_routers: u8;
            expected_routers = self.evaluation.current_routers.len() as u8;
            let key = (msg.from_chain.clone(), msg.id);
            let mut is_revealer = false;
            // let contract = AccountId::from(msg.contract);
            if let Some(sqos) = self.sqos_table.get(&msg.contract) {
                match sqos.t {
                    SQoSType::Reveal => {
                        if let Some((submitted_message, completed)) = self.sqos_message.get(&key) {
                            if completed {
                                for (router, hash) in submitted_message {
                                    if router == caller {
                                        is_revealer = true;
                                        let caller_bytes: [u8; 32] = *(caller.as_ref());
                                        let data_bytes = ([
                                            message.id.to_be_bytes().to_vec(),
                                            message.from_chain.as_bytes().to_vec(),
                                            message.sender.clone(),
                                            message.signer.clone(),
                                            message.contract.to_vec(),
                                            message.action.to_vec(),
                                            message.data.clone(),
                                            message.session.into_raw_data(),
                                            caller_bytes.to_vec(),
                                        ])
                                        .concat();
                                        let mut message_hash = [0u8; 32];
                                        ink::env::hash_bytes::<ink::env::hash::Sha2x256>(
                                            &data_bytes,
                                            &mut message_hash,
                                        );
                                        if hash != message_hash.to_vec() {
                                            return Err(Error::RevealCheckFailed);
                                        }
                                    }
                                }
                                if !is_revealer {
                                    return Err(Error::NotRevealer);
                                }
                            } else {
                                return Err(Error::SQoSNotComplete);
                            }
                        }
                    }
                    SQoSType::Threshold => {
                        // sqos.v[0] value is 0 ~ 100
                        expected_routers = expected_routers * sqos.v[0] / 100;
                    }
                    _ => {}
                }
            }

            if !(self.is_selected(caller) || is_revealer) {
                return Err(Error::NotRouter);
            }
            self.internal_receive_message(caller, msg)?;
            self.internal_message_evaluation(key, expected_routers);
            Ok(())
        }

        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        #[ink(message)]
        fn abandon_message(
            &mut self,
            from_chain: String,
            id: u128,
            error_code: u16,
        ) -> Result<(), Error> {
            // self.only_router()?;
            let router = self.env().caller();
            if !self.is_selected(router) {
                return Err(Error::NotRouter);
            }
            let message = Message {
                id,
                from_chain: from_chain.clone(),
                sender: Vec::new(),
                signer: Vec::new(),
                sqos: Vec::new(),
                contract: AccountId::default(),
                action: [0; 4],
                data: Vec::new(),
                session: Session::new(0, 104, Vec::new(), Vec::new(), Vec::new()),
                error_code: Some(error_code),
            };
            self.internal_receive_message(router, message)?;
            self.internal_message_evaluation(
                (from_chain, id),
                self.evaluation.current_routers.len() as u8,
            );
            Ok(())
        }

        /// Returns messages that sent from chains `chain_names` and can be executed.
        #[ink(message)]
        fn get_executable_messages(&self, chain_names: Vec<String>) -> Vec<(String, u128)> {
            let mut vec: Vec<(String, u128)> = Vec::new();
            for chain in chain_names {
                for key in self.executable_key.clone() {
                    if key.0 == chain {
                        vec.push(key)
                    }
                }
            }
            vec
        }

        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128) -> Result<(), Error> {
            let key = (chain_name, id);
            if !self.executable_key.contains(&key) {
                return Err(Error::NotExecutable);
            }
            let message_hash = self.executable_message_table.get(&key).unwrap();
            let groups = self.received_message_table.get(&key).unwrap().0;
            for group in groups {
                if group.message_hash == message_hash {
                    let message = group.message;

                    if let Some(sqos) = self.sqos_table.get(&message.contract) {
                        if sqos.t == SQoSType::Challenge {
                            let current_time = Self::env().block_timestamp();
                            let confirm_windows = u64::from_be_bytes(sqos.v.try_into().unwrap());
                            if current_time - self.received_message_table.get(&key).unwrap().1 .1
                                >= confirm_windows
                            {
                                if let Some(sqos_messgae) = self.sqos_message.get(&key) {
                                    let mut total_credibility: u32 = 0;
                                    for (_, value) in sqos_messgae.0 {
                                        let credibility =
                                            u32::from_be_bytes(value.try_into().unwrap());
                                        total_credibility += credibility;
                                    }

                                    if total_credibility
                                        > group.group_credibility_value as u32
                                            * group.credibility_weight
                                            / 10_000
                                    {
                                        let index = self
                                            .executable_key
                                            .iter()
                                            .position(|x| {
                                                *x == (message.from_chain.clone(), message.id)
                                            })
                                            .unwrap();
                                        self.executable_key.remove(index);
                                        let coefficient =
                                            self.evaluation.evaluation_coefficient.clone();
                                        for router in group.routers {
                                            let origin_node_credibility =
                                                self.evaluation.get_router_credibility(&router);
                                            let credibility_value = origin_node_credibility
                                                - coefficient.do_evil_step
                                                    * (origin_node_credibility
                                                        - coefficient.min_credibility)
                                                    / coefficient.range_crediblility;
                                            self.evaluation.update_router_credibility(
                                                &router,
                                                credibility_value,
                                            )
                                        }
                                        
                                        self.pending_message_key.push(key.clone());
                                        self.received_message_table.remove(key.clone());
                                        self.sqos_message.remove(key);
                                        ink::env::debug_println!("challenge success");
                                        return Ok(());
                                    }
                                }
                            } else {
                                return Err(Error::SQoSNotComplete);
                            }
                        }
                    }

                    self.context = Some(Context {
                        id: message.id,
                        from_chain: message.from_chain.clone(),
                        sender: message.sender.clone(),
                        signer: message.signer.clone(),
                        sqos: message.sqos.clone(),
                        contract: message.contract,
                        action: message.action,
                        session: message.session.clone(),
                    });
                    // TODO
                    // currently, remove key from executable_key regardless of whether cross-call fails
                    let index = self.executable_key.iter().position(|x| *x == key).unwrap();
                    self.executable_key.remove(index);

                    self.flush();
                    if message.session.session_type == 104 {
                        self.process_error(message.from_chain, message.id);
                        return Ok(());
                    }
                    if message.session.session_type == 105 {
                        self.process_remote_error(message.clone()).unwrap();
                    } else {
                        let cc_result = self.process_remote_call(message.clone());
                        if cc_result.is_err() {
                            // let e = cc_result.unwrap_err();
                            self.process_error(message.from_chain, message.session.id);
                            // return Err(Error::CrossContractCallFailed);
                        }
                    };
                    self.load();
                    self.context = None;
                }
            }
            Ok(())
        }

        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(&self) -> Option<IContext> {
            self.context.as_ref()?;
            Some(self.context.clone().unwrap().derive())
        }

        /// Returns the number of messages sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message_number(&self, chain_name: String) -> u128 {
            self.latest_sent_message_id.get(&chain_name).unwrap_or(0)
        }

        /// Returns the number of messages received from chain `chain_name`
        #[ink(message)]
        fn get_received_message_number(&self, chain_name: String) -> u128 {
            self.latest_message_id.get(&chain_name).unwrap_or(0)
        }

        /// Returns the message with id `id` sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message(&self, chain_name: String, id: u128) -> Result<SentMessage, Error> {
            self.sent_message_table
                .get(&(chain_name, id))
                .ok_or(Error::ChainMessageNotFound)
        }

        /// Returns the message with id `id` received from chain `chain_name`
        #[ink(message)]
        fn get_received_message(
            &self,
            chain_name: String,
            id: u128,
        ) -> Result<(Vec<Group>, (bool, u64)), Error> {
            self.received_message_table
                .get(&(chain_name, id))
                .ok_or(Error::ChainMessageNotFound)
        }

        /// Returns the message abandoned from chain `chain_name`
        #[ink(message)]
        fn get_abandoned_message(&self, chain_name: String) -> Vec<AbandonedMessage> {
            self.abandoned_message.get(&chain_name).unwrap_or_default()
        }
    }

    impl RoutersCore for CrossChain {
        #[ink(message)]
        fn select_routers(&mut self) -> Result<Vec<AccountId>, Error> {
            self.only_owner()?;

            let mut total_credit = 0_u32;
            let mut candidates = Vec::<Candidate>::new();
            let mut trustworthy_all: u32 = 0;
            for index in self.evaluation.routers.iter() {
                if index.1 >= self.evaluation.threshold.min_seleted_threshold {
                    let c = Candidate {
                        id: index.0,
                        low: total_credit,
                        high: total_credit + index.1,
                        selected: false,
                        credit: index.1,
                    };

                    total_credit = c.high;
                    candidates.push(c);
                }
            }
            // ink::env::debug_println!("total_credit:{}", total_credit);
            // ink::env::debug_println!("candidates number:{}", candidates.len());

            if candidates.len() <= (self.evaluation.selected_number as usize) {
                // ink::env::debug_println!("{}", "Not Enough");
                let selected_routers: Vec<AccountId> =
                    candidates.into_iter().map(|c| c.id).collect();
                self.evaluation.current_routers = selected_routers;
            } else {
                // ink::env::debug_println!("{}", "Enough");
                // Compute total trustworthy value
                for c in candidates.iter() {
                    if c.credit >= self.evaluation.threshold.trustworthy_threshold {
                        let probability = PRECISION * c.credit / total_credit;
                        trustworthy_all += probability;
                    }
                }
                // ink::env::debug_println!("trustworthy_all:{}", trustworthy_all);

                // Number of credibility selecting
                let mut credibility_selected_ratio = trustworthy_all;
                if credibility_selected_ratio
                    > self.evaluation.credibility_selection_ratio.upper_limit
                {
                    credibility_selected_ratio =
                        self.evaluation.credibility_selection_ratio.upper_limit;
                }
                if credibility_selected_ratio
                    < self.evaluation.credibility_selection_ratio.lower_limit
                {
                    credibility_selected_ratio =
                        self.evaluation.credibility_selection_ratio.lower_limit;
                }
                // ink::env::debug_println!(
                //     "credibility_selected_ratio:{}",
                //     credibility_selected_ratio
                // );
                let credibility_selected_num = (self.evaluation.selected_number as u32)
                    * (credibility_selected_ratio as u32)
                    / PRECISION;
                // ink::env::debug_println!("credibility_selected_num:{}", credibility_selected_num);

                // Select routers according to credibility
                let mut selected_routers = Vec::<AccountId>::new();
                let mut start_index = 0;
                while selected_routers.len() < (credibility_selected_num as usize) {
                    // let random_seed =
                    //     ink::env::random::<ink::env::DefaultEnvironment>(&[start_index])
                    //         .unwrap()
                    //         .0;
                    let random_seed = self.generate_random(&start_index);
                    let mut seed_index = 0;

                    while seed_index < (random_seed.as_ref().len() - 1) {
                        let two_bytes: [u8; 2] = random_seed.as_ref()[seed_index..seed_index + 2]
                            .try_into()
                            .unwrap();
                        let rand_num = u16::from_be_bytes(two_bytes) as u64;

                        let rand_credit = rand_num * (total_credit as u64) / (u16::MAX as u64);
                        // ink::env::debug_println!(
                        //     "credit rand_num:{}, position:{}",
                        //     rand_num,
                        //     rand_credit
                        // );

                        let mut choose_next;
                        for c in candidates.iter_mut() {
                            if c.contains(rand_credit as u32) {
                                if !c.selected {
                                    selected_routers.push(c.id);
                                    c.selected = true;
                                    break;
                                } else {
                                    choose_next = true;
                                }

                                if choose_next && !c.selected {
                                    selected_routers.push(c.id);
                                    c.selected = true;
                                    break;
                                }
                            }
                        }

                        if selected_routers.len() >= (credibility_selected_num as usize) {
                            break;
                        }

                        seed_index += 2;
                    }

                    start_index += 1;
                }

                // Select routers randomly
                start_index += 1;
                while selected_routers.len() < (self.evaluation.selected_number as usize) {
                    // let random_seed =
                    //     ink::env::random::<ink::env::DefaultEnvironment>(&[start_index])
                    //         .unwrap()
                    //         .0;
                    let random_seed = self.generate_random(&start_index);
                    let mut seed_index = 0;

                    while seed_index < (random_seed.as_ref().len() - 1) {
                        let left_router_num = candidates.len() - selected_routers.len();
                        let two_bytes: [u8; 2] = random_seed.as_ref()[seed_index..seed_index + 2]
                            .try_into()
                            .unwrap();
                        let rand_num = u16::from_be_bytes(two_bytes) as u32;
                        let position = rand_num * (left_router_num as u32) / (u16::MAX as u32);
                        // ink::env::debug_println!(
                        //     "random rand_num:{}, posotion:{}",
                        //     rand_num,
                        //     position
                        // );

                        let mut pos_index = 0;
                        for i in candidates.iter_mut() {
                            if !i.selected {
                                if position == pos_index {
                                    selected_routers.push(i.id);
                                    i.selected = true;
                                    break;
                                }
                                pos_index += 1;
                            }
                        }

                        if selected_routers.len() >= (self.evaluation.selected_number as usize) {
                            break;
                        }

                        seed_index += 2;
                    }

                    start_index += 1;
                }
                self.evaluation.current_routers = selected_routers;
            }

            Ok(self.evaluation.current_routers.clone())
        }

        #[ink(message)]
        fn get_routers(&self) -> Vec<(AccountId, u32)> {
            self.evaluation.routers.clone()
        }

        #[ink(message)]
        fn register_router(&mut self, router: AccountId) -> Result<(), Error> {
            self.only_owner()?;

            for r in self.evaluation.routers.iter() {
                if r.0 == router {
                    return Err(Error::RouterAlreadyRegisterd);
                }
            }

            self.evaluation
                .routers
                .push((router, self.evaluation.initial_credibility_value));

            Ok(())
        }

        #[ink(message)]
        fn unregister_router(&mut self, router: AccountId) -> Result<(), Error> {
            self.only_owner()?;

            let mut index = 0;
            let mut found = false;
            for i in 0..self.evaluation.routers.len() {
                if self.evaluation.routers[i].0 == router {
                    found = true;
                    index = i;
                }
            }

            if !found {
                return Err(Error::RouterNotExist);
            }

            if index == self.evaluation.routers.len() - 1 {
                self.evaluation
                    .routers
                    .pop()
                    .ok_or(Error::RemoveRouterError)?;
            } else {
                let last_router = self
                    .evaluation
                    .routers
                    .pop()
                    .ok_or(Error::RemoveRouterError)?;
                self.evaluation.routers[index] = last_router;
            }

            Ok(())
        }

        #[ink(message)]
        fn set_initial_credibility(&mut self, value: u32) -> Result<(), Error> {
            self.only_owner()?;

            if value > PRECISION {
                return Err(Error::CreditBeyondUpLimit);
            }

            self.evaluation.initial_credibility_value = value;

            Ok(())
        }

        #[ink(message)]
        fn set_selected_number(&mut self, number: u8) -> Result<(), Error> {
            self.only_owner()?;

            self.evaluation.selected_number = number;

            Ok(())
        }

        #[ink(message)]
        fn set_threshold(&mut self, threshold: Threshold) -> Result<(), Error> {
            self.only_owner()?;

            if threshold.min_seleted_threshold > threshold.trustworthy_threshold {
                return Err(Error::CreditValueError);
            }

            if threshold.credibility_weight_threshold > PRECISION {
                return Err(Error::CreditBeyondUpLimit);
            }

            if threshold.trustworthy_threshold > PRECISION {
                return Err(Error::CreditBeyondUpLimit);
            }

            self.evaluation.threshold = threshold;

            Ok(())
        }

        #[ink(message)]
        fn set_credibility_selection_ratio(
            &mut self,
            ratio: CredibilitySelectionRatio,
        ) -> Result<(), Error> {
            self.only_owner()?;

            if ratio.lower_limit > ratio.upper_limit {
                return Err(Error::CreditValueError);
            }

            if ratio.upper_limit > PRECISION {
                return Err(Error::CreditBeyondUpLimit);
            }

            self.evaluation.credibility_selection_ratio = ratio;

            Ok(())
        }
    }

    // for debug
    impl CrossChain {
        #[ink(message)]
        pub fn clear_messages(&mut self, chain_name: String) -> Result<(), Error> {
            self.only_owner()?;
            let total = self.latest_message_id.get(&chain_name).unwrap_or(0);
            for i in 1..(total + 1) {
                let key = (chain_name.clone(), i);
                if self.received_message_table.get(&key).is_some() {
                    self.received_message_table.remove(&key);
                }
                if self.sqos_message.get(&key).is_some() {
                    self.sqos_message.remove(&key)
                }
                if self.executable_message_table.get(&key).is_some() {
                    self.executable_message_table.remove(&key)
                }
            }
            // for router
            for (router, _) in self.evaluation.routers.clone() {
                self.final_received_message_id
                    .insert((chain_name.clone(), router), &0);
            }
            self.executable_key = Vec::new();
            self.latest_message_id.insert(chain_name, &0);
            Ok(())
        }

        #[ink(message)]
        pub fn clear_send_messages(&mut self, chain_name: String) -> Result<(), Error> {
            self.only_owner()?;
            let total = self.latest_sent_message_id.get(&chain_name).unwrap_or(0);
            for i in 1..(total + 1) {
                if self
                    .sent_message_table
                    .get(&(chain_name.clone(), i))
                    .is_some()
                {
                    self.sent_message_table.remove(&(chain_name.clone(), i));
                }
            }
            self.latest_sent_message_id.insert(chain_name, &0);
            Ok(())
        }

        #[ink(message)]
        pub fn change_sqos(&mut self, contract: AccountId, sqos: SQoS) -> Result<(), Error> {
            self.sqos_table.insert(contract, &sqos);
            Ok(())
        }

        #[ink(message)]
        pub fn get_chain_name(&self) -> String {
            self.chain_name.clone()
        }

        #[ink(message)]
        pub fn register_routers(
            &mut self,
            routers: Vec<AccountId>,
            initial_credibility_value: u32,
        ) -> Result<(), Error> {
            self.only_owner()?;
            let mut contains: bool = false;
            for router in routers {
                for r in self.evaluation.routers.iter() {
                    if router == r.0 {
                        contains = true;
                        break;
                    }
                }
                if !contains {
                    self.evaluation
                        .routers
                        .push((router, initial_credibility_value));
                }
                contains = false;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn unregister_routers(&mut self) -> Result<(), Error> {
            self.only_owner()?;
            self.evaluation.routers.clear();
            self.evaluation.current_routers.clear();
            Ok(())
        }

        #[ink(message)]
        pub fn get_sqos_message(
            &self,
            from_chain: String,
            id: u128,
        ) -> (Vec<(AccountId, Bytes)>, bool) {
            self.sqos_message
                .get(&(from_chain, id))
                .unwrap_or((Vec::new(), false))
        }

        #[ink(message)]
        pub fn submitte_fake_message(
            &mut self,
            from_chain: String,
            malicious_router: AccountId,
            contract: AccountId,
        ) -> Result<(), Error> {
            let id = self.latest_message_id.get(&from_chain).unwrap_or_default() + 1;
            let key = (from_chain, id);
            if self.received_message_table.contains(&key) {
                return Err(Error::AlreadReceived);
            }
            // self.received_message_table.inser
            let message = Message {
                id: self.latest_message_id.get(&key.0).unwrap() + 1,
                from_chain: key.0.clone(),
                sender: Bytes::new(),
                signer: Bytes::new(),
                sqos: Vec::new(),
                contract,
                action: [0; 4],
                data: Bytes::new(),
                session: Session {
                    id: 0,
                    session_type: 0,
                    callback: Bytes::new(),
                    commitment: Bytes::new(),
                    answer: Bytes::new(),
                },
                error_code: None,
            };

            if !self.is_router(&malicious_router).0 {
                return Err(Error::NotRouter);
            }
            self.internal_receive_message(malicious_router, message)?;
            self.internal_message_evaluation(key, 1);
            Ok(())
            // received_message_table: Mapping<(String, u128), (Vec<Group>, (bool, u64))>,
        }
    }

    impl SQoSBase for CrossChain {
        #[ink(message)]
        fn receive_hidden_message(
            &mut self,
            from_chain: String,
            id: u128,
            contract: AccountId,
            signature: Bytes,
        ) -> Result<(), Error> {
            let caller = Self::env().caller();
            if !self.is_selected(caller) {
                return Err(Error::NotSelected);
            }
            let key = (from_chain, id);
            if self.sqos_table.contains(&contract) {
                if self.sqos_table.get(&contract).unwrap().t == SQoSType::Reveal {
                    match self.sqos_message.get(&key) {
                        Some(mut sqos) => {
                            if sqos.1 {
                                return Err(Error::Completed);
                            }
                            for (router, _) in sqos.0.iter() {
                                if *router == caller {
                                    return Err(Error::Exist);
                                }
                            }
                            sqos.0.push((caller, signature));
                            if sqos.0.len() >= self.evaluation.current_routers.len() {
                                sqos.1 = true;
                            }
                            self.sqos_message.insert(key, &sqos);
                        }
                        None => {
                            if self.evaluation.current_routers.len() <= 1 {
                                self.sqos_message
                                    .insert(key, &(vec![(caller, signature)], true));
                            } else {
                                self.sqos_message
                                    .insert(key, &(vec![(caller, signature)], false));
                            }
                        }
                    }
                } else {
                    return Err(Error::WrongSQoSType);
                }
            }
            Ok(())
        }

        #[ink(message)]
        fn challenge(&mut self, from_chain: String, id: u128) -> Result<(), Error> {
            let caller = Self::env().caller();
            // self.only_router().unwrap();
            let (is_router, credibility) = self.is_router(&caller);
            if !is_router {
                return Err(Error::NotRouter);
            }
            let key = (from_chain, id);
            if !self.executable_key.contains(&key) {
                return Err(Error::NotCompleted);
            }
            let message_hash = self.executable_message_table.get(&key).unwrap();
            let groups = self.received_message_table.get(&key).unwrap().0;
            for group in groups {
                if group.message_hash == message_hash {
                    let message = group.message;
                    if let Some(sqos) = self.sqos_table.get(&message.contract) {
                        if sqos.t != SQoSType::Challenge {
                            return Err(Error::WrongSQoSType);
                        }
                        if let Some(mut sqos) = self.sqos_message.get(&key) {
                            for (router, _) in sqos.0.iter() {
                                if *router == caller {
                                    return Err(Error::Exist);
                                }
                            }
                            sqos.0.push((caller, credibility.to_be_bytes().to_vec()));
                            self.sqos_message.insert(key.clone(), &sqos);
                        } else {
                            self.sqos_message.insert(
                                key.clone(),
                                &(vec![(caller, credibility.to_be_bytes().to_vec())], true),
                            );
                        }
                    } else {
                        return Err(Error::NotRegister);
                    }
                }
            }
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use crate::storage_define::{Content, Message};
        use ink::env::{
            test::{self, default_accounts, DefaultAccounts},
            DefaultEnvironment,
        };

        use ink::prelude::vec::Vec as Bytes;
        use payload::message_define::{IContent, ISQoS, ISession};

        fn init_default() -> (CrossChain, DefaultAccounts<DefaultEnvironment>) {
            let accounts = default_accounts::<DefaultEnvironment>();
            let cross_chain = CrossChain::new_default("POLKADOT".to_string());
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            (cross_chain, accounts)
        }

        fn init(
            credibility_weight_threshold: u32,
            initial_credibility_value: u32,
            trustworthy_threshold: u32,
        ) -> (CrossChain, DefaultAccounts<DefaultEnvironment>) {
            let threshold = Threshold {
                min_seleted_threshold: 3500,
                credibility_weight_threshold,
                trustworthy_threshold,
            };

            let evaluation_coefficient = EvaluationCoefficient {
                min_credibility: 0,
                max_credibility: 10_000,
                middle_credibility: (10_000 - 0) / 2,
                range_crediblility: 10_000 - 0,
                success_step: 100,
                do_evil_step: 200,
                exception_step: 100,
            };

            let credibility_selection_ratio = CredibilitySelectionRatio {
                upper_limit: 8000,
                lower_limit: 5000,
            };
            let accounts = default_accounts::<DefaultEnvironment>();
            let cross_chain = CrossChain::new(
                "POLKADOT".to_string(),
                threshold,
                credibility_selection_ratio,
                evaluation_coefficient,
                initial_credibility_value,
                13,
            );
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            (cross_chain, accounts)
        }

        fn to_ireceive_message(message: Message) -> IReceivedMessage {
            let mut sqos: Vec<ISQoS> = Vec::new();
            for s in message.sqos.into_iter() {
                sqos.push(SQoS::to(s));
            }
            IReceivedMessage {
                id: message.id,
                from_chain: message.from_chain,
                to_chain: "POLKADOT".to_string(),
                sender: message.sender,
                signer: message.signer,
                sqos,
                contract: *message.contract.as_ref(),
                action: message.action,
                data: message.data,
                session: ISession {
                    id: message.session.id,
                    session_type: 0,
                    callback: message.session.callback,
                    commitment: ink::prelude::vec::Vec::new(),
                    answer: ink::prelude::vec::Vec::new(),
                },
            }
        }

        // data is vec![String::from("Hi there"), String::from("1"), String::from("1")]
        fn get_message() -> (Message, Message, Message) {
            let message_1 = Message {
                id: 1,
                from_chain: String::from("ETHEREUM"),
                sender: String::from("0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b")
                    .as_bytes()
                    .to_vec(),
                signer: String::from("0x3aE841B899Ae4652784EA734cc61F524c36325d1")
                    .as_bytes()
                    .to_vec(),
                sqos: Vec::new(),
                contract: AccountId::from([0; 32]),
                action: [0x3a, 0x4a, 0x5a, 0x6a],
                data: vec![
                    1, 4, 32, 103, 114, 101, 101, 116, 105, 110, 103, 11, 12, 32, 72, 105, 32, 116,
                    104, 101, 114, 101, 4, 49, 4, 49,
                ],
                session: Session::new(0, 0, Vec::new(), Vec::new(), Vec::new()),
                error_code: None,
            };
            let message_2 = Message {
                id: 1,
                from_chain: String::from("ETHEREUM"),
                sender: String::from("0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b")
                    .as_bytes()
                    .to_vec(),
                signer: String::from("0x3aE841B899Ae4652784EA734cc61F524c36325d1")
                    .as_bytes()
                    .to_vec(),
                sqos: Vec::new(),
                contract: AccountId::from([1; 32]),
                action: [0x3a, 0x4a, 0x5a, 0x6a],
                data: vec![
                    1, 4, 32, 103, 114, 101, 101, 116, 105, 110, 103, 11, 12, 32, 72, 105, 32, 116,
                    104, 101, 114, 101, 4, 49, 4, 49,
                ],
                session: Session::new(0, 0, Vec::new(), Vec::new(), Vec::new()),
                error_code: None,
            };
            let message_3 = Message {
                id: 1,
                from_chain: String::new(),
                sender: Bytes::new(),
                signer: Bytes::new(),
                sqos: Vec::new(),
                contract: AccountId::default(),
                action: [0x3a, 0x4a, 0x5a, 0x6a],
                data: Vec::new(),
                session: Session::new(0, 0, Vec::new(), Vec::new(), Vec::new()),
                error_code: Some(1),
            };
            (message_1, message_2, message_3)
        }

        fn register_routers(
            cross_chain: &mut CrossChain,
            total_num: u8,
            selected_num: u8,
        ) -> Vec<AccountId> {
            // let mut routers: Vec<AccountId> = Vec::new();
            for i in 0..total_num {
                let bytes = u8::to_be_bytes(i);
                let mut account_bytes: [u8; 32] = [0; 32];
                account_bytes[31] = bytes[0];
                let acc = AccountId::from(account_bytes);
                // routers.push(acc);
                cross_chain.register_router(acc).unwrap();
            }
            cross_chain.set_selected_number(selected_num).unwrap();
            cross_chain.select_routers().unwrap()
        }

        fn get_unselected_routers(cross_chain: &CrossChain) -> Vec<AccountId> {
            // cross_chain.evaluation.routers.clone().into_iter().map(|(account, _)| if cross_chain.is_selected(account)).collect()
            let mut accounts: Vec<AccountId> = Vec::new();
            for (account, _) in cross_chain.evaluation.routers.clone().into_iter() {
                if cross_chain.is_selected(account) {
                    continue;
                }
                accounts.push(account);
            }
            return accounts;
        }

        fn receive_message(cross_chain: &mut CrossChain, routers: &[AccountId], message: Message) {
            let imessage = to_ireceive_message(message);
            let mut i = 0;
            for router in routers {
                test::set_caller::<DefaultEnvironment>(*router);
                i = i + 1;
                cross_chain.receive_message(imessage.clone()).unwrap();
            }
        }

        // fn receive_abandoned_message(
        //     cross_chain: &mut CrossChain,
        //     routers: &[AccountId],
        //     from_chain: String,
        //     id: u128,
        //     error_code: u16,
        // ) {
        //     // let imessage = to_ireceive_message(message);
        //     for router in routers {
        //         test::set_caller::<DefaultEnvironment>(*router);
        //         cross_chain
        //             .abandon_message(from_chain.clone(), id, error_code)
        //             .unwrap();
        //     }
        // }

        fn challenge(
            cross_chain: &mut CrossChain,
            from_chain: String,
            id: u128,
            routers: &[AccountId],
        ) {
            let mut i = 0;
            for router in routers {
                test::set_caller::<DefaultEnvironment>(*router);
                i = i + 1;
                cross_chain.challenge(from_chain.clone(), id).unwrap();
            }
        }

        /// Tests for trait Ownable
        #[ink::test]
        fn owner_works() {
            let (cross_chain, accounts) = init_default();
            // Owner should be Bob.
            assert_eq!(cross_chain.owner().unwrap(), accounts.alice);
        }

        #[ink::test]
        fn renounce_ownership_works() {
            let (mut cross_chain, _) = init_default();
            // Renounce ownership.
            cross_chain.renounce_ownership().unwrap();
            // Owner is None.
            assert_eq!(cross_chain.owner(), None);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let (mut cross_chain, accounts) = init_default();
            // Transfer ownership.
            cross_chain.transfer_ownership(accounts.bob).unwrap();
            // Owner is Bob.
            assert_eq!(cross_chain.owner().unwrap(), accounts.bob);
        }
        #[ink::test]
        fn only_owner_works() {
            let (cross_chain, _) = init_default();
            assert_eq!(cross_chain.only_owner(), Ok(()));
        }

        /// Test send message
        #[ink::test]
        fn test_send_message() {
            let (mut cross_chain, accounts) = init_default();
            let to_chain = "ETHEREUM".to_string();
            let send_message = SentMessage {
                id: 1,
                from_chain: String::from("POLKADOT"),
                to_chain: to_chain.clone(),
                sender: *accounts.alice.as_ref(),
                signer: *accounts.alice.as_ref(),
                sqos: Vec::new(),
                content: Content {
                    contract: String::from("ETHEREUM_CONTRACT").as_bytes().to_vec(),
                    action: String::from("ETHERERUM_ACTION").as_bytes().to_vec(),
                    data: Bytes::new(),
                },
                session: Session {
                    id: 0,
                    session_type: 0,
                    callback: Bytes::new(),
                    commitment: Bytes::new(),
                    answer: Bytes::new(),
                },
            };
            let content = send_message.content.clone();
            let session = send_message.session.clone();
            let imessage = ISentMessage {
                to_chain: to_chain.clone(),
                sqos: Vec::new(),
                content: IContent {
                    contract: content.contract,
                    action: content.action,
                    data: content.data,
                },
                session: ISession {
                    id: session.id,
                    session_type: 0,
                    callback: session.callback,
                    commitment: Bytes::new(),
                    answer: Bytes::new(),
                },
            };
            let id = cross_chain.send_message(imessage);
            // Number of sent messages is 1.
            let num = cross_chain.get_sent_message_number(to_chain.clone());
            assert_eq!(num, id);
            assert_eq!(
                send_message,
                cross_chain.get_sent_message(to_chain, id).unwrap()
            );
        }
        #[ink::test]
        fn test_select_routers() {
            let (mut cross_chain, _) = init_default();
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            assert_eq!(50, cross_chain.get_routers().len());
            assert_eq!(13, selected_routers.len());
            // println!("---- total routers ----");
            // for (router, _) in cross_chain.get_routers() {
            //     println!("{:?}", router);
            // }
            // println!("---- selected routers ----");
            // for router in selected_routers {
            //     println!("{:?}", router);
            // }
        }

        #[ink::test]
        fn test_receive_message() {
            let (mut cross_chain, _) = init_default();
            let selected_routers = register_routers(&mut cross_chain, 1, 1);
            let (message, _, _) = get_message();
            receive_message(&mut cross_chain, &selected_routers, message.clone());
            assert_eq!(
                message,
                cross_chain
                    .get_received_message(message.from_chain.clone(), message.id)
                    .unwrap()
                    .0[0]
                    .message
            );
            // println!(
            //     "{:?}",
            //     cross_chain
            //         .get_received_message(message.from_chain, message.id)
            //         .unwrap()
            //         .0[0]
            // );
        }

        /// test credibility < middle
        #[ink::test]
        fn test_routers_crediblity_greater_middle_crediblity() {
            let initial_credibiltiy_value: u32 = 4800u32;
            let credibility_weight_threshold: u32 = 1000u32;
            let trustworthy_threshold: u32 = 3500;
            let (mut cross_chain, _) = init(
                credibility_weight_threshold,
                initial_credibiltiy_value,
                trustworthy_threshold,
            );
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            // let routers = cross_chain.get_routers();
            // for router in selected_routers.iter() {
            //     for (r, v) in routers.iter() {
            //         if r == router {
            //             println!("({:?}, {})", *r, *v);
            //             break;
            //         }
            //     }
            // }
            let (message, _, _) = get_message();
            receive_message(&mut cross_chain, &selected_routers.clone(), message.clone());
            let executable_key =
                cross_chain.get_executable_messages(vec![message.from_chain.clone()]);
            assert_eq!(executable_key[0], (message.from_chain.clone(), message.id));
            let expect_value: u32 =
                100 * initial_credibiltiy_value / 10000 + initial_credibiltiy_value;
            let routers = cross_chain.get_routers();
            for router in selected_routers.iter() {
                for (r, v) in routers.iter() {
                    if r == router {
                        assert_eq!(expect_value, *v);
                        // println!("({:?}, {})", *r, *v);
                        break;
                    }
                }
            }
            // println!(
            //     "received_message: \n{:?}",
            //     cross_chain
            //         .get_received_message(message.from_chain, message.id)
            //         .unwrap()
            // )
        }

        /// test with untrusted node
        #[ink::test]
        pub fn test_with_untrusted() {
            let initial_credibiltiy_value: u32 = 6000u32;
            let credibility_weight_threshold: u32 = 1000u32;
            let trustworthy_threshold: u32 = 3500;
            let (mut cross_chain, _) = init(
                credibility_weight_threshold,
                initial_credibiltiy_value,
                trustworthy_threshold,
            );
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            // let routers = cross_chain.get_routers();
            // for router in selected_routers.iter() {
            //     for (r, v) in routers.iter() {
            //         if r == router {
            //             println!("({:?}, {})", *r, *v);
            //             break;
            //         }
            //     }
            // }
            let (message, malicious_message, _) = get_message();
            receive_message(&mut cross_chain, &selected_routers[..9], message.clone());
            receive_message(
                &mut cross_chain,
                &selected_routers[9..],
                malicious_message.clone(),
            );
            let executable_key =
                cross_chain.get_executable_messages(vec![message.from_chain.clone()]);
            assert_eq!(executable_key[0], (message.from_chain.clone(), message.id));
            let expect_trusted_value: u32 =
                100 * (10000 - initial_credibiltiy_value) / 10000 + initial_credibiltiy_value;
            // println!(
            //     "received_message: \n{:?}",
            //     cross_chain
            //         .get_received_message(message.from_chain, message.id)
            //         .unwrap()
            // );
            let routers = cross_chain.get_routers();
            for router in selected_routers[..9].iter() {
                for (r, v) in routers.iter() {
                    if r == router {
                        assert_eq!(expect_trusted_value, *v);
                        // println!("({:?}, {})", *r, *v);
                        break;
                    }
                }
            }
            let expect_untrusted_value: u32 =
                initial_credibiltiy_value - 200 * initial_credibiltiy_value / 10000;
            for router in selected_routers[9..].iter() {
                for (r, v) in routers.iter() {
                    if r == router {
                        assert_eq!(expect_untrusted_value, *v);
                        // println!("({:?}, {})", *r, *v);
                        break;
                    }
                }
            }
        }

        /// test with inconsistency
        #[ink::test]
        fn test_with_inconsistency() {
            let initial_credibiltiy_value: u32 = 6000u32;
            let credibility_weight_threshold: u32 = 6000u32;
            let trustworthy_threshold: u32 = 3500;
            let (mut cross_chain, _) = init(
                credibility_weight_threshold,
                initial_credibiltiy_value,
                trustworthy_threshold,
            );
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            // let routers = cross_chain.get_routers();
            // for router in selected_routers.iter() {
            //     for (r, v) in routers.iter() {
            //         if r == router {
            //             println!("({:?}, {})", *r, *v);
            //             break;
            //         }
            //     }
            // }
            let (message_1, message_2, _) = get_message();
            receive_message(&mut cross_chain, &selected_routers[..7], message_1.clone());
            receive_message(&mut cross_chain, &selected_routers[7..], message_2.clone());
            let executable_key =
                cross_chain.get_executable_messages(vec![message_1.from_chain.clone()]);
            assert_eq!(executable_key, Vec::new());
            let executable_key =
                cross_chain.get_executable_messages(vec![message_2.from_chain.clone()]);
            assert_eq!(executable_key, Vec::new());
            // println!(
            //     "received_message: \n{:?}",
            //     cross_chain
            //         .get_received_message(message_1.from_chain, message_1.id)
            //         .unwrap()
            // );
            let exception_total_credibility =
                initial_credibiltiy_value * selected_routers.len() as u32;
            let exception_total_credibility1 =
                initial_credibiltiy_value * selected_routers[..7].len() as u32;
            let exception_credibility_weight1 =
                10000 * exception_total_credibility1 / exception_total_credibility;

            let exception_total_credibility2 =
                initial_credibiltiy_value * selected_routers[7..].len() as u32;
            let exception_credibility_weight2 =
                10000 * exception_total_credibility2 / exception_total_credibility;

            let expect_exception_credibity1: u32 = initial_credibiltiy_value
                - 100 * (initial_credibiltiy_value) / 10000
                    * (10000 - exception_credibility_weight1)
                    / 10000;

            let routers = cross_chain.get_routers();
            for router in selected_routers[..7].iter() {
                for (r, v) in routers.iter() {
                    if r == router {
                        assert_eq!(expect_exception_credibity1, *v);
                        // println!("({:?}, {})", *r, *v);
                        break;
                    }
                }
            }
            let expect_exception_credibity2: u32 = initial_credibiltiy_value
                - 100 * (initial_credibiltiy_value) / 10000
                    * (10000 - exception_credibility_weight2)
                    / 10000;
            for router in selected_routers[7..].iter() {
                for (r, v) in routers.iter() {
                    if r == router {
                        assert_eq!(expect_exception_credibity2, *v);
                        // println!("({:?}, {})", *r, *v);
                        break;
                    }
                }
            }
        }

        // #[ink::test]
        // fn test_abandon_message() {
        //     let (mut cross_chain, _) = init_default();
        //     let (message, _, _) = get_message();
        //     let selected_routers = register_routers(&mut cross_chain, 50, 13);
        //     let error_code = 1u16;
        //     receive_message(&mut cross_chain, &selected_routers[..3], message.clone());
        //     receive_abandoned_message(
        //         &mut cross_chain,
        //         &selected_routers[3..],
        //         message.from_chain.clone(),
        //         message.id,
        //         error_code,
        //     );
        //     // let received_message = cross_chain.get_received_message(message.from_chain.clone(), message.id);
        //     // println!("{:?}", received_message);
        //     let abandoned_message = cross_chain.get_abandoned_message(message.from_chain.clone());
        //     // println!("{:?}", abandoned_message);
        //     let expected_abandoned_message = AbandonedMessage {
        //         id: message.id,
        //         error_code,
        //     };
        //     assert_eq!(abandoned_message[0], expected_abandoned_message);
        //     let executable_message = cross_chain.get_executable_messages(vec![message.from_chain]);
        //     assert_eq!(executable_message, Vec::new());
        // }

        #[ink::test]
        fn get_msg_porting_task() {
            let (mut cross_chain, _) = init_default();
            let (message, _, _) = get_message();
            let selected_routers = register_routers(&mut cross_chain, 2, 2);
            let id =
                cross_chain.get_msg_porting_task(message.from_chain.clone(), selected_routers[0]);
            assert_eq!(id, 1);
            receive_message(&mut cross_chain, &selected_routers[..1], message.clone());
            let mut message_2 = message.clone();
            message_2.id = message_2.id + 1;
            receive_message(&mut cross_chain, &selected_routers[..1], message_2.clone());
            let id =
                cross_chain.get_msg_porting_task(message.from_chain.clone(), selected_routers[1]);
            // id is 1
            assert_eq!(id, 1);
            // println!("{}", id);
            receive_message(&mut cross_chain, &selected_routers[1..], message.clone());
            let id =
                cross_chain.get_msg_porting_task(message.from_chain.clone(), selected_routers[1]);
            // id is 2
            assert_eq!(id, 2);
        }

        #[ink::test]
        fn test_sqos_challenge_failed() {
            let (mut cross_chain, _) = init_default();
            let (message, _, _) = get_message();
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            let v = (0u64).to_be_bytes().to_vec();
            cross_chain
                .set_sqos(
                    AccountId::from([0; 32]),
                    SQoS {
                        t: SQoSType::Challenge,
                        v,
                    },
                )
                .unwrap();
            receive_message(&mut cross_chain, &selected_routers, message.clone());
            let unselected_routers = get_unselected_routers(&cross_chain);
            challenge(
                &mut cross_chain,
                message.from_chain.clone(),
                message.id,
                &unselected_routers,
            );
            cross_chain
                .execute_message(message.from_chain, message.id)
                .unwrap();
        }

        #[ink::test]
        #[should_panic(
            expected = "not implemented: off-chain environment does not support contract invocation"
        )]
        fn test_sqos_challenge_successed() {
            let (mut cross_chain, _) = init_default();
            let (message, _, _) = get_message();
            let selected_routers = register_routers(&mut cross_chain, 50, 13);
            let v = (0u64).to_be_bytes().to_vec();
            cross_chain
                .set_sqos(
                    AccountId::from([0; 32]),
                    SQoS {
                        t: SQoSType::Challenge,
                        v,
                    },
                )
                .unwrap();
            receive_message(&mut cross_chain, &selected_routers, message.clone());
            let unselected_routers = get_unselected_routers(&cross_chain);
            challenge(
                &mut cross_chain,
                message.from_chain.clone(),
                message.id,
                &unselected_routers[..2],
            );
            // println!("{:?}", cross_chain.evaluation.routers);
            cross_chain
                .execute_message(message.from_chain, message.id)
                .unwrap();
            assert_eq!(cross_chain.executable_key, Vec::new());
        }

        #[ink::test]
        fn test_reveal_sqos() {
            let (mut cross_chain, _) = init_default();
            let selected_routers = register_routers(&mut cross_chain, 1, 1);
            let (message, _, _) = get_message();
            let sqos = SQoS {
                t: SQoSType::Reveal,
                v: Bytes::new(),
            };
            cross_chain
                .set_sqos(message.contract.clone(), sqos)
                .unwrap();
            let imessage = to_ireceive_message(message.clone());
            let caller_bytes: [u8; 32] = *(selected_routers[0].as_ref());
            let data_bytes = ([
                message.id.to_be_bytes().to_vec(),
                message.from_chain.as_bytes().to_vec(),
                message.sender.clone(),
                message.signer.clone(),
                imessage.contract.to_vec(),
                message.action.to_vec(),
                message.data.clone(),
                message.session.into_raw_data(),
                caller_bytes.to_vec(),
            ])
            .concat();
            let mut message_hash = [0u8; 32];
            ink::env::hash_bytes::<ink::env::hash::Sha2x256>(&data_bytes, &mut message_hash);
            // println!("message_hash: {:?}", message_hash);
            test::set_caller::<DefaultEnvironment>(selected_routers[0]);
            cross_chain
                .receive_hidden_message(
                    message.from_chain.clone(),
                    message.id,
                    message.contract,
                    message_hash.to_vec(),
                )
                .unwrap();
            cross_chain.get_sqos_message(message.from_chain, message.id);
            cross_chain.receive_message(imessage).unwrap();
        }
    }
}

#[macro_export]
macro_rules! call {
        ( $contract:ident, $selector:ident, $( $arg:expr ),* ) => {
            {
                let args = ink::env::call::ExecutionInput::new(ink::env::call::Selector::new($selector));
                $(
                    let args = args.push_arg($arg);
                )*
                ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                    .call_type(
                        ink::env::call::Call::new()
                            .callee($contract)
                            .gas_limit(0)
                            .transferred_value(0),
                    )
                    .exec_input(args)
                    .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
            }
        };
    }
