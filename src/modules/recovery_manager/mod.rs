use crate::constants;
use crate::helpers;
use crate::modules::Relayer;
use std::str::FromStr;
use tiny_keccak::{Hasher, Sha3, Keccak};
// extern crate rustc_hex;

// use hex::ToHex;
use rustc_hex::ToHex;

// use rustc_hex::{FromHex, ToHex};
use web3::api::Web3;
use web3::contract::Contract;
use web3::contract::Options;
use web3::futures::Future;
use web3::types::{Address, Bytes, CallRequest, TransactionRequest, H160, H256, U256};

#[derive(Clone, Debug)]
pub struct RecoveryManager<'a, T: web3::Transport> {
    pub address: Address,
    abi: ethabi::Contract,
    contract: web3::contract::Contract<T>,
    web3: &'a Web3<T>,
}

impl<'a, T: web3::Transport> RecoveryManager<'a, T> {
    pub fn new(web3: &'a Web3<T>) -> Self {
        RecoveryManager::<'a, T> {
            address: Address::from_str(&"dfa1468d07fc86840a6eb53e0e65cebde81d1af9").unwrap(),
            abi: ethabi::Contract::load(constants::abis::RECOVERY_MANAGER).unwrap(),
            contract: Contract::from_json(
                web3.eth(),
                Address::from_str(&"dfa1468d07fc86840a6eb53e0e65cebde81d1af9").unwrap(),
                constants::abis::RECOVERY_MANAGER,
            )
            .unwrap(),
            web3,
        }
    }

    pub fn initialize(&self, wallet: Address, new_owner: Address) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => {
                return Err(String::from(
                    "modules :: recovery_manager :: initialize :: unable to fetch accounts",
                ))
            }
        };

        let nonce = match self.nonce() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let data = self.encode_initialize_recovery(wallet, new_owner);

        let hash_sign = match self.hash_sign_2(wallet, data.clone(), nonce) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let signature = helpers::sign(accounts[0], hash_sign, self.web3).unwrap();

        self.execute(wallet, data, nonce, signature.as_bytes())
    }

    pub fn cancel_recovery(&self, wallet: Address) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("unable to fetch accounts")),
        };

        let nonce = match self.nonce() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        println!("Nonce {:?}", nonce);

        let data = self.encode_cancel_recovery(wallet);
        // wallet: Address, value: U256, data: ethabi::Bytes, nonce: U256, gas_price: U256, gas_limit: U256)
        let hash_sign = match self.hash_sign_2(wallet, data.clone(), nonce) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let signature = helpers::sign(accounts[0], hash_sign, self.web3).unwrap();

        self.execute(wallet, data, nonce, signature.as_bytes())
    }

    fn hash_sign(
        &self,
        wallet: Address,
        data: ethabi::Bytes,
        nonce: U256,
    ) -> Result<Bytes, String> {
        let options = Options::default();

        let erc_1077_hash_abi = ethabi::Contract::load(constants::abis::ERC1077_HASH).unwrap();
        let erc_1077_hash_address =
            Address::from_str(&"83649266Ba0a8CAd860403a6532F12c6074BBDAC").unwrap();

        let function = erc_1077_hash_abi.function("getSignHash").unwrap();
        let params: [ethabi::Token; 7] = [
            ethabi::Token::Address(self.address),
            ethabi::Token::Address(wallet),
            ethabi::Token::Uint(self.value()),
            ethabi::Token::Bytes(data.clone()),
            ethabi::Token::Uint(nonce),
            ethabi::Token::Uint(self.gas_price()),
            ethabi::Token::Uint(self.gas_limit()),
        ];

        println!("params {:#?}", params);
        let encoded = function.encode_input(&params).unwrap();

        let result = self.web3.eth().call(
            CallRequest {
                from: None,
                to: erc_1077_hash_address,
                gas: options.gas,
                gas_price: options.gas_price,
                value: options.value,
                data: Some(Bytes(encoded)),
            },
            None.into(),
        );

        let result = match result.wait() {
          Ok(s) => Ok(s),
          Err(_e) => Err(format!("unable to fetch hash sign")),
      };

      println!("From contract {:?}", result.clone().unwrap());

        // begin just get the packed data
        let function = erc_1077_hash_abi.function("justHash").unwrap();
        let encoded = function.encode_input(&params).unwrap();

        let packed = self.web3.eth().call(
            CallRequest {
                from: None,
                to: erc_1077_hash_address,
                gas: options.gas,
                gas_price: options.gas_price,
                value: options.value,
                data: Some(Bytes(encoded.clone())),
            },
            None.into(),
        );

        let packed = packed.wait().unwrap();
        // println!("Data {:?}", data.clone());
        println!("Packed 1 {:?}", packed);

        self.just_hash(packed.0);
        // end - just get the packed data

        // match result.wait() {
        //     Ok(s) => Ok(s),
        //     Err(_e) => Err(format!("unable to fetch hash sign")),
        // }


        // keccak start

        let mut sha3 = Keccak::v256();
        let mut hash = [0u8; 32];

        let mut packed = Vec::<u8>::new();
        
        // let pre = b"0x".as_bytes();
        let prefix = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 25, 0];
        let padding = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        // packed.push(*hex::decode("19").unwrap().first().unwrap());
        // packed.push(*hex::decode("0").unwrap().first().unwrap());
        // packed = [prefix, *self.address.as_bytes()].concat();

        let value: [u8; 32] = self.value().into();
        let gas_price: [u8; 32] = self.gas_price().into();
        let gas_limit: [u8; 32] = self.gas_limit().into();
        let nonce: [u8; 32] = nonce.into();

        // let t = ;
        let t = "0x".as_bytes().to_hex::<String>();

        // packed.extend_from_slice(t.as_bytes());
        // packed.extend_from_slice(&prefix);
        packed.extend_from_slice(b"\x19");
        packed.extend_from_slice(b"\x00");
        packed.extend_from_slice(self.address.as_bytes());
        packed.extend_from_slice(wallet.as_bytes());
        packed.extend_from_slice(&value);
        packed.extend_from_slice(&data);
        packed.extend_from_slice(&nonce);
        packed.extend_from_slice(&gas_price);
        packed.extend_from_slice(&gas_limit);
        // packed.extend_from_slice(&padding); 

        // println!("Packed 2 {:?}", packed);

        // sha3.update(b"0x");

        // let packed2 = packed.as_slice().to_hex::<String>();

        // sha3.update(packed2.as_bytes());
        sha3.update(packed.as_slice());

// 
 

        sha3.finalize(&mut hash);

        println!("hashed {:?}", hash);

        // self.just_hash(packed.to_vec());
  
        

        result
    }


    fn hash_sign_2(
        &self,
        wallet: Address,
        data: ethabi::Bytes,
        nonce: U256,
    ) -> Result<Bytes, String> {
        let mut sha3 = Keccak::v256();
        let mut hash = [0u8; 32];

        let mut packed = Vec::<u8>::new();

        let value: [u8; 32] = self.value().into();
        let gas_price: [u8; 32] = self.gas_price().into();
        let gas_limit: [u8; 32] = self.gas_limit().into();
        let nonce: [u8; 32] = nonce.into();


        packed.extend_from_slice(b"\x19");
        packed.extend_from_slice(b"\x00");
        packed.extend_from_slice(self.address.as_bytes());
        packed.extend_from_slice(wallet.as_bytes());
        packed.extend_from_slice(&value);
        packed.extend_from_slice(&data);
        packed.extend_from_slice(&nonce);
        packed.extend_from_slice(&gas_price);
        packed.extend_from_slice(&gas_limit);

        sha3.update(packed.as_slice());
        sha3.finalize(&mut hash);

        println!("hashed {:?}", hash);

        // self.just_hash(packed.to_vec());
  
        

        Ok(Bytes(hash.to_vec()))
    }

    fn just_hash(&self, data: Vec<u8>) {
        let options = Options::default();

        let erc_1077_hash_abi = ethabi::Contract::load(constants::abis::ERC1077_HASH).unwrap();
        let erc_1077_hash_address =
            Address::from_str(&"83649266Ba0a8CAd860403a6532F12c6074BBDAC").unwrap();

        let function = erc_1077_hash_abi.function("doKeccak").unwrap();
        let params: [ethabi::Token; 1] = [
            ethabi::Token::Bytes(data),
        ];
        let encoded = function.encode_input(&params).unwrap();

        let result = self.web3.eth().call(
            CallRequest {
                from: None,
                to: erc_1077_hash_address,
                gas: options.gas,
                gas_price: options.gas_price,
                value: options.value,
                data: Some(Bytes(encoded)),
            },
            None.into(),
        );

        let result = match result.wait() {
          Ok(s) => Ok(s),
          Err(_e) => Err(format!("unable to fetch just hash sign")),
      };

      println!("Just hash {:?}", result);

    }

    fn encode_initialize_recovery(&self, wallet: Address, new_owner: Address) -> ethabi::Bytes {
        let function = self.abi.function("executeRecovery").unwrap();
        let params: [ethabi::Token; 2] = [
            ethabi::Token::Address(wallet),
            ethabi::Token::Address(new_owner),
        ];

        function.encode_input(&params).unwrap()
    }

    fn encode_cancel_recovery(&self, wallet: Address) -> ethabi::Bytes {
        let function = self.abi.function("cancelRecovery").unwrap();
        let params: [ethabi::Token; 1] = [ethabi::Token::Address(wallet)];

        function.encode_input(&params).unwrap()
    }
}

impl<'a, T: web3::Transport> Relayer<T> for RecoveryManager<'a, T> {
    fn execute(
        &self,
        wallet: Address,
        data: ethabi::Bytes,
        nonce: U256,
        signature: &[u8],
    ) -> Result<H256, String> {
        let options = Options::default();

        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("unable to fetch accounts")),
        };

        let function = self.abi.function("execute").unwrap();
        let params: [ethabi::Token; 6] = [
            ethabi::Token::Address(wallet),
            ethabi::Token::Bytes(data),
            ethabi::Token::Uint(nonce),
            ethabi::Token::Bytes(signature.to_vec()),
            ethabi::Token::Uint(self.gas_price()),
            ethabi::Token::Uint(self.gas_limit()),
        ];
        let encoded = function.encode_input(&params).unwrap();

        let tx = self.web3.eth().send_transaction(TransactionRequest {
            from: accounts[0],
            to: Some(self.address),
            gas: options.gas,
            gas_price: options.gas_price,
            value: options.value,
            nonce: options.nonce,
            data: Some(Bytes(encoded)),
            condition: options.condition,
        });

        match tx.wait() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!(
                "unable to initialize recovery for {:?}: {}",
                self.address, e
            )),
        }
    }

    fn web3(&self) -> &Web3<T> {
        self.web3
    }
}
