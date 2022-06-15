#![cfg_attr(not(feature = "std"), no_std)]

mod test;

use ink_lang as ink;
use ink_prelude;

use payload::message_protocol::{ MessagePayload, MessageItem, MsgType};
use payload::message_define::{ISentMessage, IReceivedMessage};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct MyData {
    td: payload::TestData,
}

#[ink::contract]
mod callee {

    #[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
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
            Self { 
                value: init_value, 
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

        #[ink(message)]
        pub fn get_struct_message_u8(& self, msg: MessageDetail) -> ink_prelude::vec::Vec::<u8>{
            let mut v = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);
            v
        }

        #[ink(message)]
        pub fn get_struct_message_vec_u8(& self, msg: MessageDetail) -> ink_prelude::vec::Vec::<u8>{
            let mut v = ink_prelude::vec::Vec::<u8>::new();
            let mut v_vec = ink_prelude::vec![msg.clone(), msg.clone()];
            scale::Encode::encode_to(&v_vec, &mut v);
            v
        }

        // test Payload as parameter
        #[ink(message)]
        pub fn get_payload(&self, msg_vec: super::MessagePayload) -> ink_prelude::string::String{
            ink_prelude::format!("{:?}", msg_vec)
            // let mut vv = msg_vec.as_slice();
            // let vout: Payload::MessagePayload = scale::Decode::decode(&mut vv).unwrap();
        }

        #[ink(message)]
        pub fn get_recv_message(&self, msg: super::IReceivedMessage) -> ink_prelude::string::String{
            ink_prelude::format!("{:?}", msg)
        }

        #[ink(message)]
        pub fn test_ud_en_de(&self, msg: MessageDetail) -> MessageDetail {
            let msg_item = super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                    super::MsgType::InkU32, 
                                                    msg);

            msg_item.in_to()
        }

        #[ink(message)]
        pub fn test_ud_en_de_vec(&self, msg: MessageDetail) -> ink_prelude::vec::Vec<MessageDetail> {
            let msg_vec = super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                    super::MsgType::InkU32, 
                                                    ink_prelude::vec![msg.clone(), msg.clone()]);

            msg_vec.in_to()
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

        /// test `Payload`
        #[ink::test]
        fn test_payload() {

            // let n_s = ink_prelude::string::String::from("Nika");
            // let mut n_vec = ink_prelude::vec::Vec::<u8>::new();
            // scale::Encode::encode_to(&n_s, &mut n_vec);

            let v_u16 : u16 = 99;
            let mut v_vec = ink_prelude:: vec::Vec::<u8>::new();
            scale::Encode::encode_to(&v_u16, &mut v_vec);

            let mut pl = super::super::MessagePayload::new();
            let mut msg_item = super::super::MessageItem{
                n: ink_prelude::string::String::from("1"),
                t: super::super::MsgType::InkU16,
                v: v_vec,
            };

            pl.push_item(ink_prelude::string::String::from("1"), super::super::MsgType::InkU16, v_u16);
            msg_item.t = super::super::MsgType::InkU8;
            assert_eq!(pl.push_item(ink_prelude::string::String::from("1"), super::super::MsgType::InkU16, v_u16), false);
            // msg_item.n = 2;
            msg_item.v.push(100);

            // Attention, `assert_eq` use the concrete implementation of `PartialEq` to chack equal
            // So it doesn't matter whether the `t` and `v` is the same
            assert_eq!(pl.get_item(ink_prelude::string::String::from("1")), Some(&msg_item));
        }

        /// test `MessageItem::from`, `MessageItem::into` 
        fn test_from_into(){
            let mut msg_item = super::super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                            super::super::MsgType::InkU32, 
                                                            128 as u32);

            let num: u32 = msg_item.in_to();

            assert_eq!(num, 128 as u32);
        }
    }
}
