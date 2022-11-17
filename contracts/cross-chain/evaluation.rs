use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

use crate::storage_define::{CredibilitySelectionRatio, Error, Threshold};

#[ink::trait_definition]
pub trait RoutersCore {
    /// @notice Called from cross-chain routers for re-selecting routers for this time stage.
    ///
    /// @dev Refresh the begining and end of the current time stage if the current period ended.
    /// Cross contract call to `cross-chain protocol contract` to `select_routers` new routers
    #[ink(message)]
    fn select_routers(&mut self) -> Result<Vec<AccountId>, Error>;

    /// @notice Called from `msg verify contract` to get the credibilities of routers to take weighted aggregation verification of messages
    ///
    /// @dev
    /// @param routers
    #[ink(message)]
    fn get_routers(&self) -> Vec<(AccountId, u32)>;

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
    fn set_credibility_selection_ratio(
        &mut self,
        ratio: CredibilitySelectionRatio,
    ) -> Result<(), Error>;
}
