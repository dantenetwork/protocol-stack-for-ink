#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod tokenomics {

    use ink_storage::traits::{PackedLayout, SpreadLayout, SpreadAllocate};

    /// for test
    #[derive(Debug, PartialEq, Clone, Eq, PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(SpreadAllocate, ::scale_info::TypeInfo))]
    pub struct StakingInfo{
        amount: u128,
        reward: u128,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Tokenomics {
        owner: AccountId,
        ps_contract: Option<AccountId>,
        staking_routers: ink_storage::Mapping<AccountId, StakingInfo>,
    }

    impl Tokenomics {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = ink_env::caller::<ink_env::DefaultEnvironment>();
                contract.ps_contract = None;
            })

            // Self { 
            //     value: init_value,
            //     owner: ink_env::caller::<ink_env::DefaultEnvironment>(),
            //     ps_contract: None,
            // }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
            // Self::new(Default::default())
        }

        /// set the protocol stack contract address
        #[ink(message)]
        pub fn set_protocol_stack(&mut self, ps_addr: AccountId) {
            if ink_env::caller::<ink_env::DefaultEnvironment>() != self.owner {
                // TODO: `chain-extension`
                return;
            }
            
            self.ps_contract = Some(ps_addr);
        }

        /// Register router
        #[ink(message)]
        pub fn register_router(&mut self) {
            let router_addr = ink_env::caller::<ink_env::DefaultEnvironment>();
            // register router to storage
            let staking_info = StakingInfo {
                amount: 0,
                reward: 0,
            };

            self.staking_routers.insert(router_addr, &staking_info);
        }

        /// Pledge
        #[ink(message)]
        pub fn pledge(&mut self, value: u128) {
            let router_addr = ink_env::caller::<ink_env::DefaultEnvironment>();
            // TODO: call `transferFrom` to check if `value` is valid

            // add `value` to the staking amount of the related router
            if let Some(mut staking_info) = self.staking_routers.get(router_addr) {
                staking_info.amount += value;
                self.staking_routers.insert(router_addr, &staking_info);
            } else{
                let staking_info = StakingInfo{
                    amount: value,
                    reward: 0,
                };
                self.staking_routers.insert(router_addr, &staking_info);
            }
        }

        // get the staking amount of the router
        #[ink(message)]
        pub fn get_staking(&self, router_addr: AccountId) -> Option<StakingInfo> {
            if let Some(staking_info) = self.staking_routers.get(router_addr) {
                Some(staking_info)
            } else{
                None
            }
        }

        /// Reward
        #[ink(message)]
        pub fn reward(&mut self, router_addr: AccountId, value: u128) {
            if ink_env::caller::<ink_env::DefaultEnvironment>() != self.ps_contract.unwrap() {
                // TODO: `chain-extension`
                return;
            }
            
            if let Some(mut staking_info) = self.staking_routers.get(router_addr) {
                staking_info.reward += value;
                self.staking_routers.insert(router_addr, &staking_info);
            }
        }

        /// get the owner.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// get the protocol stack contract address.
        #[ink(message)]
        pub fn get_protocol_stack(&self) -> Option<AccountId> {
            self.ps_contract
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

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            
        }
    }
}
