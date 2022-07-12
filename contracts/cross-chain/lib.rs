#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub mod cross_chain_base;
pub mod evaluation;
pub mod storage_define;

#[ink::contract]
pub mod cross_chain {
    use crate::cross_chain_base::CrossChainBase;
    use ink_lang as ink;
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::{traits::SpreadAllocate, Mapping};
    // use crate::storage_define::Evaluation;
    // use crate::evaluation::{ICredibilitySelectionRatio, IEvaluationCoefficient, IThreshold};
    use crate::storage_define::{
        AbandonedMessage, Context, CredibilitySelectionRatio, Error, Evaluation,
        EvaluationCoefficient, Group, Message, Routers, SQoS, SentMessage, Threshold,
    };
    // use String as ChainId;
    use payload::message_define::{IContext, IReceivedMessage, ISQoS, ISentMessage};
    use payload::message_protocol::MessagePayload;

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

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
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
        received_message_table: Mapping<(String, u128), (Vec<Group>, bool)>,
        /// latest received message id
        latest_message_id: Mapping<String, u128>,
        /// router final received message id
        final_received_message_id: Mapping<(String, AccountId), u128>,
        /// Table of executable messages
        executable_message_table: Mapping<String, Vec<Message>>,

        abandoned_message: Mapping<String, Vec<AbandonedMessage>>,
        /// Context of a cross-contract call
        context: Option<Context>,

        evaluation: Evaluation,
        // SQoS
        sqos_table: Mapping<AccountId, Vec<SQoS>>,
    }

    impl CrossChain {
        /// Constructor that initializes `chain_name`.
        #[ink(constructor)]
        pub fn new_default(chain_name: String) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, chain_name, Evaluation::new_default_evaluation())
            })
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
            ink_lang::utils::initialize_contract(|contract| {
                let evaluation = Evaluation {
                    threshold,
                    credibility_selection_ratio,
                    evaluation_coefficient,
                    current_routers: Vec::new(),
                    routers: Vec::new(),
                    initial_credibility_value,
                    selected_number,
                };
                Self::new_init(contract, chain_name, evaluation)
            })
        }

        /// Initializes the contract with the specified chain name.
        fn new_init(&mut self, chain_name: String, evaluation: Evaluation) {
            let caller = Self::env().caller();
            self.owner = Some(caller);
            self.chain_name = chain_name;
            self.evaluation = evaluation;
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
        fn only_router(&self) -> Result<(), Error> {
            let caller = self.env().caller();

            for router in self.evaluation.current_routers.iter() {
                if *router == caller {
                    return Ok(());
                }
            }

            Err(Error::NotRouter)
        }

        /// Receives message
        // fn internal_receive_message(&mut self, message: IReceivedMessage) -> Result<(), Error> {
        //     let mut chain_message = self.received_message_table.get(&message.from_chain).unwrap_or(Vec::<ReceivedMessage>::new());
        //     let current_id = chain_message.len() + 1;
        //     if current_id != message.id.try_into().unwrap() {
        //         return Err(Error::IdNotMatch)
        //     }

        //     let m = ReceivedMessage::new(message.clone());
        //     chain_message.push(m);
        //     self.received_message_table.insert(message.from_chain, &chain_message);
        //     Ok(())
        // }

        // /// Abandons message
        // fn internal_abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) -> Result<(), Error> {
        //     let mut chain_message = self.received_message_table.get(&from_chain).unwrap_or(Vec::<ReceivedMessage>::new());
        //     let current_id = chain_message.len() + 1;
        //     if current_id != (id as usize) {
        //         return Err(Error::IdNotMatch)
        //     }

        //     let message = ReceivedMessage::new_with_error(id, from_chain.clone(), error_code);
        //     chain_message.push(message);
        //     self.received_message_table.insert(from_chain, &chain_message);
        //     Ok(())
        // }

        /// Registers SQoS
        #[ink(message)]
        pub fn set_sqos(&mut self, sqos: Vec<ISQoS>) {
            let caller = Self::env().caller();
            let mut v_sqos = Vec::<SQoS>::new();
            for i in sqos {
                let s = SQoS::from(i);
                v_sqos.push(s);
            }
            self.sqos_table.insert(caller, &v_sqos);
        }

        /// Returns SQoS
        #[ink(message)]
        pub fn get_sqos(&self) -> Vec<ISQoS> {
            let caller = Self::env().caller();
            let sqos = self.sqos_table.get(caller).unwrap_or(Vec::<SQoS>::new());
            let mut ret = Vec::<ISQoS>::new();
            for i in sqos {
                ret.push(i.derive());
            }
            ret
        }

        /// Method flushes the current state of `Self` into storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to flush the correct state of the contract,
        /// you have to this method on storage struct.
        fn flush(&self) {
            let root_key = ::ink_primitives::Key::from([0x00; 32]);
            ::ink_storage::traits::push_spread_root::<Self>(self, &root_key);
        }

        /// Method loads the current state of `Self` from storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to load the correct state of the contract,
        /// you have to this method on storage struct.
        fn load(&mut self) {
            let root_key = ::ink_primitives::Key::from([0x00; 32]);
            let mut state = ::ink_storage::traits::pull_spread_root::<Self>(&root_key);
            core::mem::swap(self, &mut state);
            let _ = core::mem::ManuallyDrop::new(state);
        }

        fn message_verify(
            &mut self,
            key: &(String, u128),
            total_credibility: u64,
        ) -> (Vec<AccountId>, Vec<AccountId>, Vec<(Vec<AccountId>, u32)>) {
            let mut aggregation: Vec<Group> = self
                .received_message_table
                .get(&key)
                .unwrap()
                .0
                .into_iter()
                .map(|group| {
                    let mut reture_value = group.clone();
                    reture_value.credibility_weight =
                        (group.group_credibility_value * 10000 / total_credibility) as u32;
                    reture_value
                })
                .collect();
            aggregation.sort_by(|a, b| b.credibility_weight.cmp(&a.credibility_weight));
            let mut trusted: Vec<AccountId> = Vec::new();
            let mut untrusted: Vec<AccountId> = Vec::new();
            let mut exeception: Vec<(Vec<AccountId>, u32)> = Vec::new();
            if aggregation[0].credibility_weight
                >= self.evaluation.threshold.credibility_weight_threshold
            {
                if aggregation[0].message.error_code.is_some() {
                    let mut abandoned_messages = self
                        .abandoned_message
                        .get(&aggregation[0].message.from_chain)
                        .unwrap_or(Vec::new());
                    abandoned_messages.push(AbandonedMessage {
                        id: aggregation[0].message.id,
                        error_code: aggregation[0].message.error_code.unwrap(),
                    });
                    self.abandoned_message
                        .insert(&aggregation[0].message.from_chain, &abandoned_messages);
                } else {
                    let mut executable = self
                        .executable_message_table
                        .get(&key.0)
                        .unwrap_or(Vec::new());
                    executable.push(aggregation[0].message.clone());
                    self.executable_message_table.insert(&key.0, &executable);
                    trusted = aggregation.remove(0).routers;
                    for group in aggregation {
                        untrusted.extend(group.routers);
                    }
                }
            } else {
                for group in aggregation {
                    exeception.push((group.routers, group.credibility_weight));
                }
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

        /// For debug
        #[ink(message)]
        pub fn clear_messages(&mut self, chain_name: String) -> Result<(), Error> {
            self.only_owner()?;
            let total = self.latest_message_id.get(&chain_name).unwrap();
            for i in 0..total {
                if self
                    .received_message_table
                    .get(&(chain_name.clone(), i))
                    .is_some()
                {
                    self.received_message_table.remove(&(chain_name.clone(), i));
                }
            }
            Ok(())
        }

        /// For debug
        #[ink(message)]
        pub fn get_chain_name(&self) -> String {
            self.chain_name.clone()
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
            let signer = caller.clone();
            let sent_message: SentMessage = SentMessage::new(
                id.try_into().unwrap(),
                self.chain_name.clone(),
                caller,
                signer,
                message.clone(),
            );
            self.sent_message_table
                .insert(&(message.to_chain, id), &sent_message);
            id
        }

        // #[ink(message)]
        // fn receive_message(&mut self, message: IReceivedMessage) -> Result<(), Error> {
        //     self.only_owner()?;
        //     let messsage = Message::new(message);

        // }
        // /// Cross-chain receives message from chain `from_chain`, the message
        // /// will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, message: IReceivedMessage) -> Result<(), Error> {
            self.only_router()?;
            let router = self.env().caller();
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
            if id != final_received_message_id {
                return Err(Error::AlreadReceived);
            }

            if id < final_received_message_id
                || (id < latest_message_id + 1 && final_received_message_id == 0)
            {
                if !self.received_message_table.contains(&key)
                    || self.received_message_table.get(&key).unwrap().1
                {
                    return Err(Error::ReceiveCompleted);
                }
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
                            message: Message::new(message.clone()),
                            routers: ink_prelude::vec![router],
                            group_credibility_value: router_credibility as u64,
                            credibility_weight: 0,
                        };
                        groups.push(group);
                    }
                    self.received_message_table
                        .insert(&key, &(groups, completed));
                }
                None => {
                    let groups = ink_prelude::vec![Group {
                        message_hash,
                        message: Message::new(message.clone()),
                        routers: ink_prelude::vec![router],
                        group_credibility_value: router_credibility as u64,
                        credibility_weight: 0,
                    }];
                    self.received_message_table.insert(&key, &(groups, false));
                }
            }

            let mut len: u8 = 0;
            let mut total_credibility: u64 = 0;
            for group in self.received_message_table.get(&key).unwrap().0 {
                len += group.routers.len() as u8;
                total_credibility += group.group_credibility_value;
            }

            if len >= self.evaluation.selected_number {
                let (trusted, untrusted, exeception) = self.message_verify(&key, total_credibility);
                self.update_validator_credibility(trusted, untrusted, exeception);
                // TODO remove immediate?
                // self.received_message_table.get(&key).as_mut().and_then(|received_message|{received_message.1 = true;})
                self.received_message_table.remove(&key);
            }
            Ok(())
            // self.internal_receive_message(message)
        }

        // /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        // #[ink(message)]
        // fn abandon_message(
        //     &mut self,
        //     from_chain: String,
        //     id: u128,
        //     error_code: u16,
        // ) -> Result<(), Error> {
        //     self.only_router()?;

        //     self.internal_abandon_message(from_chain, id, error_code)
        // }

        /// Returns messages that sent from chains `chain_names` and can be executed.
        #[ink(message)]
        fn get_executable_messages(&self, chain_names: Vec<String>) -> Vec<Message> {
            let mut ret = Vec::<Message>::new();

            for chain_name in chain_names {
                let messages: Vec<Message> = self
                    .executable_message_table
                    .get(&chain_name)
                    .unwrap_or(Vec::new());
                ret.extend(messages);
            }
            ret
        }

        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128) -> Result<String, Error> {
            let mut executable_messages: Vec<Message> = self
                .executable_message_table
                .get(&chain_name)
                .ok_or(Error::ChainMessageNotFound)?;
            let mut message: Option<Message> = None;
            let mut index: usize = 0;
            for (i, m) in executable_messages.iter().enumerate() {
                // for m in executable_messages.iter() {
                if m.id == id {
                    message = Some(m.clone());
                    index = i;
                    break;
                }
            }
            if message.is_none() {
                return Err(Error::IdOutOfBound);
            }
            executable_messages.remove(index);
            let message = message.unwrap();
            self.context = Some(Context::new(
                message.id,
                message.from_chain.clone(),
                message.sender.clone(),
                message.signer.clone(),
                message.sqos.clone(),
                message.contract.clone(),
                message.action.clone(),
                message.session.clone(),
            ));

            // Construct paylaod
            let mut data_slice = message.data.as_slice();
            let payload: MessagePayload = scale::Decode::decode(&mut data_slice)
                .ok()
                .ok_or(Error::DecodeDataFailed)?;

            self.flush();

            // Cross-contract call
            let selector: [u8; 4] = message.action.clone().try_into().unwrap();
            let cc_result: Result<String, ink_env::Error> =
                ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                    .call_type(
                        ink_env::call::Call::new()
                            .callee(message.contract)
                            .gas_limit(0)
                            .transferred_value(0),
                    )
                    .exec_input(
                        ink_env::call::ExecutionInput::new(ink_env::call::Selector::new(selector))
                            .push_arg(payload),
                    )
                    .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                    .returns::<String>()
                    .fire();

            self.load();

            if cc_result.is_err() {
                // let e = cc_result.unwrap_err();
                return Err(Error::CrossContractCallFailed);
            }

            Ok(cc_result.unwrap())
        }

        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(&self) -> Option<IContext> {
            if self.context.is_none() {
                return None;
            }

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
        ) -> Result<(Vec<Group>, bool), Error> {
            self.received_message_table
                .get(&(chain_name, id))
                .ok_or(Error::ChainMessageNotFound)
        }
    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;
    //     use ink_prelude::vec::Vec as Bytes;
    //     use payload::message_define::{IContent, ISQoS, ISQoSType, ISession};
    //     use std::{fmt::Write, num::ParseIntError};

    //     fn set_caller(sender: AccountId) {
    //         ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
    //     }

    //     fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    //         (0..s.len())
    //             .step_by(2)
    //             .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
    //             .collect()
    //     }

    //     fn create_contract_with_received_message() -> CrossChain {
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Receive message.
    //         let from_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let sender = "0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b".to_string();
    //         let signer = "0x3aE841B899Ae4652784EA734cc61F524c36325d1".to_string();
    //         let contract = [0; 32];
    //         let mut action = [0x3a, 0x4a, 0x5a, 0x6a];
    //         let sqos = Vec::<ISQoS>::new();
    //         let raw_data = "010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000201c68746875616e67030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600".to_string();
    //         let data = decode_hex(&raw_data).unwrap();
    //         let session = ISession::new(0, None);
    //         let message = IReceivedMessage::new(
    //             id, from_chain, sender, signer, sqos, contract, action, data, session,
    //         );
    //         cross_chain.receive_message(message);
    //         cross_chain
    //     }

    //     fn create_contract_with_sent_message() -> CrossChain {
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());

    //         // Send message.
    //         let to_chain = "ETHEREUM".to_string();
    //         let contract = "ETHEREUM_CONTRACT".to_string();
    //         let action = "ETHERERUM_ACTION".to_string();
    //         let data = Bytes::new();
    //         let sqos = Vec::<ISQoS>::new();
    //         let session = ISession::new(0, None);
    //         let content = IContent::new(contract, action, data);
    //         let message = ISentMessage::new(to_chain.clone(), sqos, content, session);
    //         cross_chain.send_message(message);
    //         cross_chain
    //     }

    //     /// We test if the new constructor does its job.
    //     #[ink::test]
    //     fn new_works() {
    //         // Constructor works.
    //         let cross_chain = CrossChain::new("POLKADOT".to_string());
    //     }

    //     /// Tests for trait Ownable
    //     #[ink::test]
    //     fn owner_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         set_caller(accounts.bob);
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Owner should be Bob.
    //         assert_eq!(cross_chain.owner().unwrap(), accounts.bob);
    //     }

    //     #[ink::test]
    //     fn renounce_ownership_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Renounce ownership.
    //         cross_chain.renounce_ownership();
    //         // Owner is None.
    //         assert_eq!(cross_chain.owner(), None);
    //     }

    //     #[ink::test]
    //     fn transfer_ownership_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Transfer ownership.
    //         cross_chain.transfer_ownership(accounts.bob);
    //         // Owner is Bob.
    //         assert_eq!(cross_chain.owner().unwrap(), accounts.bob);
    //     }

    //     #[ink::test]
    //     fn only_owner_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Call of only_owner should return Ok.
    //         set_caller(accounts.alice);
    //         assert_eq!(cross_chain.only_owner(), Ok(()));
    //     }

    //     #[ink::test]
    //     fn not_owner_only_owner_should_fail() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Call of only_owner should return Err.
    //         set_caller(accounts.bob);
    //         assert_eq!(cross_chain.only_owner(), Err(Error::NotOwner));
    //     }

    //     /// Tests for CrossChainBase
    //     #[ink::test]
    //     fn send_message_works() {
    //         let to_chain = "ETHEREUM".to_string();
    //         let cross_chain = create_contract_with_sent_message();
    //         // Number of sent messages is 1.
    //         let num = cross_chain.sent_message_table.get(&to_chain).unwrap().len();
    //         assert_eq!(num, 1);
    //     }

    //     #[ink::test]
    //     fn receive_message_works() {
    //         let from_chain = "ETHEREUM".to_string();
    //         let cross_chain = create_contract_with_received_message();
    //         // Number of sent messages is 1.
    //         let num = cross_chain
    //             .received_message_table
    //             .get(&from_chain)
    //             .unwrap()
    //             .len();
    //         assert_eq!(num, 1);
    //     }

    //     #[ink::test]
    //     fn abandon_message_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Receive message.
    //         let from_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let error_code = 1;
    //         cross_chain.abandon_message(from_chain.clone(), id, error_code);
    //         // Number of sent messages is 1.
    //         let num = cross_chain
    //             .received_message_table
    //             .get(&from_chain)
    //             .unwrap()
    //             .len();
    //         assert_eq!(num, 1);
    //     }

    //     #[ink::test]
    //     fn get_executable_messages_works() {
    //         let from_chain = "ETHEREUM".to_string();
    //         let mut cross_chain = create_contract_with_received_message();
    //         // Number of sent messages is 1.
    //         let num = cross_chain
    //             .received_message_table
    //             .get(&from_chain)
    //             .unwrap()
    //             .len();
    //         assert_eq!(num, 1);
    //         // Get executable messages
    //         let mut chains = Vec::<String>::new();
    //         chains.push("ETHEREUM".to_string());
    //         let messages = cross_chain.get_executable_messages(chains);
    //         // Number of messages is 1
    //         assert_eq!(messages.len(), 1);
    //     }

    //     #[ink::test]
    //     fn execute_message_works() {
    //         // let from_chain = "ETHEREUM".to_string();
    //         // let id = 1;
    //         // let mut cross_chain = create_contract_with_received_message();
    //         // // Execute message
    //         // let ret = cross_chain.execute_message(from_chain.clone(), id);
    //         // assert_eq!(ret, Ok(()));
    //         println!("Cross-contract call can not be tested");
    //     }

    //     #[ink::test]
    //     fn get_context_works() {
    //         // let from_chain = "ETHEREUM".to_string();
    //         // let id = 1;
    //         // let mut cross_chain = create_contract_with_received_message();
    //         // // Execute message
    //         // let ret = cross_chain.execute_message(from_chain.clone(), id);
    //         // assert_eq!(ret, Ok(()));
    //         // // Context not None.
    //         // let context = cross_chain.get_context();
    //         // assert_eq!(context.is_some(), true);
    //         println!("Cross-contract call can not be tested");
    //     }

    //     #[ink::test]
    //     fn get_sent_message_number_works() {
    //         let to_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let mut cross_chain = create_contract_with_sent_message();
    //         // Number of sent messages is 1.
    //         let num = cross_chain.get_sent_message_number(to_chain);
    //         assert_eq!(num, 1);
    //     }

    //     #[ink::test]
    //     fn get_received_message_number_works() {
    //         let from_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let mut cross_chain = create_contract_with_received_message();
    //         // Number of received messages is 1.
    //         let num = cross_chain.get_received_message_number(from_chain);
    //         assert_eq!(num, 1);
    //     }

    //     #[ink::test]
    //     fn get_sent_message_works() {
    //         let to_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let mut cross_chain = create_contract_with_sent_message();
    //         // Sent message is Ok.
    //         let message = cross_chain.get_sent_message(to_chain, 1);
    //         assert_eq!(message.is_ok(), true);
    //     }

    //     #[ink::test]
    //     fn get_received_message_works() {
    //         let from_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let mut cross_chain = create_contract_with_received_message();
    //         // Received message is Ok.
    //         let message = cross_chain.get_received_message(from_chain, 1);
    //         assert_eq!(message.is_ok(), true);
    //     }

    //     // Tests for trait MultiRouters
    //     #[ink::test]
    //     fn change_routers_and_requirement_works() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let mut cross_chain = CrossChain::new("POLKADOT".to_string());
    //         // Resister.
    //         let mut routers = Routers::new();
    //         routers.push(accounts.alice);
    //         routers.push(accounts.bob);
    //         let required = 2;
    //         cross_chain.change_routers_and_requirement(routers.clone(), required);
    //         // Requirement is 2.
    //         let r = cross_chain.get_requirement();
    //         assert_eq!(r, 2);
    //         // Check routers.
    //         let p = cross_chain.get_routers();
    //         assert_eq!(p, routers);
    //     }

    //     #[ink::test]
    //     fn get_msg_porting_task() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         let from_chain = "ETHEREUM".to_string();
    //         let id = 1;
    //         let mut cross_chain = create_contract_with_received_message();
    //         // Received message is Ok.
    //         let message = cross_chain.get_received_message(from_chain.clone(), 1);
    //         assert_eq!(message.is_ok(), true);
    //         // Get porting task id
    //         let id = cross_chain.get_msg_porting_task(from_chain, accounts.alice);
    //         // id is 2
    //         assert_eq!(id, 2);
    //     }

    //     #[ink::test]
    //     fn get_selector() {
    //         let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
    //         // Create a new contract instance.
    //         let s = vec![0x3a, 0x6e, 0x96, 0x96];
    //         let selector: [u8; 4] = s.clone().try_into().unwrap();
    //         println!("{:?}", selector);
    //     }
    // }
}
