use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use web3::types::Address;

lazy_static! {
    static ref MODULES: HashMap<Address, &'static str> = {
        let mut modules = HashMap::new();

        modules.insert(
            Address::from_str(&"FF5A7299ff6f0fbAad9b38906b77d08c0FBdc9A7").unwrap(),
            "GuardianManager",
        );
        modules.insert(
            Address::from_str(&"0BC693480d447AB97AfF7aa215D1586f1868Cb01").unwrap(),
            "LockManager",
        );
        modules.insert(
            Address::from_str(&"dfa1468D07Fc86840A6EB53E0e65CEBDE81D1af9").unwrap(),
            "RecoveryManager",
        );
        modules.insert(
            Address::from_str(&"ed0DA07AAB7257Df53Efc4DfC076745744138Ed9").unwrap(),
            "TokenExchanger",
        );
        modules.insert(
            Address::from_str(&"1848e646Bba45174f4044443719Db6E5E6Cf5D66").unwrap(),
            "NftTransfer",
        );
        modules.insert(
            Address::from_str(&"963F86DA34Cf2CE619d4B8e5cE96577943f95B6b").unwrap(),
            "MakerManager",
        );
        modules.insert(
            Address::from_str(&"A5d7d68D7975e89FEb240f42feD1D77bb71b1cAF").unwrap(),
            "CompoundManager",
        );
        modules.insert(
            Address::from_str(&"5388b0f8106BDE37DC6982b4Ba5771d2E8D9dc42").unwrap(),
            "UniswapManager",
        );
        modules.insert(
            Address::from_str(&"2B6D87F12B106E1D3fA7137494751566329d1045").unwrap(),
            "TransferManager",
        );
        modules.insert(
            Address::from_str(&"cd23f51912ea8Fff38815f628277731C25c7Fb02").unwrap(),
            "ApprovedTransfer",
        );
        modules.insert(
            Address::from_str(&"7557f4199aa99e5396330BaC3b7bDAa262CB1913").unwrap(),
            "MakerV2Manager",
        );
        modules.insert(
            Address::from_str(&"0045684552109f8551CC5c8aa7B1f52085adFf47").unwrap(),
            "ApprovedTransfer",
        );
        modules.insert(
            Address::from_str(&"4DD68a6C27359E5640Fa6dCAF13631398C5613f1").unwrap(),
            "ModuleManager",
        );
        modules.insert(
            Address::from_str(&"df6767A7715381867738cF211290F61697ecd938").unwrap(),
            "TokenTransfer",
        );

        modules
    };
}

pub fn name(address: &Address) -> &'static str {
    let name: &'static str = MODULES.get(address).unwrap_or(&"Unknown module");

    &*name
}
