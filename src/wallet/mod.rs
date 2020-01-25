
use std::process;
use std::str::FromStr;
use web3::api::Web3;
use web3::contract::Contract;
use web3::contract::{Options};
use web3::futures::Future;
use web3::types::{Address, Bytes, FilterBuilder};
use crate::constants;
use crate::helpers;
use crate::tui;

#[derive(Clone, Debug)]
pub struct Wallet<'a, T: web3::Transport> {
  pub contract: Contract<T>,
  web3: &'a Web3<T>
}

impl<'a, T: web3::Transport> Wallet<'a, T> {

  pub fn new(address: Address, web3: &'a Web3<T>) -> Self {
    let wallet = Wallet::<'a, T> {contract: Contract::from_json(web3.eth(), address, constants::abis::WALLET).unwrap(), web3: web3};

    wallet
  }

  pub fn ens(&self) -> String {
    helpers::to_ens(self.contract.address(), self.web3)
  }

  pub fn owner(&self) -> Address {
    let result = self.contract.query("owner", (), None, Options::default(), None);

    let owner: Address = result.wait().unwrap_or_else(|_e| {
      tui::error(format!("unable to fetch owner for {:?}", self.contract.address()));
      process::exit(1)
    });

    owner
  }
  
  pub fn guardians(&self) -> Vec<Address> {
    let wallet_address = self.contract.address();

    let guardian_manager_address = Address::from_str(&"FF5A7299ff6f0fbAad9b38906b77d08c0FBdc9A7").unwrap();


    let gardian_manager = Contract::from_json(self.web3.eth(), guardian_manager_address, constants::abis::GUARDIAN_MANAGER).unwrap();
    let query = gardian_manager.query("guardianStorage", (), None, Options::default(), None);
    let guardian_storage_address: Address = query.wait().unwrap_or_else(|_e| {
      tui::error(format!("unable to fetch guardian storage address for {:?}", self.contract.address()));
      process::exit(1)
    });


    let guardian_storage = Contract::from_json(self.web3.eth(), guardian_storage_address, constants::abis::GUARDIAN_STORAGE).unwrap();
    let guardians: Vec::<Address> = guardian_storage.query("getGuardians", (wallet_address,), None, Options::default(), None).wait().unwrap_or_else(|_e| {
      tui::error(format!("unable to fetch guardians for {:?}", self.contract.address()));
      process::exit(1)
    });


    guardians
  }

  pub fn modules(&self) -> Vec<Address> {
    let bytes_false: Bytes = Bytes::from(vec![0; 32]);
    // let BYTES_TRUE: Bytes = Bytes::from([vec![0; 31], vec![1]].concat());

    let filter = FilterBuilder::default()
      .address(vec![self.contract.address()])
      .topics(
        Some(vec![
            "8da3ff870ae294081392139550e167f1f31f277f22015ee22fbffdbd7758f4e1"
              .parse()
              .unwrap(),
        ]),
        None,
        None,
        None,
      )
      .from_block(constants::ARGENT_GENESIS_BLOCK.into())
      .build();

    let logs = self.web3.eth().logs(filter).wait().unwrap();
    let mut modules = Vec::<Address>::new();

    for log in logs.iter() {
      let address = Address::from(log.topics[1]);

      if log.data.eq(&bytes_false) {
        modules.retain(|&module| module != address);
      } else {
        modules.push(address);
      }
    }

    modules
  }
}