#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::magink_nft::MaginkNftRef;

#[openbrush::implementation(PSP34, PSP34Enumerable, Ownable, PSP34Mintable, PSP34Metadata)]
#[openbrush::contract]
pub mod magink_nft {
  use super::ensure;
  use ink::prelude::string::String;
  use openbrush::traits::Storage;

  #[ink(event)]
  pub struct Mint {
    to: AccountId,
    id: u8,
  }

  #[derive(Default, Storage)]
  #[ink(storage)]
  pub struct MaginkNft {
    #[storage_field]
    psp34: psp34::Data,
    #[storage_field]
    ownable: ownable::Data,
    #[storage_field]
    metadata: metadata::Data,
    #[storage_field]
    enumerable: enumerable::Data,
  }

  impl MaginkNft {
    #[ink(constructor, payable)]
    pub fn new(owner: AccountId, uri: String) -> Self {
      let mut _instance = Self::default();
      ownable::Internal::_init_with_owner(&mut _instance, owner);

      let collection_id = psp34::PSP34Impl::collection_id(&_instance);
      metadata::Internal::_set_attribute(
        &mut _instance,
        collection_id.clone(),
        String::from("name"),
        String::from("Magink_Nft"),
      );
      metadata::Internal::_set_attribute(
        &mut _instance,
        collection_id.clone(),
        String::from("symbol"),
        String::from("MGN"),
      );
      metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("URI"), uri);

      _instance
    }

    #[ink(message)]
    #[openbrush::modifiers(only_owner)]
    pub fn mint(&mut self, account: AccountId) -> Result<String, PSP34Error> {
      ensure!(
        psp34::PSP34Impl::balance_of(self, account) == 0,
        PSP34Error::TokenExists
      );

      let id = psp34::PSP34Impl::total_supply(self) as u8;
      psp34::InternalImpl::_mint_to(self, account, Id::U8(id)).expect("can't mint");
      self.env().emit_event(Mint { to: account, id });

      Ok(String::from("mint successfull"))
    }

    #[ink(message)]
    pub fn account_id(&self) -> AccountId {
      self.env().account_id()
    }
  }
}

#[macro_export]
macro_rules! ensure {
  ( $x:expr, $y:expr $(,)? ) => {{
    if !$x {
      return Err($y.into())
    }
  }};
}
