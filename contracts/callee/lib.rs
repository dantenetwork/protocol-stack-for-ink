#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod callee {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: ink_prelude::string::String,
        age: u32,
        phones: ink_prelude::vec::Vec<ink_prelude::string::String>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Callee {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Callee {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
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

        /// test cross contract call
        #[ink(message)]
        pub fn encode_user_defined_struct(&self, msg: MessageDetail) -> ink_prelude::string::String{
            ink_prelude::format!("{:?}", msg)
        }

        /// test cross contract call
        #[ink(message)]
        pub fn encode_user_multi_params(&self, msg: MessageDetail, p_str: ink_prelude::string::String, p_int: u32) -> ink_prelude::string::String{
            ink_prelude::format!("{}, {}, {:?}", p_int, p_str, msg)
        }

        /// test encoding user defined struct to u8 
        #[ink(message)]
        pub fn encode_uds(&self, msg: MessageDetail) -> ink_prelude::vec::Vec<u8>{
            let s = ink_prelude::format!("{{ name: {}, age: {}, phones: [] }}", msg.name, msg.age);
            s.into_bytes()
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
            let callee = Callee::default();
            assert_eq!(callee.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut callee = Callee::new(false);
            assert_eq!(callee.get(), false);
            callee.flip();
            assert_eq!(callee.get(), true);
        }
    }
}
