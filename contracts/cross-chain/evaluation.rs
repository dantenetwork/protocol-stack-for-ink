
use ink_lang as ink;
use ink_env::AccountId;
use ink_prelude::vec::Vec;
use ink_storage::{
  traits::{
      SpreadLayout,
      PackedLayout,
  }
};
use crate::storage_define::{
  Context, Error, Threshold, CredibilitySelectionRatio
};

#[ink::trait_definition]
pub trait RoutersCore {
    /// @notice Called from cross-chain routers for re-selecting routers for this time stage.
    ///
    /// @dev Refresh the begining and end of the current time stage if the current period ended.
    /// Cross contract call to `cross-chain protocol contract` to `select_routers` new routers
    #[ink(message)]
    fn select_routers(&mut self) -> Result<(), Error>;
    
    /// @notice Called from `msg verify contract` to get the credibilities of routers to take weighted aggregation verification of messages
    ///
    /// @dev
    /// @param routers
    #[ink(message)]
    fn get_routers(&self, routers: Option<Vec<AccountId>>) -> Vec<(AccountId, u32)>;

    /// @notice Called from off-chain router to register themselves as the cross chain router.
    /// Get router accountId through `Self::env::caller()`.
    #[ink(message)]
    fn register_router(&mut self, router: AccountId) -> Result<(), Error>;

    /// @notice Called from off-chain router to unregister.
    /// Get node address through `Self::env::caller()`.
    #[ink(message)]
    fn unregister_router(&mut self, router: AccountId) -> Result<(), Error>;

    /// set the initial value of the credibility of the newly added router
    #[ink(message)]
    fn set_initial_credibility(&mut self, value: u32) -> Result<(), Error>;

    /// set the number of routers to be selected
    #[ink(message)]
    fn set_selected_number(&mut self, number: u8) -> Result<(), Error>;

    #[ink(message)]
    fn set_threshold(&mut self, threshold: Threshold) -> Result<(), Error>;

    #[ink(message)]
    fn set_credibility_selection_ratio(&mut self, ratio: CredibilitySelectionRatio) -> Result<(), Error>;
}

// #[ink::trait_definition]
// pub trait MultiPorters {
//     /// Changes routers and requirement.
//     #[ink(message)]
//     fn change_routers_and_requirement(&mut self, routers: Porters, requirement: u16) -> Result<(), Error>;
//     /// Get routers.
//     #[ink(message)]
//     fn get_routers(& self) -> Porters;
//     /// Get requirement
//     #[ink(message)]
//     fn get_requirement(& self) -> u16;
//     /// Get the message id which needs to be ported by `validator` on chain `chain_name`
//     #[ink(message)]
//     fn get_msg_porting_task(& self, chain_name: String, validator: AccountId) -> u128;
// }