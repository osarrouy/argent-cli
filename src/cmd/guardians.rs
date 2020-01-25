use web3::api::Web3;
use crate::helpers;
use crate::tui;
use crate::wallet::Wallet;

pub fn ls<T: web3::Transport>(wallet: &str, web3: Web3<T>) {
  let address = helpers::to_address(wallet, &web3);
  let wallet = Wallet::new(address, &web3);
  
  let guardians = wallet.guardians();
  let mut list = Vec::<String>::new();
  for guardian in guardians.iter() {
    list.push(format!("{:?}", guardian));
  }

  tui::header("guardians");
  tui::list(&list);
  tui::end();
}