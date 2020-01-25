use web3::api::Web3;
use crate::helpers;
use crate::modules;
use crate::tui;
use crate::wallet::Wallet;

pub fn info<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
  let address = helpers::to_address(wallet, &web3);
  let wallet = Wallet::new(address, &web3);

  let ens = wallet.ens();
  let owner = wallet.owner();
  let modules = wallet.modules();
  let guardians = wallet.guardians();
  let mut modules_list = Vec::<String>::new();
  for module in modules.iter() {
    modules_list.push(format!("{:?} | {}", module, modules::name(&module)));
  }
  let mut guardians_list = Vec::<String>::new();
  for guardian in guardians.iter() {
    guardians_list.push(format!("{:?}", guardian));
  }

  tui::header("address");
  tui::address(address);
  tui::header("ens");
  tui::info(ens);
  tui::header("owner");
  tui::address(owner);
  tui::header("guardians");
  tui::list(&guardians_list);
  tui::header("modules");
  tui::list(&modules_list);
  tui::end();
}

pub fn ens<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
  let address = helpers::to_address(wallet, &web3);
  let wallet = Wallet::new(address, &web3);

  let ens = wallet.ens();

  tui::header("address");
  tui::address(address);
  tui::header("ens");
  tui::info(ens);
  tui::end();
}

pub fn owner<T: web3::Transport>(wallet: &str, web3: Web3<T>) {  
  let address = helpers::to_address(wallet, &web3);
  let wallet = Wallet::new(address, &web3);

  let owner = wallet.owner();

  tui::header("address");
  tui::address(address);
  tui::header("owner");
  tui::address(owner);
  tui::end();
}