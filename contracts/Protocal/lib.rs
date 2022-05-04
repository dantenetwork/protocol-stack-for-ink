#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;


#[ink::contract]
mod d_protocol_stack {

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
    pub struct DProtocalStack {
        /// Stores a single `bool` value on the storage.
        value: bool,
        account: AccountId,
    }

    impl DProtocalStack {
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
        pub fn call_to_contracts(&self, callee_account: AccountId, msg: MessageDetail) -> ink_prelude::string::String{
            let my_return_value: MessageDetail =  ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(callee_account)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0xa9, 0x45, 0xce, 0xc7]))
                    .push_arg(msg)
                )
                .returns::<MessageDetail>()
                .fire()
                .unwrap();
            ink_prelude::format!("{:?}", my_return_value)
            // my_return_value
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
    }
}
