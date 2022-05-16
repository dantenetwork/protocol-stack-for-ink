#![cfg_attr(not(feature = "std"), no_std)]

pub use self::Payload::{
    MsgType,
    MessageItem,
    MessageVec,
    MessagePayload,
};

use ink_lang as ink;
use ink_prelude;
use ink_storage;

#[ink::contract]
mod Payload {

    use ink_storage::traits::SpreadAllocate;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum MsgType{
        InkString,
        InkU8,
        InkU16,
        UserData,
    }

    #[derive(Debug, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageItem{
        pub n: u128,
        pub t: MsgType,
        pub v: ink_prelude::vec::Vec<u8>,
    }

    impl PartialEq for MessageItem {
        fn eq(&self, other: &MessageItem) -> bool{
            return self.n == other.n;
        }
    }

    #[derive(Debug, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageVec{
        pub n: u128,
        pub t: MsgType,
        pub v: ink_prelude::vec::Vec<ink_prelude::vec::Vec<u8>>,
    }

    impl PartialEq for MessageVec {
        fn eq(&self, other: &MessageVec) -> bool{
            return self.n == other.n;
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessagePayload{
        pub items: Option<ink_prelude::vec::Vec<MessageItem>>,
        pub vecs: Option<ink_prelude::vec::Vec<MessageVec>>,
    }

    impl MessagePayload{
        pub fn new() -> MessagePayload{
            MessagePayload {
                items: None,
                vecs: None,
            }
        }

        pub fn add_item(&mut self, msg_item: MessageItem){
            if let Some(item) = &mut self.items {
                item.push(msg_item);
            } else{
                let mut item_vec = ink_prelude::vec::Vec::new();
                item_vec.push(msg_item);
                self.items = Some(item_vec);
            }
        }

        pub fn add_vec(&mut self, msg_vec: MessageVec){
            if let Some(m_vec) = &mut self.vecs {
                m_vec.push(msg_vec);
            } else {
                let mut vec_one = ink_prelude::vec::Vec::new();
                vec_one.push(msg_vec);
                self.vecs = Some(vec_one);
            }
        }
    }

    // for test
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
    #[derive(SpreadAllocate)]
    pub struct Payload {
        /// Stores a single `bool` value on the storage.
        value: bool,
        info: Option<ink_prelude::string::String>,
        items: ink_storage::Mapping<ink_prelude::string::String, u128>,
    }

    impl Payload {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                contract.info = None;
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
            // Self::new(Default::default())
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

        /// Test the message Type.
        #[ink(message)]
        pub fn getMessage(&self, msg: MessagePayload) -> MessagePayload {
            msg
        }

        /// User defined behaviors when messages or invocations are received from other chains
        #[ink(message)]
        pub fn test_callee_received(&self, m_payload: MessagePayload, m_hm: ink_prelude::vec::Vec<(u32, u32)>) ->ink_prelude::string::String{
            
            let mut m_hash_map: ink_prelude::collections::HashMap<u32, u32> = ink_prelude::collections::HashMap::from_iter(m_hm);
            
            ink_prelude::string::String::new()
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
            let mut m_hm: ink_prelude::vec::Vec<(u32, u32)> = ink_prelude::vec::Vec::new();
            m_hm.push((1, 1));
            m_hm.push((2, 2));

            assert_eq!(*m_hm, [(1, 1), (2, 2)]);

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
            assert_eq!(Some(msg), Some(vout));
        }
    }
}
