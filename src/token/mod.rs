use std::str::FromStr;
use web3::types::{Address, U256};

#[derive(Clone, Debug)]
pub struct Token {
    pub address: Address,
    pub symbol: &'static str,
    pub decimals: u32,
}

impl Token {
    pub fn from_symbol(symbol: &str) -> Result<Token, String> {
        match symbol {
            "eth" | "ETH" => Ok(Token {
                symbol: "ETH",
                decimals: 18,
                address: Address::zero(),
            }),
            "ant" | "ANT" => Ok(Token {
                symbol: "ANT",
                decimals: 18,
                address: Address::from_str(&"960b236A07cf122663c4303350609A66A7B288C0").unwrap(),
            }),
            "dai" | "DAI" => Ok(Token {
                symbol: "DAI",
                decimals: 18,
                address: Address::from_str(&"6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(),
            }),
            "sai" | "SAI" => Ok(Token {
                symbol: "SAI",
                decimals: 18,
                address: Address::from_str(&"89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(),
            }),
            "wbtc" | "WBTC" => Ok(Token {
                symbol: "WBTC",
                decimals: 8,
                address: Address::from_str(&"2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(),
            }),
            _ => Err(format!("unknown token {}", symbol)),
        }
    }

    pub fn to_decimals(&self, value: U256) -> f64 {
        value.as_u128() as f64 / (10u128.pow(self.decimals) as f64)
    }
}
