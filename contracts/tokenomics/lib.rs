#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod tokenomics {

    use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout, SpreadAllocate};

    #[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum StakingError {
        NotExist,
        NotEnough,
        ParamInvalid,
        CallerInvalid,
    }

    /// for test
    #[derive(Debug, PartialEq, Clone, Eq, SpreadAllocate, PackedLayout, SpreadLayout, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct StakingInfo{
        amount: u128,
        reward: u128,
    }

    /// system parameters
    #[derive(Debug, SpreadLayout, SpreadAllocate, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(StorageLayout, ::scale_info::TypeInfo))]
    pub struct SysParams {
        gc: u128,
        m: u128,
        b: u128,
        r: u128,
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
        total: u128,
        sp: SysParams,
    }

    impl Tokenomics {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = ink_env::caller::<ink_env::DefaultEnvironment>();
                contract.ps_contract = None;
                contract.total = 0;
                contract.sp = SysParams {
                    gc: 100,
                    m: 1000,
                    b: 50,
                    r: 100,
                }
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     ink_lang::utils::initialize_contract(|_| {})
        //     // Self::new(Default::default())
        // }

        /// set the protocol stack contract address
        #[ink(message)]
        pub fn set_protocol_stack(&mut self, ps_addr: AccountId) -> Result<(), StakingError>{
            if ink_env::caller::<ink_env::DefaultEnvironment>() != self.owner {
                // TODO: `chain-extension`
                return Err(StakingError::CallerInvalid);
            }
            
            self.ps_contract = Some(ps_addr);
            Ok(())
        }

        /// set system parameters
        #[ink(message)]
        pub fn set_sys_params(&mut self, sp: SysParams) -> Result<(), StakingError> {
            if ink_env::caller::<ink_env::DefaultEnvironment>() != self.owner {
                // TODO: `chain-extension`
                return Err(StakingError::CallerInvalid);
            }
            
            if (sp.gc > 100) || (sp.b > 100) {
                return Err(StakingError::ParamInvalid);
            }

            self.sp = sp;
            Ok(())
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
                self.total += value;
            } else{
                let staking_info = StakingInfo{
                    amount: value,
                    reward: 0,
                };
                self.staking_routers.insert(router_addr, &staking_info);
                self.total += value;
            }
        }

        /// withdraw
        #[ink(message)]
        pub fn withdraw(&mut self, value: u128) -> Result<(), StakingError> {
            let router_addr = ink_env::caller::<ink_env::DefaultEnvironment>();

            if let Some(mut staking_info) = self.staking_routers.get(router_addr) {
                if staking_info.amount < value {
                    Err(StakingError::NotEnough)
                } else {
                    staking_info.amount -= value;
                    self.staking_routers.insert(router_addr, &staking_info);
                    self.total -= value;
                    Ok(())
                }
            } else {
                Err(StakingError::NotExist)
            }
        }

        // get the staking amount of the router
        #[ink(message)]
        pub fn get_staking_info(&self, router_addr: AccountId) -> Option<StakingInfo> {
            if let Some(staking_info) = self.staking_routers.get(router_addr) {
                Some(staking_info)
            } else{
                None
            }
        }

        /// get staking score
        #[ink(message)]
        pub fn get_staking_weights(&self, router_addr: AccountId) -> Option<u128> {
            let coe: u128 = 10000;
            if let Some(staking_info) = self.staking_routers.get(router_addr) {
                if staking_info.amount <= self.sp.m {
                    let alpha: u128 = staking_info.amount * coe / self.sp.m;
                    let weights: u128 = self.sp.b * alpha / (2 * coe - alpha);
                    Some(weights)
                } else {
                    let alpha: u128 = self.sp.m * coe / staking_info.amount;
                    let weights: u128 = self.sp.b * (2 * coe - alpha) / coe;
                    Some(weights)
                }
                
            } else{
                None
            }
        }

        /// Reward
        #[ink(message)]
        pub fn reward(&mut self, router_addr: AccountId, credibility: u32) -> Result<(), StakingError>{
            if ink_env::caller::<ink_env::DefaultEnvironment>() != self.ps_contract.unwrap() {
                // TODO: `chain-extension`
                return Err(StakingError::CallerInvalid);
            }

            if credibility > 100 {
                return Err(StakingError::ParamInvalid);
            }
            
            if let Some(mut staking_info) = self.staking_routers.get(router_addr) {
                if credibility <= 50 {
                    staking_info.reward += self.sp.r;
                    self.staking_routers.insert(router_addr, &staking_info);
                } else {
                    staking_info.reward += self.sp.r + self.sp.r * (credibility as u128 - 50) / 50;
                    self.staking_routers.insert(router_addr, &staking_info);
                }
                
                Ok(())
            }else {
                Err(StakingError::NotExist)
            }
        }

        /// get the owner.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// get the protocol stack contract address.
        #[ink(message)]
        pub fn get_protocol_addr(&self) -> Option<AccountId> {
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
