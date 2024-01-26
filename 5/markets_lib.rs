#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod markets {

    use base_erc20::{BaseErc20};
    use erc721::{Erc721Ref,Erc721,TokenId};

    #[ink(storage)]
    pub struct Markets{
        acceptable_erc20: ink::contract_ref!(BaseErc20),
        erc721: Erc721Ref,
        price:Balance,
        minted_count:u32,
    }
    
    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
      Erc20TransferFail,
      Erc721MinFail,
      Erc721TransferFail,
    }

    pub type Result<T> = core::result::Result<T, Error>;


    impl Markets{
        #[ink(constructor)]
        pub fn new (erc721:Erc721Ref,erc20:AccountId)->Self{
                Self{
                    acceptable_erc20:erc20.into(),
                    erc721,
                    price:20,
                    minted_count:0,
                }
        }

        #[ink(message)]
        pub fn buy_nft(&mut self)->Result<()>{
            let sender= self.env().caller();
            let res= self.acceptable_erc20.transfer_from(sender,self.env().account_id(),self.price);
            if res.is_err(){
                return Err(Error::Erc20TransferFail);
            }
            self.minted_count+=1;
            let min_res = self.erc721.mint(self.minted_count);
            if min_res.is_err(){
                return Err(Error::Erc721MinFail);
            }
            let tranfer_res = self.erc721.transfer(sender,self.minted_count);
            if tranfer_res.is_err(){
                return Err(Error::Erc721TransferFail);
            }
            Ok(())
        }
    }
}