use std::str::FromStr;
use web3::api::Web3;
use web3::types::{Address};
use crate::ens::ENS;
use crate::tui;

pub fn to_address<T: web3::Transport>(address: &str, web3: &Web3<T>) -> Address {
  let result: Address;

  if address.ends_with(".eth") || address.ends_with(".xyz") {
    let ens = ENS::new(&web3);
    result  = ens.address(address).unwrap_or_else(|_e| {
      tui::error(format!("unable to resolve ENS address {}", address));
      std::process::exit(1);
    });
  } else {
    result = Address::from_str(&address.replace("0x", "")).unwrap_or_else(|_e| {
      tui::error(format!("invalid address {}", address));
      std::process::exit(1);
    });
  }
  
  result
}

pub fn to_ens<T: web3::Transport>(address: Address, web3: &Web3<T>) -> Result <String, String> {
  let ens = ENS::new(&web3);
  
  match ens.name(address) {
    Ok(s) => Ok(s),
    Err(_e) => Err(format!("unable to ENS reverse address {}", address))
  }
}

