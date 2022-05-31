#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod locker_mock {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use cross_chain::message_define::{
        SentMessage,
        Session,
        SQOS,
        Content,
        Bytes,
    };
    use cross_chain::payload::{
        MsgType,
        MessageItem,
        MessageVec,
        MessagePayload,
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: String,
        age: u32,
        phones: Vec<String>,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct LockerMock {
        cross_chain_contract: Option<AccountId>,
        ret: Option<String>,
    }

    impl LockerMock {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                cross_chain_contract: None,
                ret: None,
            }
        }

        /// Sets cross-chain contract address
        #[ink(message)]
        pub fn set_cross_chain_contract(&mut self, contract: AccountId) {
            self.cross_chain_contract = Some(contract);
        }

        /// Sends message to another chain 
        #[ink(message)]
        pub fn send_message(&mut self, uint_value: u32, string_value: String, struct_value: MessageDetail) {
            let to_chain = String::try_from("ETHEREUM").unwrap();
            let contract = String::try_from("ETHEREUM_CONTRACT").unwrap();
            let action = String::try_from("ETHERERUM_ACTION").unwrap();
            let mut msg_payload = MessagePayload::new();
            // msg_payload.add_item();
            let mut pl_code: Bytes = Bytes::new();
            scale::Encode::encode_to(&msg_payload, &mut pl_code);
            let data = pl_code;
            let sqos = SQOS::new(0);
            let session = Session::new(0, 0);
            let content = Content::new(contract, action, data);
            let message = SentMessage::new_sending_message(to_chain.clone(), sqos, session, content);

            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(self.cross_chain_contract.unwrap())
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x27, 0x26, 0x79, 0x17]))
                    .push_arg(message)
                )
                .returns::<()>()
                .fire()
                .unwrap();
        }

        /// Get bytes of payload
        #[ink(message)]
        pub fn get_bytes(&self, uint_value: u32, string_value: String, struct_value: MessageDetail) -> Bytes {
            // u32 item
            let mut u32_vec = Bytes::new();
            scale::Encode::encode_to(&uint_value, &mut u32_vec);

            let mut u32_item = MessageItem{
                n: 1,
                t: MsgType::InkU32,
                v: u32_vec,
            };

            // string item
            let mut string_vec = Bytes::new();
            scale::Encode::encode_to(&string_value, &mut string_vec);

            let mut string_item = MessageItem{
                n: 2,
                t: MsgType::InkString,
                v: string_vec,
            };

            // struct item
            let mut struct_vec = Bytes::new();
            scale::Encode::encode_to(&struct_value, &mut struct_vec);

            let mut struct_item = MessageItem{
                n: 3,
                t: MsgType::UserData,
                v: struct_vec,
            };

            let mut paylaod = MessagePayload::new();
            paylaod.add_item(u32_item);
            paylaod.add_item(string_item);
            paylaod.add_item(struct_item);
            
            let mut ret = Bytes::new();
            scale::Encode::encode_to(&paylaod, &mut ret);
            ret
        }

        /// Receives message from another chain 
        #[ink(message)]
        pub fn receive_message(&mut self, payload: MessagePayload) -> String {
            let item1 = payload.get_item(1).unwrap();
            let item2 = payload.get_item(2).unwrap();
            let item3 = payload.get_item(3).unwrap();
            let param1: u32 = scale::Decode::decode(&mut item1.v.as_slice()).unwrap();
            let param2: String = scale::Decode::decode(&mut item2.v.as_slice()).unwrap();
            let param3: MessageDetail = scale::Decode::decode(&mut item3.v.as_slice()).unwrap();
            // let payload
            let mut s = String::new();
            s = s + &ink_prelude::format!("{:?}-", param1);
            s = s + &ink_prelude::format!("{:?}-", param2);
            s = s + &ink_prelude::format!("{:?}-", param3);
            self.ret = Some(s.clone());
            s
        }

        /// Receives message from another chain 
        #[ink(message)]
        pub fn get_ret(& self) -> String {
            self.ret.clone().unwrap()
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
        use cross_chain::message_define::{
            SentMessage,
            Session,
            SQOS,
            Content,
            Bytes,
        };
        use cross_chain::CrossChainRef;

        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            let locker = LockerMock::new();
        }

        /// We test if set_cross_chain_contract works.
        #[ink::test]
        fn set_cross_chain_contract_works() {
            let mut locker = LockerMock::new();
            let contract_id = ink_env::test::callee::<ink_env::DefaultEnvironment>();
            locker.set_cross_chain_contract(contract_id);
        }
    }
}
