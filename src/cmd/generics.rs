use crate::helpers;
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
