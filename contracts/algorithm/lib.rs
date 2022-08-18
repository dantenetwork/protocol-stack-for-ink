#![cfg_attr(not(feature = "std"), no_std)]

//! Note:
//! The `algorithm_prototype` is to help users understand the mechanisms of the protocol stack more intuitively

use ink_lang as ink;

use payload::message_define::{IReceivedMessage};
 
#[ink::contract]
mod algorithm {

    use ink_storage::{
        traits::{
            SpreadLayout,
            StorageLayout,
            PackedLayout,
            SpreadAllocate,
            PackedAllocate,
        },
    };

    /// Simulation
    #[derive(SpreadLayout, PackedLayout, SpreadAllocate, Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct SimNode(u16, u32);

    impl PackedAllocate for SimNode {
        fn allocate_packed(&mut self, at: &ink_primitives::Key) {
            PackedAllocate::allocate_packed(&mut self.0, at);
            PackedAllocate::allocate_packed(&mut self.1, at);
        }
    }

    /// selection interval
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct SelectionInterval {
        pub id: u16,
        pub cre: u32,
        pub low: u32,
        pub high: u32,
        pub selected: u16,
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

    /// message simulation
    #[derive(SpreadLayout, PackedLayout, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct MessageInfo {
        msg_hash: [u8;32],
        // the struct is `IReceivedMessage`
        msg_detail: ink_prelude::vec::Vec<u8>,
        submitters: ink_prelude::vec::Vec<u16>,
    }

    impl MessageInfo {
        pub fn get_submitter_count(&self) -> u16 {
            self.submitters.len() as u16
        }
    }

    #[derive(SpreadLayout, PackedLayout, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct RecvedMessage {
        msg_id: u128,
        msg_vec: ink_prelude::vec::Vec<MessageInfo>,
        processed: bool,
    }

    impl RecvedMessage {
        pub fn get_submitter_count(&self) -> u16 {
            let mut count: u16 = 0;
            for ele in self.msg_vec.iter() {
                count += ele.get_submitter_count();
            }

            count
        }

        pub fn contains(&self, router_id: u16) -> bool {
            for msg_ele in self.msg_vec.iter() {
                for router_ele in msg_ele.submitters.iter() {
                    if *router_ele == router_id {
                        return true;
                    }
                }
            }

            false
        }
    }

    #[derive(SpreadLayout, PackedLayout, Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct VerifyInfo {
        cred_sum: u128,
        submitters: ink_prelude::vec::Vec<u16>,
    }

    #[ink(event)]
    pub struct VerifiedMessage {
        vf_passed: bool,
        submitted: ink_prelude::vec::Vec<VerifyInfo>,
    }

    #[ink(event)]
    pub struct EvaluateResult {
        behavior_type: ink_prelude::string::String,
        results: ink_prelude::vec::Vec<u32>,
    }

    #[derive(SpreadLayout, PackedLayout, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, StorageLayout))]
    pub struct VerifiedCache {
        msg_id: u128,
        submitted: ink_prelude::vec::Vec<VerifyInfo>,
        vf_passed: bool,
    }

    // use serde_json::json;
    // use serde_json_wasm::{from_str, to_string};
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct AlgorithmPrototype {
        /// Stores a single `bool` value on the storage.
        value: bool,
        account: AccountId,
        msg_copy_count: u16,
        vf_threshold: u128,
        coe_middle_cred: u32,
        coe_min_cred: u32,
        coe_max_cred: u32,
        coe_range_cred: u32,

        /// This type of storage needs to be optimized in product implementation
        /// Follow this [issue: Allow iteration over contract storage #11410](https://github.com/paritytech/substrate/issues/11410#issuecomment-1156775111)
        sim_router_keys: ink_prelude::vec::Vec<u16>,
        sim_routers: ink_storage::Mapping<u16, SimNode>,

        /// To be optimized
        msg_v_keys: ink_prelude::vec::Vec<(ink_prelude::string::String, u128)>,
        msg_2_verify: ink_storage::Mapping<(ink_prelude::string::String, u128), RecvedMessage>,

        /// To be optimized
        /// Just for showing the result of the verification
        cache_verified_keys: ink_prelude::vec::Vec<u128>,
        cache_verified: ink_storage::Mapping<u128, VerifiedCache>,
    }

    impl AlgorithmPrototype {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                contract.account = Self::env().caller();
                contract.msg_copy_count = 5;
                contract.vf_threshold = 7000;
                contract.coe_middle_cred = 50;
                contract.coe_min_cred = 0;
                contract.coe_max_cred = 100;
                contract.coe_range_cred = contract.coe_max_cred - contract.coe_min_cred;
                contract.sim_router_keys = ink_prelude::vec![];
                contract.msg_v_keys = ink_prelude::vec![];
                contract.cache_verified_keys = ink_prelude::vec![];
            })
        }

        #[ink(message)]
        /// Set the system parameters
        pub fn set_sysinfo(&mut self, msg_copy_count: u16, vf_t: u128) {
            // just for test without account validation 
            self.value = !self.value;
            self.msg_copy_count = msg_copy_count;
            self.vf_threshold = vf_t;
        }

        #[ink(message)]
        /// Simply returns the current system setting.
        pub fn get_sysinfo(&self) -> (bool, u16, u128) {
            (self.value, self.msg_copy_count, self.vf_threshold)
        }
    
        #[ink(message)]
        /// Simulation to the simplest version of the routers selection algoritm in Dante protocol
        /// 
        /// Call `random_register_routers` to add some simulation routers with fixed credibility, 
        /// which will be dynamically adjusted by *router evaluation* algorithm in product implementation.
        /// 
        /// `create_intervals` is part of router selection algorithm
        /// 
        /// `selection_test` will randomly choose `n` routers according to their credibility
        /// 
        /// `selection_statistic` provides an intuitive validation of the 'Probability distribution' results of the router selection algorithm
        /// parameter `n` is the number of select times
        pub fn create_intervals(&self, just_for_test: bool) -> ink_prelude::vec::Vec<SelectionInterval>{
            let mut sum: u32 = 0;
            let mut select_intervals = ink_prelude::vec![];
            for router_key in self.sim_router_keys.iter() {
                if let Some(router) = self.sim_routers.get(router_key) {
                    select_intervals.push(SelectionInterval{
                        id: router.0,
                        cre: router.1,
                        low: sum,
                        high: sum + router.1,
                        selected: 0,
                    });
                    sum += router.1;
                } 
            }

            select_intervals
        }

        /// Test selection algorithm
        /// test interface for register
        #[ink(message)]
        pub fn random_register_routers(&mut self, routers: ink_prelude::vec::Vec<u32>) {
            let mut start_id = self.sim_router_keys.len() as u16;
            for ele in routers {
                self.sim_router_keys.push(start_id);
                self.sim_routers.insert(&start_id, &SimNode(start_id, ele));
                start_id += 1;
            }
        }

        #[ink(message)]
        pub fn get_registered_routers(&self, flag: bool) -> ink_prelude::vec::Vec<SimNode> {
            let mut reg_routers = ink_prelude::vec![];
            for ele in self.sim_router_keys.iter() {
                if let Some(router) = self.sim_routers.get(ele) {
                    reg_routers.push(router);
                }
            }

            reg_routers
        }

        #[ink(message)]
        pub fn clear_routers(&mut self) {
            for ele in self.sim_router_keys.iter() {
                    self.sim_routers.remove(ele);
            }

            self.sim_router_keys.clear();
        }

        /// selection statistic
        /// This provides an intuitive validation of the 'Probability distribution' results of the router selection algorithm
        /// Prameter@n: Sampling times
        #[ink(message)]
        pub fn selection_statistic(&self, n: u16) -> Option<ink_prelude::vec::Vec<SelectionInterval>>{
            let mut start_idx: u16 = 0;
            let mut select_intervals = self.create_intervals(true);

            if select_intervals.len() == 0 {
                return None;
            }

            let mut selected = 0;

            while selected < n {
                let start_seed = u16::to_be_bytes(start_idx);
                let random_seed = ink_env::random::<ink_env::DefaultEnvironment>(&start_seed).unwrap().0;
                let mut seed_idx = 0;

                while seed_idx < (random_seed.as_ref().len() - 1) {
                    let two_bytes: [u8; 2] = random_seed.as_ref()[seed_idx..seed_idx+2].try_into().unwrap();
                    let rand_num = u16::from_be_bytes(two_bytes) as u32;

                    let max = select_intervals[select_intervals.len() - 1].high;

                    // rand_num will multiple 100 in later implementation as the credibility does
                    let rand_num = rand_num % max;

                    for ele in select_intervals.iter_mut() {
                        if ele.contains(rand_num) {
                            selected += 1;
                            ele.selected += 1;
                            break;
                        }
                    }

                    if selected >= n {
                        return Some(select_intervals);
                    }

                    seed_idx += 2;
                }

                start_idx += 1;
            }

            Some(select_intervals)
        }

        /// Test selection algorithm
        /// this will randomly choose `n` routers according to their credibility
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

                while seed_idx < (random_seed.as_ref().len() - 1) {
                    let two_bytes: [u8; 2] = random_seed.as_ref()[seed_idx..seed_idx+2].try_into().unwrap();
                    let rand_num = u16::from_be_bytes(two_bytes) as u32;

                    let max = select_intervals[select_intervals.len() - 1].high;

                    // rand_num will multiple 100 in later implementation as the credibility does
                    let rand_num = rand_num % max;

                    let mut choose_next = false;
                    for ele in select_intervals.iter_mut() {
                        if ele.contains(rand_num) {
                            if ele.selected == 0 {
                                selected.push(ele.id);
                                ele.selected += 1;
                                break;
                            } else {
                                choose_next = true;
                            }
                        }

                        if choose_next && (ele.selected == 0) {
                            selected.push(ele.id);
                            ele.selected += 1;
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

        /// simulation of message verification
        /// 
        /// In this simulation, we do not limit the number of message copies to verify a message. 
        /// And the number determines how many routers one message needs to be delivered parallelly, 
        /// this will be configured by users through SQoS settings in the product implementation.
        /// At that time, when enough copies have been delivered, `simu_message_verification` will be called dynamically.
        /// 
        /// `simu_submit_message` simulates the submittion of delivered message copies
        /// Message copies belong to the same message only if they have the same `IReceivedMessage::id` and `IReceivedMessage::from_chain` 
        /// 
        /// #param@router_id: this is a parameter just for test. In product implementation, this will be `Self::env().caller()`
        /// 
        #[ink(message)]
        pub fn simu_submit_message(&mut self, recv_msg: super::IReceivedMessage, router_id: u16) {
            // `router_id` validation
            if !self.sim_routers.contains(router_id) {
                return;
            }

            let key = (recv_msg.from_chain.clone(), recv_msg.id);

            if let Some(mut msg_instance) = self.msg_2_verify.get(&key) {
                // check whether the related message is out of time
                if msg_instance.processed {
                    return;
                }

                // check submit once
                if msg_instance.contains(router_id) {
                    return;
                }

                let msg_hash = recv_msg.into_hash::<ink_env::hash::Keccak256>();
                let mut hash_found = false;

                for ele in msg_instance.msg_vec.iter_mut() {
                    if ele.msg_hash == msg_hash {
                        ele.submitters.push(router_id);
                        hash_found = true;
                        break;
                    }
                }

                if !hash_found {
                    let mut msg_info = MessageInfo {
                        msg_hash: msg_hash,
                        msg_detail: recv_msg.into_bytes(),
                        submitters: ink_prelude::vec![],
                    };
                    msg_info.submitters.push(router_id);
                    msg_instance.msg_vec.push(msg_info);
                }

                // we comment off the following lines to manually call `simu_message_verification` for simulation
                if msg_instance.get_submitter_count() >= self.msg_copy_count {
                    // self.msg_2_verify.remove(&key);

                    self.simu_message_verification(&msg_instance);

                    let msg_processed = RecvedMessage {
                        msg_id: recv_msg.id,
                        msg_vec: ink_prelude::vec![],
                        processed: true,
                    };

                    self.msg_2_verify.insert(&key, &msg_processed);

                } else {
                    self.msg_2_verify.insert(&key, &msg_instance);
                }

            } else {
                let msg_hash = recv_msg.into_hash::<ink_env::hash::Keccak256>();

                let mut msg_instance = RecvedMessage{
                    msg_id: recv_msg.id,
                    msg_vec: ink_prelude::vec![],
                    processed: false,
                };

                let mut msg_info = MessageInfo {
                    msg_hash: msg_hash,
                    msg_detail: recv_msg.into_bytes(),
                    submitters: ink_prelude::vec![],
                };
                msg_info.submitters.push(router_id);
                msg_instance.msg_vec.push(msg_info);
                self.msg_2_verify.insert(&key, &msg_instance);

                self.msg_v_keys.push(key);

                // at least two message copies 
            }
        }

        /// Clear submitted messages manually
        #[ink(message)]
        pub fn simu_clear_message(&mut self, flag: bool) {
            for ele in self.msg_v_keys.iter() {
                self.msg_2_verify.remove(ele);
            }

            self.msg_v_keys.clear();
        }

        /// Get submitted messages
        #[ink(message)]
        pub fn simu_get_message(&self, flag: bool) -> ink_prelude::vec::Vec<RecvedMessage>{
            let mut messages = ink_prelude::vec![];
            for msg_key in self.msg_v_keys.iter() {
                if let Some(msg) = self.msg_2_verify.get(msg_key) {
                    messages.push(msg);
                }
            }

            messages
        }

        /// When enough message copies are submitted, `simu_message_verification` will be called internally
        /// The result will be cached in order to be checked manually
        /// and an event `VerifiedMessage` will be emitted to show the result, but the result event need to be decoded by `Polkadot.js` 
        fn simu_message_verification(&mut self, msg_instance: &RecvedMessage) {
            if msg_instance.msg_vec.len() > 1 {
                let mut index_cred = ink_prelude::vec![];
                let mut idx: u16 = 0;
                let mut total_cred = 0;

                let mut verified_msg = VerifiedMessage {
                    vf_passed: false,
                    submitted: ink_prelude::vec![],
                };

                // just for showing the result of the verification
                self.cache_verified_keys.push(msg_instance.msg_id);
                let mut cache_verified = VerifiedCache {
                    msg_id: msg_instance.msg_id,
                    submitted: ink_prelude::vec![],
                    vf_passed: false,
                };

                for msg_ele in msg_instance.msg_vec.iter() {
                    let mut vf_info = VerifyInfo {
                        cred_sum: 0,
                        submitters: ink_prelude::vec![],
                    };

                    let mut sum_cred = 0;
                    for submitter in msg_ele.submitters.iter() {
                        if let Some(router) = self.sim_routers.get(&submitter) {
                            sum_cred += router.1;

                            vf_info.submitters.push(router.0);
                        }
                    }

                    vf_info.cred_sum = sum_cred as u128;
                    verified_msg.submitted.push(vf_info.clone());
                    // just for showing the result of the verification
                    cache_verified.submitted.push(vf_info);

                    index_cred.push((idx, sum_cred as u128));
                    idx += 1;
                    total_cred += sum_cred as u128;
                }

                let coe: u128 = 10000;

                let mut max_cred: (u16, u128) = (0, 0);

                for cred_ele in index_cred.iter_mut() {
                    cred_ele.1 = cred_ele.1 * coe / total_cred;
                    if max_cred.1 < cred_ele.1 {
                        max_cred = (cred_ele.0, cred_ele.1);
                    }
                }

                if max_cred.1 >= self.vf_threshold {
                    verified_msg.vf_passed = true;
                    // just for showing the result of the verification
                    cache_verified.vf_passed = true;

                    Self::env().emit_event(verified_msg);
                } else {
                    verified_msg.vf_passed = false;
                    // just for showing the result of the verification
                    cache_verified.vf_passed = false;

                    Self::env().emit_event(verified_msg);
                }

                // just for showing the result of the verification
                self.cache_verified.insert(&msg_instance.msg_id, &cache_verified);

            } else if msg_instance.msg_vec.len() == 1{
                let vf_info = VerifyInfo {
                    cred_sum: 100,
                    submitters: msg_instance.msg_vec[0].submitters.clone(),
                };
                
                let verified_msg = VerifiedMessage {
                    vf_passed: true,
                    submitted: ink_prelude::vec![vf_info.clone()],
                };
                
                // just for showing the result of the verification
                self.cache_verified_keys.push(msg_instance.msg_id);
                self.cache_verified.insert(&msg_instance.msg_id, &VerifiedCache {
                    msg_id: msg_instance.msg_id,
                    submitted: ink_prelude::vec![vf_info],
                    vf_passed: true,
                });

                Self::env().emit_event(verified_msg);
            } else {
                let verified_msg = VerifiedMessage {
                    vf_passed: false,
                    submitted: ink_prelude::vec![],
                };

                Self::env().emit_event(verified_msg);
            }
        }

        #[ink(message)]
        pub fn get_verified_results(&self, flag: bool) -> ink_prelude::vec::Vec<VerifiedCache> {
            let mut rst = ink_prelude::vec![];
            for ele in self.cache_verified_keys.iter() {
                if let Some(verified) = self.cache_verified.get(ele) {
                    rst.push(verified);
                }
            }

            rst
        }

        #[ink(message)]
        pub fn clear_verified_cache(&mut self) {
            for ele in self.cache_verified_keys.iter() {
                self.cache_verified.remove(ele);
            }

            self.cache_verified_keys.clear();
        }

        /// simulation of node evaluation
        /// 
        /// This is a on-chain prototype for routers eveluation to show the principle of node evaluation algorithms
        /// When a router does `do_honest_once`, its credibility will increase
        /// On the contrary, when a router does `do_evil_once`, its credibility will decrease
        #[ink(message)]
        pub fn do_honest_once(&mut self, id: u16) {
            if let Some(mut router) = self.sim_routers.get(id) {
                // TODO: increase credibility
                if router.1 < self.coe_middle_cred {
                    router.1 = 10
                        * (router.1 - self.coe_min_cred)
                        / self.coe_range_cred
                        + router.1;
                } else {
                    router.1 = 10
                        * (self.coe_max_cred - router.1)
                        / self.coe_range_cred
                        + router.1;
                }

                self.sim_routers.insert(&id, &router);
            }
        }

        #[ink(message)]
        pub fn do_evil_once(&mut self, id: u16) {
            if let Some(mut router) = self.sim_routers.get(id) {
                // TODO: decrease credibility

                router.1 = router.1
                    - 20
                        * (router.1 - self.coe_min_cred)
                        / self.coe_range_cred;

                self.sim_routers.insert(&id, &router);
            }
        }

        #[ink(message)]
        pub fn get_credibility(&self, id: u16) -> Option<u32> {
            if let Some(router) = self.sim_routers.get(id) {
                Some(router.1)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn do_honest(&mut self, id: u16, times: u32) {
            let mut honest_rst = EvaluateResult {
                behavior_type: ink_prelude::string::String::from("honest"),
                results: ink_prelude::vec![],
            };

            if let Some(mut router) = self.sim_routers.get(id) {
                honest_rst.results.push(router.1);

                // increase credibility
                let mut count = 0;
                while count < times {
                    
                    if router.1 < self.coe_middle_cred {
                        router.1 = 10
                            * (router.1 - self.coe_min_cred)
                            / self.coe_range_cred
                            + router.1;
                    } else {
                        router.1 = 10
                            * (self.coe_max_cred - router.1)
                            / self.coe_range_cred
                            + router.1;
                    }

                    honest_rst.results.push(router.1);

                    count += 1;
                }

                self.sim_routers.insert(&id, &router);

                Self::env().emit_event(honest_rst);
            }
        }

        #[ink(message)]
        pub fn do_evil(&mut self, id: u16, times: u32) {
            let mut evil_rst = EvaluateResult {
                behavior_type: ink_prelude::string::String::from("evil"),
                results: ink_prelude::vec![],
            };

            if let Some(mut router) = self.sim_routers.get(id) {
                evil_rst.results.push(router.1);
                
                // decrease credibility
                let mut count = 0;
                while count < times {
                    
                    router.1 = router.1
                                - 20
                                * (router.1 - self.coe_min_cred)
                                / self.coe_range_cred;

                    evil_rst.results.push(router.1);

                    count += 1;
                }

                self.sim_routers.insert(&id, &router);

                Self::env().emit_event(evil_rst);
            }
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

    }
}
