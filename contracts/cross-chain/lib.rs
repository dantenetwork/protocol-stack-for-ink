#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod cross_chain {
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
    
    type Bytes = Vec<u8>;
    type Porters = Vec<String>;

    /// Received message structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct ReceivedMessage {
        id: u128,
        from_chain: String,
        sender: String,
        signer: String,
        contract: AccountId,
        action: String,
        data: Bytes,
        executed: bool,
        error_code: u32,
    }

    /// Content structure
    #[derive(SpreadAllocate, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct Content {
        contract: String,
        action: String,
        data: Bytes,
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
        content: Content,
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

    /// Trait for owner
    trait Ownable {
        /// Returns the account id of the current owner
        fn owner() -> AccountId;
        /// Renounces ownership of the contract
        fn renounce_ownership();
        /// Transfer ownership to a new account id
        fn transfer_ownership(new_owner: AccountId);
    }

    /// Trait for basic cross-chain contract
    trait CrossChainBase {
        /// Sets DAT token contract address
        fn set_token_contract(token: AccountId);
        /// Cross-chain calls method `action` of contract `contract` on chain `to_chain` with data `data`
        fn send_message(to_chain: String, contract: String, action: String, data: &Bytes);
        /// Cross-chain receives message from chain `from_chain`, the message will be handled by method `action` of contract `to` with data `data`
        fn receive_message(from_chain: String, id: u128, sender: String, signer: String, to: AccountId, action: String, data: Bytes);
        /// Cross-chain abandons message from chain `from_chain`, the message will be skipped and not be executed
        fn abandon_message(from_chain: String, id: u128, error_code: u32);
        /// Triggers execution of a message sent from chain `chain_name` with id `id`
        fn execute_message(chain_name: String, id: u128);
        /// Returns the simplified message, this message is reset every time when a contract is called
        fn get_context() -> Context;
        /// Returns the number of messages sent to chain `chain_name`
        fn get_sent_message_number(chain_name: String) -> u128;
        /// Returns the number of messages received from chain `chain_name`
        fn get_received_message_number(chain_name: String) -> u128;
        /// Returns the message with id `id` sent to chain `chain_name`
        fn get_sent_message(chain_name: String, id: u128) -> SentMessage;
        /// Returns the message with id `id` received from chain `chain_name`
        fn get_received_message(chain_name: String, id: u128) -> ReceivedMessage;
        /// Registers external callable interface information
        fn register_interface(action: String, interface: String);
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

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: ink_prelude::string::String,
        age: u32,
        phones: ink_prelude::vec::Vec<ink_prelude::string::String>,
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
        owner: AccountId,

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
            self.owner = caller;
            self.chain_name = chain_name;
        }

        /// Interface for Sending information from Polkadot
        #[ink(message)]
        pub fn send_message(&mut self){

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
    }
}
