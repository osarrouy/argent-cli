use crate::constants;
use crate::helpers;
use crate::token::Token;
use lazy_static::lazy_static;
use std::str::FromStr;
use web3::api::Web3;
use web3::contract::tokens::Tokenize;
use web3::contract::Contract;
use web3::contract::Options;
use web3::futures::Future;
use web3::types::{
    Address, Bytes, CallRequest, FilterBuilder, BlockId, BlockNumber, TransactionRequest, H256, H520, U256,
};
// use web3::types::block::BlockId;
// use ethabi::Token;

lazy_static! {
    static ref value: U256 = web3::types::U256::from(0u32);
    static ref gas_price: U256 = web3::types::U256::from(0u32);
    static ref gas_limit: U256 = web3::types::U256::from(250000);
}

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

    pub fn nonce(&self, wallet: Address) -> Result<U256, String> {
        let result = self
            .contract
            .query("getNonce", (wallet,), None, Options::default(), None);

        match result.wait() {
            Ok(s) => Ok(s),
            Err(_e) => Err(format!(
                "unable to fetch recovery manager nonce for {:?}",
                wallet
            )),
        }
    }

    pub fn updated_nonce(&self, wallet: Address) -> Result<U256, String> {
        // let nonce = match self.nonce(wallet) {
        //     Ok(s) => s,
        //     Err(e) => return Err(e),
        // };

        // match nonce.checked_add(U256::from(1)) {
        //     Some(s) => Ok(s),
        //     None => Err(format!(
        //         "unable to update recovery manager nonce for {:?}",
        //         wallet
        //     )),
        // }

        // now = SystemTime::now()
        // {block number}{timestamp}
        // let bn = self.web3.eth().block_number().wait().unwrap();

        let block = match self.web3.eth().block(BlockId::Number(BlockNumber::Latest)).wait() {
          Ok(s) => s.unwrap(),
          Err(_e) => return Err(format!("unable to fetch last block"))
        };
      

        //   println!("{}", block.number.unwrap()); // u64 need to convert to u128 //
        //   println!("{}", block.timestamp); // u256 needs to convert to u128
        //   println!("{}", block.timestamp.low_u128()); // u256 needs to convert to u128

        // let block_number = block.number.unwrap().low_u64();
        // let block_number = block_number as u128;
        // let timestamp = block.timestamp.low_u128();

        let nonce = format!("{:?}{:?}", block.number.unwrap(), block.timestamp);
        let nonce = U256::from_dec_str(&nonce).unwrap();
        
        Ok(nonce)
    }

    pub fn hash_sign(
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
            ethabi::Token::Uint(*value),
            ethabi::Token::Bytes(data),
            ethabi::Token::Uint(nonce),
            ethabi::Token::Uint(*gas_price),
            ethabi::Token::Uint(*gas_limit),
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

    pub fn execute(
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
            ethabi::Token::Uint(*gas_price),
            ethabi::Token::Uint(*gas_limit),
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

    pub fn encode_initialize_recovery(&self, wallet: Address, new_owner: Address) -> ethabi::Bytes {
        let function = self.abi.function("executeRecovery").unwrap();
        let params: [ethabi::Token; 2] = [
            ethabi::Token::Address(wallet),
            ethabi::Token::Address(new_owner),
        ];

        function.encode_input(&params).unwrap()
    }

    pub fn encode_cancel_recovery(&self, wallet: Address) -> ethabi::Bytes {
        let function = self.abi.function("cancelRecovery").unwrap();
        let params: [ethabi::Token; 1] = [ethabi::Token::Address(wallet)];

        function.encode_input(&params).unwrap()
    }

    pub fn initialize(&self, wallet: Address, new_owner: Address) -> Result<H256, String> {
        let accounts = match self.web3.eth().accounts().wait() {
            Ok(s) => s,
            Err(_e) => return Err(String::from("modules :: recovery_manager :: initialize :: unable to fetch accounts")),
        };

        let nonce = match self.updated_nonce(wallet) {
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

        let nonce = match self.updated_nonce(wallet) {
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
}
