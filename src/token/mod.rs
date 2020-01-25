use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use web3::types::{Address, U256};

lazy_static! {
    static ref TOKENS: HashMap<&'static str, Token> = {
        let mut tokens = HashMap::new();

        let eth: Token = Token {
            symbol: "ETH",
            decimals: 18,
            address: Address::zero(),
        };
        let dai: Token = Token {
            symbol: "DAI",
            decimals: 18,
            address: Address::from_str(&"6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(),
        };
        let sai: Token = Token {
            symbol: "SAI",
            decimals: 18,
            address: Address::from_str(&"89d24a6b4ccb1b6faa2625fe562bdd9a23260359").unwrap(),
        };
        let wbtc: Token = Token {
            symbol: "WBTC",
            decimals: 8,
            address: Address::from_str(&"2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(),
        };
        let ant: Token = Token {
            symbol: "ANT",
            decimals: 18,
            address: Address::from_str(&"960b236A07cf122663c4303350609A66A7B288C0").unwrap(),
        };

        tokens.insert("ETH", eth);
        tokens.insert("DAI", dai);
        tokens.insert("SAI", sai);
        tokens.insert("WBTC", wbtc);
        tokens.insert("ANT", ant);

        tokens
    };
}

#[derive(Clone, Debug)]
pub struct Token {
    pub address: Address,
    pub symbol: &'static str,
    pub decimals: u32,
}

impl Token {
    pub fn from_symbol(symbol: &str) -> Result<&Token, String> {
        match TOKENS.get(symbol) {
            Some(s) => Ok(s),
            None => Err(format!("unknown token {}", symbol)),
        }
    }

    pub fn to_decimals(&self, value: U256) -> f64 {
        let value = value.as_u128();
        let value = value as f64;

        value / (10u128.pow(self.decimals) as f64)
    }
}
