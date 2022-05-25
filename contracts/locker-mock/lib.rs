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
    }

    impl LockerMock {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                cross_chain_contract: None,
            }
        }

        /// Sets cross-chain contract address
        #[ink(message)]
        pub fn set_cross_chain_contract(&mut self, contract: AccountId) {
            self.cross_chain_contract = Some(contract);
        }

        /// Sends message to another chain 
        #[ink(message)]
        pub fn send_message(&self, uint_value: u32, string_value: String, struct_value: MessageDetail) {
            let to_chain = String::try_from("ETHEREUM").unwrap();
            let contract = String::try_from("ETHEREUM_CONTRACT").unwrap();
            let action = String::try_from("ETHERERUM_ACTION").unwrap();
            let data = Bytes::new();
            let sqos = SQOS::new(0);
            let session = Session::new(0, 0);
            let content = Content::new(contract, action, data);
            let message = SentMessage::new_sending_message(to_chain.clone(), sqos, session, content);

            let ret: String = ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(self.cross_chain_contract.unwrap())
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x27, 0x26, 0x79, 0x17]))
                    .push_arg(message)
                )
                .returns::<String>()
                .fire()
                .unwrap();
        }

        // /// Receives message from another chain 
        // #[ink(message)]
        // pub fn receive_message(&self, uint_value: u32, string_value: String, struct_value: MessageDetail) {
        // }
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
            // let cross_chain = CrossChainRef::new("POLKADOT".to_string());
            let contract_id = ink_env::test::callee::<ink_env::DefaultEnvironment>();
            println!("{:?}", contract_id);
            // locker.set_cross_chain_contract()
        }
    }
}
