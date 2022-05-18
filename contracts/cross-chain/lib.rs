#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod cross_chain {
    use ink_lang as ink;    
    use ink_storage::{
        traits::{
            SpreadLayout,
            SpreadAllocate,
            StorageLayout,
        },
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
    
    type Bytes = Vec<u8>;
    type Porters = Vec<String>;

    /// Errors for cross-chain contract
    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        IdNotMatch,
    }

    /// Received message structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct ReceivedMessage {
        id: u128,
        from_chain: String,
        sender: String,
        signer: String,
        sqos: SQOS,
        contract: AccountId,
        action: String,
        data: Bytes,
        session: Session,
        executed: bool,
        error_code: u16,
    }

    impl ReceivedMessage {
        fn new(id: u128, from_chain: String, sender: String, signer: String, sqos: SQOS,
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
            }
        }
    }

    /// Content structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct Content {
        contract: String,
        action: String,
        data: Bytes,
    }

    impl Content {
        fn new(contract: String, action: String, data: Bytes) -> Self {
            Self {
                contract,
                action,
                data,
            }
        }
    }

    /// Sent message structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct SentMessage {
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
        fn new(id: u128, from_chain: String, to_chain: String, sender: AccountId, signer: AccountId,
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
    }

    /// Context structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct Context {
        id: u128,
        from_chain: String,
        sender: String,
        signer: String,
        contract: String,
        action: String,
    }

    /// SQOS structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct SQOS {
        reveal: u8,
    }

    /// Session Structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct Session {
        msg_type: u8,
        id: u128,
    }

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
    trait CrossChainBase {
        /// Sets DAT token contract address
        #[ink(message)]
        fn set_token_contract(&mut self, token: AccountId);
        /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
        #[ink(message)]
        fn send_message(&mut self, to_chain: String, contract: String, action: String, data: &Bytes);
        /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String, to: AccountId, action: String, data: Bytes);
        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        #[ink(message)]
        fn abandon_message(&mut self, from_chain: String, id: u128, error_code: u16);
        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128);
        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(& self, ) -> Context;
        /// Returns the number of messages sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message_number(& self, chain_name: String) -> u128;
        /// Returns the number of messages received from chain `chain_name`
        #[ink(message)]
        fn get_received_message_number(& self, chain_name: String) -> u128;
        /// Returns the message with id `id` sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message(& self, chain_name: String, id: u128) -> SentMessage;
        /// Returns the message with id `id` received from chain `chain_name`
        #[ink(message)]
        fn get_received_message(& self, chain_name: String, id: u128) -> ReceivedMessage;
        /// Registers external callable interface information
        #[ink(message)]
        fn register_interface(&mut self, action: String, interface: String);
    }

    /// Trait for multi porters
    trait MultiPorters {
        /// Changes porters and requirement
        fn change_porters_and_requirement(porters: Porters, requirement: u128);
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
        interfaces: Mapping<AccountId, Mapping<String, String>>,
        /// Dante token contract
        /// Table of sent messages
        sent_message_table: Mapping<String, Vec<SentMessage>>,
        /// Table of received messages
        received_message_table: Mapping<String, Vec<ReceivedMessage>>,
        /// Context of a cross-contract call
        context: Context,
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

        /// Restricted to owner only
        fn only_owner(& self) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.owner.unwrap() != caller {
                return Err(Error::NotOwner)
            }

            Ok(())
        }

        /// Receives message
        fn internal_receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String, contract: AccountId
            sqos: SQOS, action: String, data: Bytes, session: Session) -> Result<(), Err> {
            let mut chain_message = self.received_message_table.get(from_chain).unwrap_or(Vec::<ReceivedMessage>::new());
            let current_id = chain_message.len() + 1;
            if current_id != id {
                return Err(Error::IdNotMatch)
            }

            let message = ReceivedMessage::new(id, from_chain, sender, signer, sqos, contract, action, data, session);
            chain_message.push(message);
            self.received_message_table.insert(from_chain, chain_message);
        }

        /// Abandons message
        fn internal_abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) -> Result<(), Err> {
            
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
        fn set_token_contract(&mut self, token: AccountId) {

        }

        /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
        #[ink(message)]
        fn send_message(&mut self, to_chain: String, contract: String, action: String, data: &Bytes) {
            let mut chain_message: Vec<SentMessage> = self.sent_message_table.get(to_chain).unwrap_or(Vec::<SentMessage>::new());
            let id = chain_message.len() + 1;
            let caller = Self::env().caller();
            let signer = caller.clone();
            let content = Content::new(contract, action, data);
            let mut message: SentMessage = SentMessage::new(id, self.chain_name, to_chain, caller, signer, content);
            chain_message.push(message);
            self.sent_message_table.insert(to_chain, chain_message);
        }

        /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
        #[ink(message)]
        fn receive_message(&mut self, from_chain: String, id: u128, sender: String, signer: String,
            sqos: SQOS, contract: AccountId, action: String, data: Bytes, session: Session) {
            internal_receive_message(from_chain, id, sender, signer, sqos, contract, action, data, session);
        }

        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        #[ink(message)]
        fn abandon_message(&mut self, from_chain: String, id: u128, error_code: u16) {

        }
        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        #[ink(message)]
        fn execute_message(&mut self, chain_name: String, id: u128) {

        }
        /// Returns the simplified message, this message is reset every time when a contract is called
        #[ink(message)]
        fn get_context(& self, ) -> Context {
            self.context
        }
        /// Returns the number of messages sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message_number(& self, chain_name: String) -> u128 {
            0
        }
        /// Returns the number of messages received from chain `chain_name`
        #[ink(message)]
        fn get_received_message_number(& self, chain_name: String) -> u128 {
            0
        }
        /// Returns the message with id `id` sent to chain `chain_name`
        #[ink(message)]
        fn get_sent_message(& self, chain_name: String, id: u128) -> SentMessage {

        }
        /// Returns the message with id `id` received from chain `chain_name`
        #[ink(message)]
        fn get_received_message(& self, chain_name: String, id: u128) -> ReceivedMessage;
        /// Registers external callable interface information
        #[ink(message)]
        fn register_interface(&mut self, action: String, interface: String);
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

        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let cross_chain = CrossChain::new("POLKADOT".to_string());
        }

        /// For trait Ownable
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
            // Call of only_owner should failed.
            set_caller(accounts.alice);
            assert_eq!(cross_chain.only_owner(), Ok(()));
        }

        #[ink::test]
        fn not_owner_only_owner_should_fail() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut cross_chain = CrossChain::new("POLKADOT".to_string());
            // Call of only_owner should failed.
            set_caller(accounts.bob);
            assert_eq!(cross_chain.only_owner(), Err(Error::NotOwner));
        }
    }
}
