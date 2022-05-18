#![cfg_attr(not(feature = "std"), no_std)]

pub use self::Payload::{
    MsgType,
    MessageItem,
    MessageVec,
    MessagePayload,
    Payload as Other,
    PayloadRef,
};

use ink_lang as ink;
use ink_prelude;
use ink_storage;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct TestData{
    pub n: u128,
    pub s: ink_prelude::string::String,
}

impl ::scale_info::TypeInfo for TestData{
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
            .path(::scale_info::Path::new("TestData", module_path!()))
            .composite(::scale_info::build::Fields::named()
                .field(|f| f.ty::<u128>().name("n").type_name("u128"))
                .field(|f| f.ty::<ink_prelude::string::String>().name("s").type_name("ink_prelude::string::String"))
            )
    }
}

#[ink::contract]
mod Payload {

    use ink_storage::traits::{SpreadAllocate};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum MsgType{
        InkString,
        InkU8,
        InkU16,
        UserData,
    }

    #[derive(Debug, Eq, scale::Encode, scale::Decode, Clone)]
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

    #[derive(Debug, Eq, scale::Encode, scale::Decode, Clone)]
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
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
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

        /// for `item`
        pub fn add_item(&mut self, msg_item: MessageItem)-> bool{
            if let Some(item) = &mut self.items {
                if item.contains(&msg_item){
                    return false;
                }

                item.push(msg_item);
                true
            } else{
                let mut item_vec = ink_prelude::vec::Vec::new();
                item_vec.push(msg_item);
                self.items = Some(item_vec);
                true
            }
        }

        pub fn get_item(&self, msg_n: u128) -> Option<&MessageItem>{
            if let Some(item) = &self.items {
                for it in item.iter() {
                    if it.n == msg_n {
                        return Some(it);
                    }
                }
            }

            None
        }

        /// for `vecs`
        pub fn add_vec(&mut self, msg_vec: MessageVec) -> bool{
            if let Some(m_vec) = &mut self.vecs {
                if m_vec.contains(&msg_vec){
                    return false;
                }
                
                m_vec.push(msg_vec);
                true
            } else {
                let mut vec_one = ink_prelude::vec::Vec::new();
                vec_one.push(msg_vec);
                self.vecs = Some(vec_one);
                true
            }
        }

        pub fn get_vec(&self, msg_n: u128) -> Option<&MessageVec>{
            if let Some(m_vec) = &self.vecs {
                for it in m_vec.iter() {
                    if it.n == msg_n {
                        return Some(it);
                    }
                }
            }

            None
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
    #[derive(SpreadAllocate, ::scale_info::TypeInfo)]
    pub struct Payload {
        /// Stores a single `bool` value on the storage.
        value: bool,
        info: Option<ink_prelude::string::String>,
        // items: ink_storage::Mapping<ink_prelude::string::String, MessagePayload>,
        // mp: ink_storage::Mapping<u8, MessagePayload>,
        // msg: MessageDetail,
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
        pub fn test_callee_received(&self, m_payload: MessagePayload) ->ink_prelude::string::String{
            let mut s = ink_prelude::string::String::new();

            // `1` is user defined `MessageItem` id
            // In this example, we use user defined data struct `MessageDetail`
            if let Some(item) = m_payload.get_item(1) {
                let mut ss = item.v.as_slice();
                let msg_data: MessageDetail = scale::Decode::decode(&mut ss).unwrap();
                s = s + &ink_prelude::format!("{:?}", msg_data);
                s = s + "\n";
            }

            if let Some(m_vec) = m_payload.get_vec(11) {
                for vec_item in m_vec.v.iter() {
                    let mut ss = vec_item.as_slice();
                    let msg_data: MessageDetail = scale::Decode::decode(&mut ss).unwrap();
                    s = s + &ink_prelude::format!("{:?}", msg_data);
                    s = s + "\n";
                }
            }

            s
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

            let mut m_hash_map: ink_prelude::collections::HashMap<u32, u32> = ink_prelude::collections::HashMap::from_iter(m_hm.clone());

            assert_eq!(*m_hm, [(1, 1), (2, 2)]);

        }

        /// test encode and decode
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

        /// test encode and decode of user defined contract interface
        #[ink::test]
        fn test_contract_en_de() {
            let msg = MessageDetail{
                name: "Nika".into(),
                age: 37,
                phones: ink_prelude::vec!["123".into(), "456".into()],
                info: None,
            };

            let rst_s = ink_prelude::format!("{:?}", msg) + "\n" + &ink_prelude::format!("{:?}", msg) + "\n" + &ink_prelude::format!("{:?}", msg) + "\n";

            let mut v: ink_prelude::vec::Vec::<u8> = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);

            let mut msg_payload = MessagePayload::new();
            let msg_item = MessageItem{
                n: 1,
                t: MsgType::UserData,
                v: v.clone(),
            };
            assert_eq!(msg_payload.add_item(msg_item), true);

            let mut vec_eles: ink_prelude::vec::Vec<ink_prelude::vec::Vec<u8>> = ink_prelude::vec::Vec::new();
            vec_eles.push(v.clone());
            vec_eles.push(v.clone());

            let msg_vec = MessageVec{
                n: 11,
                t: MsgType::UserData,
                v: vec_eles,
            };
            assert_eq!(msg_payload.add_vec(msg_vec), true);
            
            // simulate encode `MessagePayload` from routers(off-chain js)
            let mut pl_code: ink_prelude::vec::Vec::<u8> = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg_payload, &mut pl_code);

            // simulate decode `MessagePayload` implemented underlying
            let mut vv = pl_code.as_slice();
            let vout: MessagePayload = scale::Decode::decode(&mut vv).unwrap();

            // simulate contract call
            let payload = Payload::default();
            let return_s = payload.test_callee_received(vout);

            assert_eq!(return_s, rst_s);
        }
    }
}
