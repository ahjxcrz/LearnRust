#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp22 {
    
    // imports from openbrush
	use openbrush::contracts::psp22::Transfer;
	use openbrush::traits::String;
	use openbrush::traits::Storage;
	use openbrush::contracts::psp22::extensions::mintable::*;
	use openbrush::contracts::psp22::extensions::wrapper::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct drcao {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		wrapper: wrapper::Data,
		cap: Balance,
    }
    
    // Section contains default implementation without any modifications
	impl PSP22 for drcao {}
	impl PSP22Mintable for drcao {}
	impl PSP22Wrapper for drcao {}

	impl Transfer for drcao {
		fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
			_to: Option<&AccountId>,
			_amount: &Balance
        ) -> Result<(), PSP22Error> {
			if _from.is_none() && (self.total_supply() + _amount) > self.cap() {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            Ok(())
		}
	}
     
    impl drcao {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			_instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
			_instance
        }

		#[ink(message)]
		pub fn cap(&self) -> Balance {
			self.cap
		}

		fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
			if cap <= 0 {
                return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
            }
            self.cap = cap;
            Ok(())
		}
    }
}
