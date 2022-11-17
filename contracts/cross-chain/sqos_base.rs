use crate::storage_define::{Bytes, Error};
/// Trait for basic cross-chain contract
// use ink_lang as ink;
use ink::prelude::string::String;
use ink::primitives::AccountId;

#[ink::trait_definition]
pub trait SQoSBase {
    /// Sets DAT token contract address
    #[ink(message)]
    fn receive_hidden_message(
        &mut self,
        from_chain: String,
        id: u128,
        contract: AccountId,
        hash: Bytes,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn challenge(&mut self, from_chain: String, id: u128) -> Result<(), Error>;
}
