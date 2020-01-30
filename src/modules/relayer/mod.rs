use ethabi::Bytes as ABIBytes;
use web3::api::Web3;
use web3::futures::Future;
use web3::types::{Address, BlockId, BlockNumber, H256, U256};

pub trait Relayer<T: web3::Transport> {
    fn execute(
        &self,
        wallet: Address,
        data: ABIBytes,
        nonce: U256,
        signature: &[u8],
    ) -> Result<H256, String>;

    fn web3(&self) -> &Web3<T>;

    fn nonce(&self) -> Result<U256, String> {
        let block = match self
            .web3()
            .eth()
            .block(BlockId::Number(BlockNumber::Latest))
            .wait()
        {
            Ok(s) => s.unwrap(),
            Err(_e) => return Err(format!("unable to fetch last block")),
        };

        let nonce = format!("{:?}{:?}", block.number.unwrap(), block.timestamp);
        let nonce = U256::from_dec_str(&nonce).unwrap();

        Ok(nonce)
    }

    fn value(&self) -> U256 {
        U256::from(0u32)
    }

    fn gas_price(&self) -> U256 {
        U256::from(0u32)
    }

    fn gas_limit(&self) -> U256 {
        U256::from(250000u32)
    }
}
