#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod cross_chain {
    type String = ink_prelude::string::String;
    type Bytes = ink_prelude::vec::Vec<u8>;
    type Porters = ink_prelude::vec::Vec<String>;

    /// Received message structure
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
    struct Content {
        contract: String,
        action: String,
        data: Bytes,
    }

    /// Sent message structure
    struct SentMessage {
        id: u128,
        from_chain: String,
        to_chain: String,
        sender: AccountId,
        signer: AccountId,
        content: Content,
    }

    /// Context structure
    struct Context {
        id: uu128,
        from_chain: String,
        sender: String,
        signer: String,
        contract: String,
        action: String,
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
    pub struct CrossChain {
        /// Stores a single `bool` value on the storage.
        value: bool,
        account: AccountId,
    }

    impl CrossChain {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
                account: Self::env().caller(),
             }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Interface for Sending information from Polkadot
        #[ink(message)]
        pub fn send_message(&mut self){

        }

        /// Interface for receiving information from other ecosystem
        /// Submit message from routers
        #[ink(message)]
        pub fn submit_message(& self, msg: MessageDetail) -> MessageDetail{
            msg

            // // Parse the string of data into serde_json::Value.
            // let v: std::result::Result<MessageDetail, serde_json_wasm::de::Error>  = from_str(data);
            
            // // v?.to_string()
            // if let Ok(val) = v {
            //     val.to_string()
            // }else{
            //     "error!".to_string()
            // }
        }

        #[ink(message)]
        pub fn get_struct_message_u8(& self, msg: MessageDetail) -> ink_prelude::vec::Vec::<u8>{
            let mut v = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);
            v
        }
        
        #[ink(message)]
        pub fn call_to_contracts(& self, callee_account: AccountId, msg: ink_prelude::vec::Vec::<u8>) -> ink_prelude::string::String{
            let data: ink_prelude::vec::Vec::<u8> = msg.clone().drain(4..).collect();
            let wrapped_data = Wrapper::new(data);
            
            let my_return_value: ink_prelude::string::String =  ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(callee_account)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([msg[0], msg[1], msg[2], msg[3]]))
                    .push_arg(wrapped_data)
                )
                .returns::<ink_prelude::string::String>()
                .fire()
                .unwrap();
            my_return_value
        }

        /// message verification
        fn message_verification(&mut self){

        }

        /// node evaluation
        fn node_evaluation(&mut self){

        }

        /// node selection
        fn select(&self) {

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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let cross_chain = DProtocalStack::default();
            assert_eq!(cross_chain.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut cross_chain = DProtocalStack::new(false);
            assert_eq!(cross_chain.get(), false);
            cross_chain.flip();
            assert_eq!(cross_chain.get(), true);
        }

        #[ink::test]
        fn test_encode() {
            let mut data = ink_prelude::vec::Vec::<u8>::new();
            data.push(0x10);
            data.push(0x4e);
            data.push(0x69);
            data.push(0x6b);
            data.push(0x61);
            data.push(0x24);
            let wrapped_data = Wrapper::new(data);

            let mut buf = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&wrapped_data, &mut buf);
            assert_eq!(buf, [0x10, 0x4e, 0x69, 0x6b, 0x61, 0x24]);
        }

        #[ink::test]
        fn multi_params() {
            let mut phones = ink_prelude::vec::Vec::<ink_prelude::string::String>::new();
            phones.push(ink_prelude::string::String::from("123"));
            phones.push(ink_prelude::string::String::from("456"));
            let msg_struct = MessageDetail{
                name: ink_prelude::string::String::from("Nika"),
                age: 18,
                phones: phones,
            };

            let exec_input = ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0xa9, 0x45, 0xce, 0xc7]))
                    .push_arg(msg_struct)
                    .push_arg(ink_prelude::string::String::from("hthuang"))
                    .push_arg(666);
            
            let mut buf = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&exec_input, &mut buf);
            println!("{:?}", buf);
            // assert_eq!(buf, [0x10, 0x4e, 0x69, 0x6b, 0x61, 0x24]);
        }
    }
}
