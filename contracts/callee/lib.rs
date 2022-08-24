#![cfg_attr(not(feature = "std"), no_std)]

mod test;

use ink_lang as ink;
use ink_prelude;

use payload::message_protocol::{ MessagePayload, MessageItem, MsgDetail};
use payload::message_define::{ISentMessage, IReceivedMessage};

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct MyData {
    td: payload::TestData,
}

#[ink::contract]
mod callee {

    use payload::message_protocol::InMsgType;

    #[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: ink_prelude::string::String,
        age: u32,
        phones: ink_prelude::vec::Vec<ink_prelude::string::String>,
    }

    /// This is an example to impl `payload::message_protocol::InMsgType` for a user defined struct, 
    /// such that `MessageDetail` can be read directly through `payload::message_protocol::MessageItem::in_to::<MessageDetail>()`
    impl InMsgType for MessageDetail {
        type MyType = MessageDetail;
        fn get_value(type_value: & super::MsgDetail) -> Option<Self::MyType> {
            if let super::MsgDetail::UserData(val) = type_value.clone() {
                let mut v_ref = val.as_slice();
                Some(scale::Decode::decode(&mut v_ref).unwrap())
            } else {
                None
            }
        }

        /// items from traits can only be used if the trait is in scope
        fn create_message(msg_detail: Self::MyType) -> super::MsgDetail {
            let mut v = ink_prelude::vec::Vec::new();
            scale::Encode::encode_to(&msg_detail, &mut v);
            
            super::MsgDetail::UserData(v)
        }

        fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
            let mut raw_data = ink_prelude::vec![];
            
            raw_data.append(&mut ink_prelude::vec::Vec::from(self.name.as_bytes()));
            raw_data.append(&mut ink_prelude::vec::Vec::from(self.age.to_be_bytes()));

            for ele in self.phones.iter() {
                raw_data.append(&mut ink_prelude::vec::Vec::from(ele.as_bytes()));
            }

            raw_data
        }
    }

    /// event
    #[ink(event)]
    pub struct EventRecv2 {
        triggered: bool,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Callee {
        /// Stores a single `bool` value on the storage.
        message: u32,
    }

    impl Callee {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                message: 0,
            }
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
        pub fn test_ud_en_de(&self, msg: MessageDetail) -> Option<MessageDetail> {
            let mut v = ink_prelude::vec::Vec::new();
            scale::Encode::encode_to(&msg, &mut v);

            let msg_item = super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                    super::MsgDetail::UserData(v));

            msg_item.in_to::<MessageDetail>()
        }

        #[ink(message)]
        pub fn test_ud_en_de_other(&self, msg: MessageDetail) -> Option<MessageDetail> {
            let msg_vec = super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                    MessageDetail::create_message(msg));

            msg_vec.in_to::<MessageDetail>()
        }

        /// test corss contract call
        #[ink(message)]
        pub fn send_message(&mut self, addr1: AccountId, addr2: AccountId, m: u32) {
            self.flush();

            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(addr1)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    // call `receive_message` of contract `addr1`
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x3a, 0x6e, 0x96, 0x96]))
                    .push_arg(addr2)
                    .push_arg(m)
                )
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .returns::<()>()
                .fire().
                unwrap();

            self.load();
        }

        #[ink(message)]
        pub fn receive_message(&mut self, addr: AccountId, i: u32) {
            // self.flush();

            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(addr)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    // call `update_message` of contract `addr`
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x03, 0x2a, 0x6f, 0x29]))
                    .push_arg(i)
                )
                // .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .returns::<()>()
                .fire().
                unwrap();

            // self.load();
        }

        #[ink(message)]
        pub fn update_message(&mut self, i: u32) {
            Self::env().emit_event(EventRecv2{
                triggered: true,
            });
            self.message = i;
        }

        #[ink(message)]
        pub fn get_message(& self, flag: bool) -> u32 {
            self.message
        }

        /// Method flushes the current state of `Self` into storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to flush the correct state of the contract,
        /// you have to this method on storage struct.
        fn flush(&self) {
            let root_key = ::ink_primitives::Key::from([0x00; 32]);
            ::ink_storage::traits::push_spread_root::<Self>(self, &root_key);
        }

        /// Method loads the current state of `Self` from storage.
        /// ink! recursively calculate a key of each field.
        /// So if you want to load the correct state of the contract,
        /// you have to this method on storage struct.
        fn load(&mut self) {
            let root_key = ::ink_primitives::Key::from([0x00; 32]);
            let mut state = ::ink_storage::traits::pull_spread_root::<Self>(&root_key);
            core::mem::swap(self, &mut state);
            let _ = core::mem::ManuallyDrop::new(state);
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
                tv: super::super::MsgDetail::InkU16(18),
            };

            pl.push_item(ink_prelude::string::String::from("1"), super::super::MsgDetail::InkU16(24));
            msg_item.tv = super::super::MsgDetail::InkU8(255);
            assert_eq!(pl.push_item(ink_prelude::string::String::from("1"), super::super::MsgDetail::InkU16(v_u16)), false);

            // Attention, `assert_eq` use the concrete implementation of `PartialEq` to chack equal
            // So it doesn't matter whether the `t` and `v` is the same
            assert_eq!(pl.get_item(ink_prelude::string::String::from("1")), Some(&msg_item));
        }

        /// test `MessageItem::from`, `MessageItem::into` 
        fn test_from_into(){
            let mut msg_item = super::super::MessageItem::from(ink_prelude::string::String::from("Nika"), 
                                                            super::super::MsgDetail::InkU32(128));

            let num: u32 = msg_item.in_to::<u32>().unwrap();

            assert_eq!(num, 128 as u32);
        }
    }
}
