#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod greeting {
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

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Greeting {
        cross_chain_contract: Option<AccountId>,
        ret: Option<String>,
    }

    impl Greeting {
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

        /// Sends greeting to another chain 
        #[ink(message)]
        pub fn send_greeting(&mut self, chain_name: String, greeting: Vec<String>) {
            let contract = String::try_from("0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b").unwrap();
            let action = String::try_from("receiveGreeting").unwrap();
            let mut msg_payload = MessagePayload::new();
            let mut itemValue = greeting.clone();
            let mut item_vec = Bytes::new();
            scale::Encode::encode_to(&itemValue, &mut item_vec);
            // msg_payload.add_item();
            let mut item = MessageItem {
                n: 1,
                t: MsgType::InkStringArray,
                v: item_vec,
            };
            msg_payload.add_item(item);
            let mut pl_code: Bytes = Bytes::new();
            scale::Encode::encode_to(&msg_payload, &mut pl_code);
            let data = pl_code;
            let sqos = SQOS::new(1);
            let session = Session::new(0, 0);
            let content = Content::new(contract, action, data);
            let message = SentMessage::new_sending_message(chain_name.clone(), sqos, session, content);

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

        /// Receives greeting from another chain 
        #[ink(message)]
        pub fn receive_greeting(&mut self, payload: MessagePayload) -> String {
            let item = payload.get_item(1).unwrap();
            let param: Vec<String> = scale::Decode::decode(&mut item.v.as_slice()).unwrap();
            // let payload
            let mut s = String::new();
            s = s + &ink_prelude::format!("{:?}", param);
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
            let locker = Greeting::new();
        }

        /// We test if set_cross_chain_contract works.
        #[ink::test]
        fn set_cross_chain_contract_works() {
            let mut locker = Greeting::new();
            let contract_id = ink_env::test::callee::<ink_env::DefaultEnvironment>();
            locker.set_cross_chain_contract(contract_id);
        }
    }
}
