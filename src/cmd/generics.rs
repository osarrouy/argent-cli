use crate::helpers;
use crate::token::Token;
use crate::tui;
use crate::wallet::Wallet;
use dialoguer::Confirmation;
use std::process;
use web3::api::Web3;

pub fn ens<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

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
    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let wallet = Wallet::new(address, &web3);

    let owner = wallet.owner().unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    tui::header("owner");
    tui::address(owner);
    tui::end();
}

pub fn balance<T: web3::Transport>(wallet: &str, symbol: &str, web3: Web3<T>) {
    let token = Token::from_symbol(symbol).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let wallet = Wallet::new(address, &web3);

    let balance = wallet.balance(&token).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    tui::header("balance");
    tui::info(format!("{:?} {}", token.to_decimals(balance), token.symbol));
    tui::end();
}

pub fn lock<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let wallet = Wallet::new(address, &web3);

    if Confirmation::new()
        .with_text("-[ are you sure you want to lock this wallet?")
        .default(false)
        .interact()
        .unwrap()
    {
        let tx = wallet.lock().unwrap_or_else(|e| {
            tui::error(e);
            process::exit(1);
        });

        tui::header_with_state("lock", "ongoing");
        tui::info(format!("see https://etherscan.io/tx/{:?}", tx));
    }
}

pub fn unlock<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let wallet = Wallet::new(address, &web3);

    if Confirmation::new()
        .with_text("-[ are you sure you want to unlock this wallet?")
        .default(false)
        .interact()
        .unwrap()
    {
        let tx = wallet.unlock().unwrap_or_else(|e| {
            tui::error(e);
            process::exit(1);
        });

        tui::header_with_state("unlock", "ongoing");
        tui::info(format!("see https://etherscan.io/tx/{:?}", tx));
    }
}
