use crate::helpers;
use crate::token::Token;
use crate::tui;
use crate::wallet::Wallet;
use std::process;
use web3::api::Web3;

pub fn ens<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3);
    let wallet = Wallet::new(address, &web3);

    let ens = wallet.ens().unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    tui::header("address");
    tui::address(address);
    tui::header("ens");
    tui::info(ens);
    tui::end();
}

pub fn owner<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3);
    let wallet = Wallet::new(address, &web3);

    let owner = wallet.owner().unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    tui::header("owner");
    tui::address(owner);
    tui::end();
}

pub fn balance<T: web3::Transport>(wallet: &str, token: &str, web3: Web3<T>) {
    let symbol = token;
    let token = Token::from_symbol(symbol).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    // let dai = helpers::to_address(&"0x6B175474E89094C44Da98b954EedeAC495271d0F", &web3);

    let address = helpers::to_address(wallet, &web3);
    let wallet = Wallet::new(address, &web3);

    let balance = wallet.balance(token.address).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    // let balance = tokens::to_decimals(balance, token.decimals);

    tui::header("balance");
    tui::info(format!("{:?} {}", token.to_decimals(balance), token.symbol));
    tui::end();
}
