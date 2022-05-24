#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub mod payload;
pub mod message_define;

pub use self::cross_chain::{
    CrossChain,
    CrossChainRef,
};

#[ink::contract]
mod cross_chain {
    use ink_lang as ink;    
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };

    use ink_prelude::{
        vec::Vec,
        string::String,
    };

    use scale::{
        Encode,
        Decode,
    };

    use super::message_define::{
        Error,
        Content,
        SQOS,
        Session,
        ReceivedMessage,
        SentMessage,
        Context,
        Bytes,
        Porters,
    };

    use super::payload::MessagePayload;

    /// Trait for owner
    #[ink::trait_definition]
    pub trait Ownable {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(& self) -> Option<AccountId>;
        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self);
        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId);
    }

    /// Trait for basic cross-chain contract
    #[ink::trait_definition]
    pub trait CrossChainBase {
        /// Sets DAT token contract address
        #[ink(message)]
        fn set_token_contract(&mut self, token: AccountId);
        /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
        #[ink(message)]
        fn send_message(&mut self, to_chain: String, contract: String, action: String, sqos: SQOS, data: MessagePayload, session: Session);
        /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String, sqos: SQOS, contract: AccountId, action: String, data: MessagePayload, session: Session);
        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        #[ink(message)]
        fn abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) -> Result<(), Error>;
        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128) -> Result<(), Error>;
        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(& self) -> Option<Context>;
        /// Returns the number of messages sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message_number(& self, chain_name: String) -> u128;
        /// Returns the number of messages received from chain `chain_name`
        #[ink(message)]
        fn get_received_message_number(& self, chain_name: String) -> u128;
        /// Returns the message with id `id` sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message(& self, chain_name: String, id: u128) -> Result<SentMessage, Error>;
        /// Returns the message with id `id` received from chain `chain_name`
        #[ink(message)]
        fn get_received_message(& self, chain_name: String, id: u128) -> Result<ReceivedMessage, Error>;
        /// Registers external callable interface information
        #[ink(message)]
        fn register_interface(&mut self, action: String, interface: String);
        /// Returns interface information of contract `contract` and action `action`
        #[ink(message)]
        fn get_interface(& self, contract: AccountId, action: String) -> Result<String, Error>;
    }

    /// Trait for multi porters
    #[ink::trait_definition]
    pub trait MultiPorters {
        /// Changes porters and requirement.
        #[ink(message)]
        fn change_porters_and_requirement(&mut self, porters: Porters, requirement: u16);
        /// Get porters.
        #[ink(message)]
        fn get_porters(& self) -> Porters;
        /// Get requirement
        #[ink(message)]
        fn get_requirement(& self) -> u16;
    }

    /// Defines the wrapper for cross-chain data
    struct Wrapper {
        data: ink_prelude::vec::Vec::<u8>,
    }
    
    impl Wrapper {
        pub fn new(data: ink_prelude::vec::Vec::<u8>) -> Self {
            Self {
                data,
            }
        }
    }
    
    impl scale::Encode for Wrapper {
        #[inline]
        fn size_hint(&self) -> usize {
            scale::Encode::size_hint(&self.data)
        }
    
        #[inline]
        fn encode_to<O: scale::Output + ?Sized>(&self, output: &mut O) {
            output.write(&self.data);
        }
    }

    // use serde_json::json;
    // use serde_json_wasm::{from_str, to_string};
    
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
        /// Map for interfaces
        interfaces: Mapping<(AccountId, String), String>,
        /// Dante token contract
        /// Table of sent messages
        sent_message_table: Mapping<String, Vec<SentMessage>>,
        /// Table of received messages
        received_message_table: Mapping<String, Vec<ReceivedMessage>>,
        /// Context of a cross-contract call
        context: Option<Context>,

        // Data for MultiPorters
        /// Required number of porters.
        required: u16,
        /// Mapping of porters.
        is_porter: Mapping<AccountId, bool>,
        /// List of porters.
        porters: Vec<AccountId>,
    }

    impl CrossChain {
        /// Constructor that initializes `chain_name`.
        #[ink(constructor)]
        pub fn new(chain_name: String) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, chain_name)
            })
        }

        /// Initializes the contract with the specified chain name.
        fn new_init(&mut self, chain_name: String) {
            let caller = Self::env().caller();
            self.owner = Some(caller);
            self.chain_name = chain_name;
        }

        /// If caller is the owner of the contract
        fn only_owner(& self) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.owner.unwrap() != caller {
                return Err(Error::NotOwner);
            }

            Ok(())
        }

        /// Receives message
        fn internal_receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String, contract: AccountId,
            sqos: SQOS, action: String, data: MessagePayload, session: Session) -> Result<(), Error> {
            let mut chain_message = self.received_message_table.get(&from_chain).unwrap_or(Vec::<ReceivedMessage>::new());
            let current_id = chain_message.len() + 1;
            if current_id != id.try_into().unwrap() {
                return Err(Error::IdNotMatch)
            }

            let message = ReceivedMessage::new(id, from_chain.clone(), sender, signer, sqos, contract, action, data, session);
            chain_message.push(message);
            self.received_message_table.insert(from_chain, &chain_message);
            Ok(())
        }

        /// Abandons message
        fn internal_abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) -> Result<(), Error> {
            let mut chain_message = self.received_message_table.get(&from_chain).unwrap_or(Vec::<ReceivedMessage>::new());
            let current_id = chain_message.len() + 1;
            if current_id != (id as usize) {
                return Err(Error::IdNotMatch)
            }

            let message = ReceivedMessage::new_with_error(id, from_chain.clone(), error_code);
            chain_message.push(message);
            self.received_message_table.insert(from_chain, &chain_message);
            Ok(())
        }
    }

    impl Ownable for CrossChain {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(& self) -> Option<AccountId> {
            self.owner
        }

        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) {
            self.owner = None;
        }

        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) {
            self.owner = Some(new_owner);
        }
    }

    impl CrossChainBase for CrossChain {
        /// Sets DAT token contract address
        #[ink(message)]
        fn set_token_contract(&mut self, _token: AccountId) {

        }

        /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
        #[ink(message)]
        fn send_message(&mut self, to_chain: String, contract: String, action: String, sqos: SQOS, data: MessagePayload, session: Session) {
            let mut chain_message: Vec<SentMessage> = self.sent_message_table.get(&to_chain).unwrap_or(Vec::<SentMessage>::new());
            let id = chain_message.len() + 1;
            let caller = Self::env().caller();
            let signer = caller.clone();
            let content = Content::new(contract, action, data);
            let message: SentMessage = SentMessage::new(id.try_into().unwrap(), self.chain_name.clone(), to_chain.clone(), caller, signer, sqos, content, session);
            chain_message.push(message);
            self.sent_message_table.insert(to_chain, &chain_message);
        }

        /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String,
            sqos: SQOS, contract: AccountId, action: String, data: MessagePayload, session: Session) {
            self.internal_receive_message(from_chain, id, sender, signer, contract, sqos, action, data, session);
        }

        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        #[ink(message)]
        fn abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) -> Result<(), Error> {
            self.only_owner()?;

            self.internal_abandon_message(from_chain, id, error_code);

            Ok(())
        }
        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128) -> Result<(), Error> {
            let chain_message: Vec<ReceivedMessage> = self.received_message_table.get(&chain_name).ok_or(Error::ChainMessageNotFound)?;
            let message: &ReceivedMessage = chain_message.get(usize::try_from(id - 1).unwrap()).ok_or(Error::IdOutOfBound)?;

            if message.executed {
                return Err(Error::AlreadyExecuted);
            }

            self.context = Some(Context::new(message.id, message.from_chain.clone(), message.sender.clone(), message.signer.clone(),
                message.contract.clone(), message.action.clone()));

            // Cross-contract call
            Ok(())
        }

        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(& self) -> Option<Context> {
            self.context.clone()
        }

        /// Returns the number of messages sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message_number(& self, chain_name: String) -> u128 {
            let chain_message: Option<Vec<SentMessage>> = self.sent_message_table.get(chain_name);
            if chain_message.is_none() {
                return 0;
            }
            chain_message.unwrap().len().try_into().unwrap()
        }

        /// Returns the number of messages received from chain `chain_name`
        #[ink(message)]
        fn get_received_message_number(& self, chain_name: String) -> u128 {
            let chain_message: Option<Vec<ReceivedMessage>> = self.received_message_table.get(&chain_name);
            if chain_message.is_none() {
                return 0;
            }
            chain_message.unwrap().len().try_into().unwrap()
        }

        /// Returns the message with id `id` sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message(& self, chain_name: String, id: u128) -> Result<SentMessage, Error> {
            let chain_message: Vec<SentMessage> = self.sent_message_table.get(chain_name).ok_or(Error::ChainMessageNotFound)?;
            let message: &SentMessage = chain_message.get(usize::try_from(id - 1).unwrap()).ok_or(Error::IdOutOfBound)?;
            Ok(message.clone())
        }

        /// Returns the message with id `id` received from chain `chain_name`
        #[ink(message)]
        fn get_received_message(& self, chain_name: String, id: u128) -> Result<ReceivedMessage, Error> {
            let chain_message: Vec<ReceivedMessage> = self.received_message_table.get(&chain_name).ok_or(Error::ChainMessageNotFound)?;
            let message: &ReceivedMessage = chain_message.get(usize::try_from(id - 1).unwrap()).ok_or(Error::IdOutOfBound)?;
            Ok(message.clone())
        }

        /// Registers external callable interface information
        #[ink(message)]
        fn register_interface(&mut self, action: String, interface: String) {
            let caller = self.env().caller();
            self.interfaces.insert((caller, action), &interface);
        }

        #[ink(message)]
        fn get_interface(& self, contract: AccountId, action: String) -> Result<String, Error> {
            let interface = self.interfaces.get((contract, action)).ok_or(Error::InterfaceNotFound)?;
            Ok(interface)
        }
    }

    impl MultiPorters for CrossChain {
        #[ink(message)]
        fn change_porters_and_requirement(&mut self, porters: Porters, requirement: u16) {
            // Clear porters
            for i in &self.porters {
                self.is_porter.remove(i);
            }

            // self.porters.resize(porters.len(), AccountId::default());
            for i in &porters {
                self.is_porter.insert(i, &true);
            }

            self.porters = porters;
            self.required = requirement;
        }

        /// Get porters.
        #[ink(message)]
        fn get_porters(& self) -> Porters {
            self.porters.clone()
        }

        /// Get requirement
        #[ink(message)]
        fn get_requirement(& self) -> u16 {
            self.required
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }

        fn create_contract_with_received_message() -> CrossChain {
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Receive message.
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let sender = "SENDER".to_string();
            let signer = "SIGNER".to_string();
            let contract = AccountId::default();
            let action = "ETHERERUM_ACTION".to_string();
            let sqos = SQOS::new(0);
            let data = MessagePayload::new();
            let session = Session::new(0, 0);
            cross_chain.receive_message(from_chain.clone(), id, sender, signer, sqos, contract, action, data, session);
            cross_chain
        }

        fn create_contract_with_sent_message() -> CrossChain {
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Send message.
            let to_chain = "ETHEREUM".to_string();
            let contract = "ETHEREUM_CONTRACT".to_string();
            let action = "ETHERERUM_ACTION".to_string();
            let sqos = SQOS::new(0);
            let data = MessagePayload::new();
            let session = Session::new(0, 0);
            cross_chain.send_message(to_chain.clone(), contract, action, sqos, data, session);
            cross_chain
        }

        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let cross_chain = CrossChain::new("POLKADOT".to_string());
        }

        /// Tests for trait Ownable
        #[ink::test]
        fn owner_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            set_caller(accounts.bob);
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Owner should be Bob.
            assert_eq!(cross_chain.owner().unwrap(), accounts.bob);
        }

        #[ink::test]
        fn renounce_ownership_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Renounce ownership.
            cross_chain.renounce_ownership();
            // Owner is None.
            assert_eq!(cross_chain.owner(), None);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Transfer ownership.
            cross_chain.transfer_ownership(accounts.bob);
            // Owner is Bob.
            assert_eq!(cross_chain.owner().unwrap(), accounts.bob);
        }

        #[ink::test]
        fn only_owner_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Call of only_owner should return Ok.
            set_caller(accounts.alice);
            assert_eq!(cross_chain.only_owner(), Ok(()));
        }

        #[ink::test]
        fn not_owner_only_owner_should_fail() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Call of only_owner should return Err.
            set_caller(accounts.bob);
            assert_eq!(cross_chain.only_owner(), Err(Error::NotOwner));
        }

        /// Tests for CrossChainBase
        #[ink::test]
        fn send_message_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Send message.
            let to_chain = "ETHEREUM".to_string();
            let contract = "ETHEREUM_CONTRACT".to_string();
            let action = "ETHERERUM_ACTION".to_string();
            let sqos = SQOS::new(0);
            let data = MessagePayload::new();
            let session = Session::new(0, 0);
            cross_chain.send_message(to_chain.clone(), contract, action, sqos, data, session);
            // Number of sent messages is 1.
            let num = cross_chain.sent_message_table.get(&to_chain).unwrap().len();
            assert_eq!(num, 1);
        }
        
        #[ink::test]
        fn receive_message_works() {
            let from_chain = "ETHEREUM".to_string();
            let cross_chain = create_contract_with_received_message();
            // Number of sent messages is 1.
            let num = cross_chain.received_message_table.get(&from_chain).unwrap().len();
            assert_eq!(num, 1);
        }
        
        #[ink::test]
        fn abandon_message_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Receive message.
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let error_code = 1;
            cross_chain.abandon_message(from_chain.clone(), id, error_code);
            // Number of sent messages is 1.
            let num = cross_chain.received_message_table.get(&from_chain).unwrap().len();
            assert_eq!(num, 1);
        }
        
        #[ink::test]
        fn execute_message_works() {
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_received_message();
            // Execute message
            let ret = cross_chain.execute_message(from_chain.clone(), id);
            assert_eq!(ret, Ok(()));
        }
        
        #[ink::test]
        fn get_context_works() {
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_received_message();
            // Execute message
            let ret = cross_chain.execute_message(from_chain.clone(), id);
            assert_eq!(ret, Ok(()));
            // Context not None.
            let context = cross_chain.get_context();
            assert_eq!(context.is_some(), true);
        }
        
        #[ink::test]
        fn get_sent_message_number_works() {
            let to_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_sent_message();
            // Number of sent messages is 1.
            let num = cross_chain.get_sent_message_number(to_chain);
            assert_eq!(num, 1);
        }
        
        #[ink::test]
        fn get_received_message_number_works() {
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_received_message();
            // Number of received messages is 1.
            let num = cross_chain.get_received_message_number(from_chain);
            assert_eq!(num, 1);
        }

        #[ink::test]
        fn get_sent_message_works() {
            let to_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_sent_message();
            // Sent message is Ok.
            let message = cross_chain.get_sent_message(to_chain, 1);
            assert_eq!(message.is_ok(), true);
        }

        #[ink::test]
        fn get_received_message_works() {
            let from_chain = "ETHEREUM".to_string();
            let id = 1;
            let mut cross_chain = create_contract_with_received_message();
            // Received message is Ok.
            let message = cross_chain.get_received_message(from_chain, 1);
            assert_eq!(message.is_ok(), true);
        }

        #[ink::test]
        fn register_interface_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Received message is Ok.
            let action = "ETHERERUM_ACTION".to_string();
            let interface = "INTERFACE".to_string();
            cross_chain.register_interface(action.clone(), interface);
            // Check registered interface.
            let i = cross_chain.get_interface(accounts.alice, action);
            assert_eq!(i.is_ok(), true);
        }
        // change_porters_and_requirement(&mut self, porters: Porters, requirement: u16);
        // Tests for trait MultiPorters
        #[ink::test]
        fn change_porters_and_requirement_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Resister.
            let mut porters = Porters::new();
            porters.push(accounts.alice);
            porters.push(accounts.bob);
            let required = 2;
            cross_chain.change_porters_and_requirement(porters.clone(), required);
            // Requirement is 2.
            let r = cross_chain.get_requirement();
            assert_eq!(r, 2);
            // Check porters.
            let p = cross_chain.get_porters();
            assert_eq!(p, porters);
        }
    }
}
