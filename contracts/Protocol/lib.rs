#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude;

#[ink::contract]
mod d_protocol_stack {

    use ink_storage::{
        traits::{
            SpreadLayout,
            StorageLayout,
            PackedLayout,
        },
    };

    struct Wrapper {
        data: ink_prelude::vec::Vec::<u8>,
    }
    
    impl Wrapper {
        pub fn new(data: ink_prelude::vec::Vec::<u8>) -> Self {
            Self {
                data,
            }
        }
    }
    
    impl scale::Encode for Wrapper {
        #[inline]
        fn size_hint(&self) -> usize {
            scale::Encode::size_hint(&self.data)
        }
    
        #[inline]
        fn encode_to<O: scale::Output + ?Sized>(&self, output: &mut O) {
            output.write(&self.data);
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct MessageDetail{
        name: ink_prelude::string::String,
        age: u32,
        phones: ink_prelude::vec::Vec<ink_prelude::string::String>,
    }

    /// Simelation
    #[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct SimNode(u16, u32);

    /// selection interval
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct SelectionInterval {
        pub id: u16,
        pub low: u32,
        pub high: u32,
        pub selected: bool,
    }

    impl SelectionInterval {
        pub fn contains(&self, value: u32) -> bool {
            if value >= self.low && value < self.high {
                true
            } else {
                false
            }
        }
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

        sim_routers: ink_prelude::vec::Vec<SimNode>,
    }

    impl DProtocalStack {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
                account: Self::env().caller(),
                sim_routers: ink_prelude::vec![],
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
        /// Test `MessageDetail` in Protocol is the same in Callee
        #[ink(message)]
        pub fn submit_message(& self, callee_account: AccountId, msg: ink_prelude::vec::Vec::<u8>) -> ink_prelude::string::String{
            // cache the msg

            // verifiy the msg

            // submit msg to callee account
            self.call_to_contracts(callee_account, msg)
        }

        #[ink(message)]
        pub fn get_struct_message_u8(& self, msg: MessageDetail) -> ink_prelude::vec::Vec::<u8>{
            let mut v = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);
            v
        }
        
        /// inner interface for dynamically call to user application contract
        fn call_to_contracts(& self, callee_account: AccountId, msg: ink_prelude::vec::Vec::<u8>) -> ink_prelude::string::String{
            let data: ink_prelude::vec::Vec::<u8> = msg.clone().drain(4..).collect();
            let wrapped_data = Wrapper::new(data);
            
            let my_return_value: ink_prelude::string::String =  ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(callee_account)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([msg[0], msg[1], msg[2], msg[3]]))
                    .push_arg(wrapped_data)
                )
                .returns::<ink_prelude::string::String>()
                .fire()
                .unwrap();
            my_return_value
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

        #[ink(message)]
        pub fn create_intervals(&self, just_for_test: bool) -> ink_prelude::vec::Vec<SelectionInterval>{
            let mut sum: u32 = 0;
            let mut select_intervals = ink_prelude::vec![];
            for router in self.sim_routers.iter() {
                select_intervals.push(SelectionInterval{
                    id: router.0,
                    low: sum,
                    high: sum + router.1,
                    selected: false,
                });
                sum += router.1;
            }

            select_intervals
        }

        /// Test selection algorithm
        /// test interface for register
        #[ink(message)]
        pub fn random_register_routers(&mut self, routers: ink_prelude::vec::Vec<u32>) {
            let mut start_id = self.sim_routers.len() as u16;
            for ele in routers {
                self.sim_routers.push(SimNode(start_id, ele));
                start_id += 1;
            }
        }

        /// Test selection algorithm
        /// test interface 
        #[ink(message)]
        pub fn selection_test(&self, n: u16) -> Option<ink_prelude::vec::Vec<u16>>{
            let mut start_idx = 0;
            let mut select_intervals = self.create_intervals(true);
            if (select_intervals.len() as u16) < n {
                return None;
            }

            let mut selected: ink_prelude::vec::Vec<u16> = ink_prelude::vec![];
            while (selected.len() as u16) < n {
                let random_seed = ink_env::random::<ink_env::DefaultEnvironment>(&[start_idx]).unwrap().0;
                let mut seed_idx = 0;

                while seed_idx < random_seed.as_ref().len() {
                    let two_bytes: [u8; 2] = random_seed.as_ref()[seed_idx..seed_idx+2].try_into().unwrap();
                    let rand_num = u16::from_be_bytes(two_bytes) as u32;

                    let max = select_intervals[select_intervals.len() - 1].high;

                    // rand_num will multiple 100 in later implementation as the credibility does
                    let rand_num = rand_num % max;

                    let mut choose_next = false;
                    for ele in select_intervals.iter_mut() {
                        if ele.contains(rand_num) {
                            if !ele.selected {
                                selected.push(ele.id);
                                ele.selected = true;
                                break;
                            } else {
                                choose_next = true;
                            }
                        }

                        if choose_next && (!ele.selected) {
                            selected.push(ele.id);
                            ele.selected = true;
                            break;
                        }
                    }

                    if (selected.len() as u16) >= n {
                        return Some(selected);
                    }

                    seed_idx += 2;
                }

                start_idx += 1;
            }

            Some(selected)
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

        #[ink::test]
        fn test_encode() {
            let mut data = ink_prelude::vec::Vec::<u8>::new();
            data.push(0x10);
            data.push(0x4e);
            data.push(0x69);
            data.push(0x6b);
            data.push(0x61);
            data.push(0x24);
            let wrapped_data = Wrapper::new(data);

            let mut buf = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&wrapped_data, &mut buf);
            assert_eq!(buf, [0x10, 0x4e, 0x69, 0x6b, 0x61, 0x24]);
        }

        #[ink::test]
        fn multi_params() {
            let mut phones = ink_prelude::vec::Vec::<ink_prelude::string::String>::new();
            phones.push(ink_prelude::string::String::from("123"));
            phones.push(ink_prelude::string::String::from("456"));
            let msg_struct = MessageDetail{
                name: ink_prelude::string::String::from("Nika"),
                age: 18,
                phones: phones,
            };

            let exec_input = ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0xa9, 0x45, 0xce, 0xc7]))
                    .push_arg(msg_struct)
                    .push_arg(ink_prelude::string::String::from("hthuang"))
                    .push_arg(666);
            
            let mut buf = ink_prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&exec_input, &mut buf);
            println!("{:?}", buf);
            // assert_eq!(buf, [0x10, 0x4e, 0x69, 0x6b, 0x61, 0x24]);
        }
    }
}
