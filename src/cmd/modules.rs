use crate::helpers;
use crate::modules;
use crate::tui;
use crate::wallet::Wallet;
use std::process;
use web3::api::Web3;

pub fn ls<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let mut list = Vec::<String>::new();

    let address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });
    let wallet = Wallet::new(address, &web3);

    let modules = wallet.modules().unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    for module in modules.iter() {
        list.push(format!(
            "{:?} | {}",
            module,
            modules::name(&module).unwrap()
        ));
    }

    tui::header("modules");
    tui::list(&list);
    tui::end();
}
