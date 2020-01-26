use crate::helpers;
use crate::tui;
use crate::wallet::Wallet;
use std::process;
use web3::api::Web3;

pub fn ls<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });
    let wallet = Wallet::new(address, &web3);

    let guardians = wallet.guardians().unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let mut list = Vec::<String>::new();
    for guardian in guardians.iter() {
        list.push(format!("{:?}", guardian));
    }

    tui::header("guardians");
    tui::list(&list);
    tui::end();
}
