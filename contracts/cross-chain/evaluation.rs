
use ink_lang as ink;
use ink_env::AccountId;
use ink_prelude::vec::Vec;
use ink_storage::{
  traits::{
      SpreadLayout,
      PackedLayout,
  }
};


#[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct IEvaluationCoefficient {
  pub min_credibility: u32,
  pub max_credibility: u32,
  pub middle_credibility: u32,
  pub range_crediblility: u32,
  pub success_step: u32,
  pub do_evil_step: u32,
  pub exception_step: u32,
}

/// SQOS structure
#[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct ICredibilitySelectionRatio {
  pub upper_limit: u32,
  pub lower_limit: u32,
}

#[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct IThreshold {
  pub credibility_weight_threshold: u32,
  pub min_seleted_threshold: u32,
  pub trustworthy_threshold: u32,
}

#[derive(SpreadLayout, PackedLayout, Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub struct IEvaluation {
  pub threshold: IThreshold,
  pub credibility_selection_ratio: ICredibilitySelectionRatio,
  pub evaluation_coefficient: IEvaluationCoefficient,
  pub initial_credibility_value: u32,
  pub selected_number: u8,
}


#[ink::trait_definition]
pub trait RoutersCore {
    /// @notice Called from cross-chain routers for re-selecting routers for this time stage.
    ///
    /// @dev Refresh the begining and end of the current time stage if the current period ended.
    /// Cross contract call to `cross-chain protocol contract` to `select_routers` new routers
    #[ink(message)]
    fn select_routers(&mut self);
    
    /// @notice Called from `msg verify contract` to get the credibilities of routers to take weighted aggregation verification of messages
    ///
    /// @dev
    /// @param routers
    #[ink(message)]
    fn get_routers(&self, routers: Option<Vec<AccountId>>) -> Vec<(AccountId, u32)>;

    /// @notice Called from off-chain router to register themselves as the cross chain router.
    /// Get router accountId through `Self::env::caller()`.
    #[ink(message)]
    fn register_router(&mut self, routers: AccountId);

    /// @notice Called from off-chain router to unregister.
    /// Get node address through `Self::env::caller()`.
    #[ink(message)]
    fn unregister_router(&mut self, router: AccountId);

    /// set the initial value of the credibility of the newly added router
    #[ink(message)]
    fn set_initial_credibility(&mut self, value: u32);

    /// set the number of routers to be selected
    #[ink(message)]
    fn set_selected_number(&mut self, number: u8);

    #[ink(message)]
    fn set_threshold(&mut self, threshold: IThreshold);

    #[ink(message)]
    fn set_credibility_selection_ratio(&mut self, ratio: ICredibilitySelectionRatio);
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