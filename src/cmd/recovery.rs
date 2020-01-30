use crate::helpers;
use crate::modules::RecoveryManager;
use crate::tui;

use dialoguer::Confirmation;
use std::process;
use web3::api::Web3;

pub fn init<T: web3::Transport>(wallet: &str, owner: &str, web3: Web3<T>) {
    let wallet = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let owner = helpers::to_address(owner, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let recovery_manager = RecoveryManager::new(&web3);

    if Confirmation::new()
        .with_text("-[ are you sure you want to initialize the recovery of this wallet?")
        .default(false)
        .interact()
        .unwrap()
    {
        let tx = recovery_manager
            .initialize(wallet, owner)
            .unwrap_or_else(|e| {
                tui::error(e);
                process::exit(1);
            });

        tui::header_with_state("recovery initialized", "ongoing");
        tui::info(format!("see https://etherscan.io/tx/{:?}", tx));
    }
}

pub fn cancel<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let wallet = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    let recovery_manager = RecoveryManager::new(&web3);

    if Confirmation::new()
        .with_text("-[ are you sure you want to cancel the recovery of this wallet?")
        .default(false)
        .interact()
        .unwrap()
    {
        let tx = recovery_manager
            .cancel_recovery(wallet)
            .unwrap_or_else(|e| {
                tui::error(e);
                process::exit(1);
            });

        tui::header_with_state("recovery cancelled", "ongoing");
        tui::info(format!("see https://etherscan.io/tx/{:?}", tx));
    };

    // tui::header("guardians");
    // tui::list(&list);
    // tui::end();
}

pub fn finalize<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
    let _address = helpers::to_address(wallet, &web3).unwrap_or_else(|e| {
        tui::error(e);
        process::exit(1);
    });

    println!("finalize {}", wallet);

    // tui::header("guardians");
    // tui::list(&list);
    // tui::end();
}
