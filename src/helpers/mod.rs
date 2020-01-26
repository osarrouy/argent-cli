use crate::ens::ENS;
use std::str::FromStr;
use web3::api::Web3;
use web3::types::Address;

pub fn to_address<T: web3::Transport>(address: &str, web3: &Web3<T>) -> Result<Address, String> {
    if address.ends_with(".eth") || address.ends_with(".xyz") {
        let ens = ENS::new(&web3);

        match ens.address(address) {
            Ok(s) => return Ok(s),
            Err(_e) => return Err(format!("unable to resolve ENS address {}", address)),
        }
    } else {
        match Address::from_str(&address.replace("0x", "")) {
            Ok(s) => return Ok(s),
            Err(_e) => return Err(format!("invalid address {}", address)),
        }
    }
}

pub fn to_ens<T: web3::Transport>(address: Address, web3: &Web3<T>) -> Result<String, String> {
    let ens = ENS::new(&web3);

    match ens.name(address) {
        Ok(s) => Ok(s),
        Err(_e) => Err(format!("unable to ENS reverse address {}", address)),
    }
}
