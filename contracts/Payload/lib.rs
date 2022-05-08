#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod Payload {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: ink_prelude::string::String,
        age: u32,
        phones: ink_prelude::vec::Vec<ink_prelude::string::String>,
        info: Option<ink_prelude::string::String>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Payload {
        /// Stores a single `bool` value on the storage.
        value: bool,
        info: Option<ink_prelude::string::String>,
    }

    impl Payload {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
                info: None,
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
            let Payload = Payload::default();
            assert_eq!(Payload.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut Payload = Payload::new(false);
            assert_eq!(Payload.get(), false);
            Payload.flip();
            assert_eq!(Payload.get(), true);
        }

        #[ink::test]
        fn test_encode_decode() {
            let msg = MessageDetail{
                name: "Nika".into(),
                age: 37,
                phones: ink_prelude::vec!["123".into(), "456".into()],
                info: None,
            };

            let mut v: ink_prelude::vec::Vec::<u8> = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);
            let mut vv = v.as_slice();
            let vout: MessageDetail = scale::Decode::decode(&mut vv).unwrap();
            println!("{:?}", vout);
            assert_eq!(msg, vout);
        }
    }
}
