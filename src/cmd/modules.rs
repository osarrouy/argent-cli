use web3::api::Web3;
use crate::helpers;
use crate::modules;
use crate::tui;
use crate::wallet::Wallet;

pub fn ls<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
  let mut list = Vec::<String>::new();

  let address = helpers::to_address(wallet, &web3);
  let wallet = Wallet::new(address, &web3);
  
  let modules = wallet.modules().unwrap_or_else(|e| {
    tui::error(e);
    std::process::exit(1);
  });

  for module in modules.iter() {
    list.push(format!("{:?} | {}", module, modules::name(&module)));
  }

  tui::header("modules");
  tui::list(&list);
  tui::end();
}