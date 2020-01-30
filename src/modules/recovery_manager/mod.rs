use crate::constants;
use crate::helpers;
use crate::modules::Relayer;
use std::str::FromStr;
use web3::api::Web3;
use web3::contract::Contract;
use web3::contract::Options;
use web3::futures::Future;
use web3::types::{Address, Bytes, CallRequest, TransactionRequest, H256, U256};

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

        let hash_sign = match self.hash_sign(wallet, data.clone(), nonce) {
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
        let hash_sign = match self.hash_sign(wallet, data.clone(), nonce) {
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
            Address::from_str(&"c693F807D239cF7923A2eEE48db214e22BF96dBe").unwrap();

        let function = erc_1077_hash_abi.function("getSignHash").unwrap();
        let params: [ethabi::Token; 7] = [
            ethabi::Token::Address(self.address),
            ethabi::Token::Address(wallet),
            ethabi::Token::Uint(self.value()),
            ethabi::Token::Bytes(data),
            ethabi::Token::Uint(nonce),
            ethabi::Token::Uint(self.gas_price()),
            ethabi::Token::Uint(self.gas_limit()),
        ];
        let data = function.encode_input(&params).unwrap();
        let result = self.web3.eth().call(
            CallRequest {
                from: None,
                to: erc_1077_hash_address,
                gas: options.gas,
                gas_price: options.gas_price,
                value: options.value,
                data: Some(Bytes(data)),
            },
            None.into(),
        );

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!("unable to fetch hash sign")),
        }
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
