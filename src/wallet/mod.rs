use crate::constants;
use crate::helpers;
use crate::modules::RecoveryManager;
use crate::token::Token;
use std::str::FromStr;
use web3::api::Web3;
use web3::contract::tokens::Tokenize;
use web3::contract::Contract;
use web3::contract::Options;
use web3::futures::Future;
use web3::types::{
    Address, Bytes, CallRequest, FilterBuilder, TransactionRequest, H256, H520, U256,
};

// use ethabi::Token;

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

    pub fn cancel_recovery(&self) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("unable to fetch accounts")),
        };

        let recovery_manager = RecoveryManager::new(self.web3);

        // 1. encode_cancel_recovery
        // 2. update nonce
        // 3. get signed hash [function of encode_cancel_recovery, nonce and gas related things]
        // 4. sign the shit
        // 5. get the prefixed sign hash
        // 6. call execute

        recovery_manager.cancel_recovery(self.address)
    }

    
    // pub fn initialize_recovery(&self, owner: Address) -> Result<(), String> {
    //     let options = Options::default();
    //     // fetch accounts
    //     let accounts = match self.web3.eth().accounts().wait() {
    //         Ok(s) => s,
    //         Err(_e) => return Err(String::from("unable to fetch accounts")),
    //     };

    //     // initialize recovery manager artifacts
    //     let recovery_manager_abi =
    //         ethabi::Contract::load(constants::abis::RECOVERY_MANAGER).unwrap();
    //     let recovery_manager_address =
    //         Address::from_str(&"dfa1468d07fc86840a6eb53e0e65cebde81d1af9").unwrap();
    //     let recovery_manager = Contract::from_json(
    //         self.web3.eth(),
    //         recovery_manager_address,
    //         constants::abis::RECOVERY_MANAGER,
    //     )
    //     .unwrap();

    //     // fetch nonce
    //     let result =
    //         recovery_manager.query("getNonce", (self.address,), None, Options::default(), None);
    //     let nonce: U256 = match result.wait() {
    //         Ok(s) => s,
    //         Err(_e) => {
    //             return Err(format!(
    //                 "unable to fetch recovery nonce for {:?}",
    //                 self.address
    //             ))
    //         }
    //     };

    //     let nonce = nonce.checked_add(U256::from(1)).unwrap();
    //     println!("Nonce {:#?}", nonce);

    //     // let nonce = U256::from(318905930u32);

    //     // prepare to data bytes to be executed
    //     let function = recovery_manager_abi.function("executeRecovery").unwrap();
    //     let params: [ethabi::Token; 2] = [
    //         ethabi::Token::Address(self.address),
    //         ethabi::Token::Address(owner),
    //     ];
    //     let encoded: ethabi::Bytes = function.encode_input(&params).unwrap();
    //     let encoded_2: ethabi::Bytes = function.encode_input(&params).unwrap();
    //     let encoded_3: ethabi::Bytes = function.encode_input(&params).unwrap();

    //     // let encoded_2: ethabi::Bytes = function.encode_input(&params).unwrap();

    //     // let encoded_bytes = Bytes::from(function.encode_input(&params).unwrap());

    //     // println!("Encoded {:#?}", encoded);
    //     let value = web3::types::U256::from(0u32);
    //     let gas_price = web3::types::U256::from(0u32);
    //     let gas_limit = web3::types::U256::from(250017);

    //     // fetch the ERC1077 hash to sign
    //     let erc_1077_hash_abi = ethabi::Contract::load(constants::abis::ERC1077_HASH).unwrap();
    //     let erc_1077_hash_address =
    //         Address::from_str(&"c693F807D239cF7923A2eEE48db214e22BF96dBe").unwrap();
    //     // Address::from_str(&"cDB81eAB37f6bb9Be814985cb0197141ddE3d045").unwrap();
    //     let function = erc_1077_hash_abi.function("getSignHash").unwrap();
    //     let params: [ethabi::Token; 7] = [
    //         ethabi::Token::Address(recovery_manager_address),
    //         ethabi::Token::Address(self.address),
    //         ethabi::Token::Uint(value),
    //         ethabi::Token::Bytes(encoded_3),
    //         ethabi::Token::Uint(nonce),
    //         ethabi::Token::Uint(gas_price),
    //         ethabi::Token::Uint(gas_limit),
    //     ];
    //     let data = function.encode_input(&params).unwrap();
    //     let hash_to_sign = self
    //         .web3
    //         .eth()
    //         .call(
    //             CallRequest {
    //                 from: None,
    //                 to: erc_1077_hash_address,
    //                 gas: options.gas,
    //                 gas_price: options.gas_price,
    //                 value: options.value,
    //                 data: Some(Bytes(data)),
    //             },
    //             None.into(),
    //         )
    //         .wait()
    //         .unwrap();

    //     // prefixed signed hash
    //     let function = erc_1077_hash_abi.function("getPrefixedSignHash").unwrap();
    //     let data = function.encode_input(&params).unwrap();
    //     let prefixed_hash_to_sign = self
    //         .web3
    //         .eth()
    //         .call(
    //             CallRequest {
    //                 from: None,
    //                 to: erc_1077_hash_address,
    //                 gas: options.gas,
    //                 gas_price: options.gas_price,
    //                 value: options.value,
    //                 data: Some(Bytes(data)),
    //             },
    //             None.into(),
    //         )
    //         .wait()
    //         .unwrap();

    //     // sign the ERC1077hash
    //     let signature = self
    //         .web3
    //         .eth()
    //         .sign(accounts[0], hash_to_sign.clone())
    //         // .sign(accounts[0], Bytes::from(encoded))
    //         .wait()
    //         .unwrap();
    //     let signature = signature.as_bytes();
    //     // println!("Sig {:#?}", signature);

    //     // let gas_price = web3::types::U256::from(0u32);
    //     // let gas_limit = web3::types::U256::from(250u32);
    //     // prepare and send the `execute` transaction
    //     let function = recovery_manager_abi.function("execute").unwrap();
    //     let params: [ethabi::Token; 6] = [
    //         ethabi::Token::Address(self.address),
    //         ethabi::Token::Bytes(encoded_2),
    //         ethabi::Token::Uint(nonce),
    //         ethabi::Token::Bytes(signature.to_vec()),
    //         ethabi::Token::Uint(gas_price),
    //         ethabi::Token::Uint(gas_limit),
    //     ];
    //     let encoded_4 = function.encode_input(&params).unwrap();

    //     println!("Hash to sign {:#?}", hash_to_sign.clone());
    //     // testttttt
    //     let function = erc_1077_hash_abi.function("recoverSigner").unwrap();
    //     let params: [ethabi::Token; 3] = [
    //         ethabi::Token::FixedBytes(prefixed_hash_to_sign.clone().0),
    //         ethabi::Token::Bytes(signature.to_vec()),
    //         ethabi::Token::Uint(web3::types::U256::from(0u32)),
    //     ];
    //     let data = function.encode_input(&params).unwrap();

    //     let recovery = self
    //         .web3
    //         .eth()
    //         .call(
    //             CallRequest {
    //                 from: None,
    //                 to: erc_1077_hash_address,
    //                 gas: options.gas,
    //                 gas_price: options.gas_price,
    //                 value: options.value,
    //                 data: Some(Bytes(data)),
    //             },
    //             None.into(),
    //         )
    //         .wait()
    //         .unwrap();
    //     let add = Address::from_str(&"b71d2d88030a00830c3d45f84c12cc8aaf6857a5").unwrap();
    //     println!("Address {:#?}", add.as_bytes());

    //     println!("Recovery {:#?}", recovery);
    //     // testttttt

    //     // println!("Encoded {:#?}", encoded);

    //     // let b = Bytes::from(vec![0u8]);

    //     // let Options {
    //     //     gas,
    //     //     gas_price,
    //     //     value,
    //     //     nonce,
    //     //     condition,
    //     // } = Options::default();

    //     let opt = Options::default();
    //     // let Options::default()

    //     let tx = self
    //         .web3
    //         .eth()
    //         .send_transaction(TransactionRequest {
    //             from: accounts[0],
    //             to: Some(recovery_manager_address),
    //             gas: opt.gas,
    //             gas_price: opt.gas_price,
    //             value: opt.value,
    //             nonce: opt.nonce,
    //             data: Some(Bytes(encoded_4)),
    //             condition: opt.condition,
    //         })
    //         .wait();

    //     println!("TX {:#?}", tx);
    //     // self.abiy
    //     //     .function(func)
    //     //     .and_then(|function| function.encode_input(&params.into_tokens()))
    //     //     .map(move |data| {
    //     //         let Options {
    //     //             gas,
    //     //             gas_price,
    //     //             value,
    //     //             nonce,
    //     //             condition,
    //     //         } = options;

    //     //         self.eth
    //     //             .send_transaction(TransactionRequest {
    //     //                 from,
    //     //                 to: Some(self.address),
    //     //                 gas,
    //     //                 gas_price,
    //     //                 value,
    //     //                 nonce,
    //     //                 data: Some(Bytes(data)),
    //     //                 condition,
    //     //             })
    //     //             .into()
    //     //     })
    //     //     .unwrap_or_else(Into::into)

    //     // BaseWallet _wallet,
    //     // bytes calldata _data,
    //     // uint256 _nonce,
    //     // bytes calldata _signatures,
    //     // uint256 _gasPrice,
    //     // uint256 _gasLimit

    //     // let gas_price = web3::types::U256::from(0u32);
    //     // let gas_limit = web3::types::U256::from(0u32);

    //     // // self.address, encoded, nonce, signature, gas_price, gas_limit,
    //     // let result = recovery_manager.call("execute", (self.address, encoded,), accounts[0], Options::default());

    //     // match result.wait() {
    //     //     Ok(s) => Ok(()),
    //     //     // Err(_e) => Err(format!("unable to initialize recovery for {:?}", self.address)),
    //     //     Err(_e) => Err(format!("{:?}", _e)),

    //     // }

    //     Ok(())
    // }
}
