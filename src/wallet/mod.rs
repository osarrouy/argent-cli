use crate::constants;
use crate::helpers;
use crate::token::Token;
use std::str::FromStr;
use web3::api::Web3;
use web3::contract::Contract;
use web3::contract::Options;
use web3::futures::Future;
use web3::types::{Address, Bytes, FilterBuilder, H256, U256};

#[derive(Clone, Debug)]
pub struct Wallet<'a, T: web3::Transport> {
    pub address: Address,
    contract: Contract<T>,
    web3: &'a Web3<T>,
}

impl<'a, T: web3::Transport> Wallet<'a, T> {
    pub fn new(address: Address, web3: &'a Web3<T>) -> Self {
        Wallet::<'a, T> {
            address,
            contract: Contract::from_json(web3.eth(), address, constants::abis::WALLET).unwrap(),
            web3,
        }
    }

    pub fn ens(&self) -> Result<String, String> {
        let result = helpers::to_ens(self.address, self.web3);

        match result {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to fetch ENS name for {:?}", self.address)),
        }
    }

    pub fn owner(&self) -> Result<Address, String> {
        let result = self
            .contract
            .query("owner", (), None, Options::default(), None);

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to fetch owner for {:?}", self.address)),
        }
    }

    pub fn guardians(&self) -> Result<Vec<Address>, String> {
        let guardian_manager =
            Address::from_str(&"FF5A7299ff6f0fbAad9b38906b77d08c0FBdc9A7").unwrap();
        let guardian_manager = Contract::from_json(
            self.web3.eth(),
            guardian_manager,
            constants::abis::GUARDIAN_MANAGER,
        )
        .unwrap();
        let result = guardian_manager.query("guardianStorage", (), None, Options::default(), None);

        let guardian_storage = match result.wait() {
            Ok(s) => s,
            Err(_e) => {
                return Err(format!(
                    "unable to fetch guardian storage address for {:?}",
                    self.address
                ))
            }
        };
        let guardian_storage = Contract::from_json(
            self.web3.eth(),
            guardian_storage,
            constants::abis::GUARDIAN_STORAGE,
        )
        .unwrap();
        let result = guardian_storage.query(
            "getGuardians",
            (self.address,),
            None,
            Options::default(),
            None,
        );

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to fetch guardians for {:?}", self.address)),
        }
    }

    pub fn modules(&self) -> Result<Vec<Address>, String> {
        let mut modules = Vec::<Address>::new();

        let filter = FilterBuilder::default()
            .address(vec![self.address])
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
        let result = self.web3.eth().logs(filter);

        let logs = match result.wait() {
            Ok(s) => s,
            Err(_e) => {
                return Err(format!(
                    "unable to fetch modules logs for {:?}",
                    self.address
                ))
            }
        };

        for log in logs.iter() {
            let address = Address::from(log.topics[1]);

            if log.data.eq(&Bytes::from(vec![0; 32])) {
                modules.retain(|&module| module != address);
            } else {
                modules.push(address);
            }
        }

        Ok(modules)
    }

    pub fn balance(&self, token: &Token) -> Result<U256, String> {
        let error = format!(
            "unable to fetch {} balance for {:?}",
            token.symbol, self.address
        );

        if token.address == Address::zero() {
            let result = self.web3.eth().balance(self.address, None);

            match result.wait() {
                Ok(s) => return Ok(s),
                Err(_e) => return Err(error),
            }
        } else {
            let contract =
                Contract::from_json(self.web3.eth(), token.address, constants::abis::ERC20)
                    .unwrap();
            let result =
                contract.query("balanceOf", (self.address,), None, Options::default(), None);
            match result.wait() {
                Ok(s) => return Ok(s),
                Err(_e) => return Err(error),
            }
        }
    }

    pub fn lock(&self) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("unable to fetch accounts")),
        };

        let lock_manager = Address::from_str(&"0bc693480d447ab97aff7aa215d1586f1868cb01").unwrap();
        let lock_manager =
            Contract::from_json(self.web3.eth(), lock_manager, constants::abis::LOCK_MANAGER)
                .unwrap();

        let result = lock_manager.call("lock", (self.address,), accounts[0], Options::default());

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to lock {:?}", self.address)),
        }
    }

    pub fn unlock(&self) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("unable to fetch accounts")),
        };

        let lock_manager = Address::from_str(&"0bc693480d447ab97aff7aa215d1586f1868cb01").unwrap();
        let lock_manager =
            Contract::from_json(self.web3.eth(), lock_manager, constants::abis::LOCK_MANAGER)
                .unwrap();

        let result = lock_manager.call("unlock", (self.address,), accounts[0], Options::default());

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to unlock {:?}", self.address)),
        }
    }
}
