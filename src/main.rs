mod cmd;
mod constants;
mod ens;
mod helpers;
mod modules;
mod token;
mod tui;
mod wallet;

use clap::{App, AppSettings, Arg};
use std::process;

fn main() {
    const WALLET_ARG_NAME: &'static str = "wallet";
    const WALLET_ARG_HELP: &'static str = "Address or ENS name of the wallet";

    tui::figlet();

    let (_eloop, transport) = web3::transports::Http::new(
        "https://mainnet.infura.io/v3/96b017a91a2042148592487564f273b1",
    )
    .unwrap_or_else(|_e| {
        tui::error("Unable to connect to Ethereum endpoint".to_string());
        process::exit(1)
    });
    let web3 = web3::Web3::new(transport);

    let matches = App::new("argent")
        .about("A CLI for the Argent wallet")
        .version("1.0")
        .author("Olivier Sarrouy <osarrouy@protonmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("ens")
                .about("Prints the address and ENS name of a wallet")
                .arg(
                    Arg::with_name(WALLET_ARG_NAME)
                        .help(WALLET_ARG_HELP)
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("owner").about("Prints the owner of a wallet").arg(
                Arg::with_name(WALLET_ARG_NAME)
                    .help(WALLET_ARG_HELP)
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("guardians")
                .about("Guardians related commands")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("ls")
                        .about("Prints the list of guardians of a wallet")
                        .arg(
                            Arg::with_name(WALLET_ARG_NAME)
                                .help(WALLET_ARG_HELP)
                                .index(1)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("modules")
                .about("Modules related commands")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("ls")
                        .about("Prints the list of enabled modules of a wallet")
                        .arg(
                            Arg::with_name(WALLET_ARG_NAME)
                                .help(WALLET_ARG_HELP)
                                .index(1)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("balance")
                .about("Prints the balance of a wallet")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name(WALLET_ARG_NAME)
                        .help(WALLET_ARG_HELP)
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("token")
                        .help("Address or symbol of the token")
                        .index(2)
                        .required(true)
                        .default_value("ETH"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("ens", Some(args)) => {
            cmd::generics::ens(args.value_of(WALLET_ARG_NAME).unwrap(), web3);
        }
        ("owner", Some(args)) => {
            cmd::generics::owner(args.value_of(WALLET_ARG_NAME).unwrap(), web3);
        }
        ("modules", Some(params)) => match params.subcommand() {
            ("ls", Some(args)) => {
                cmd::modules::ls(args.value_of(WALLET_ARG_NAME).unwrap(), web3);
            }
            _ => unreachable!(),
        },
        ("guardians", Some(params)) => match params.subcommand() {
            ("ls", Some(args)) => {
                cmd::guardians::ls(args.value_of(WALLET_ARG_NAME).unwrap(), web3);
            }
            _ => unreachable!(),
        },
        ("balance", Some(args)) => {
            cmd::generics::balance(
                args.value_of(WALLET_ARG_NAME).unwrap(),
                args.value_of("token").unwrap(),
                web3,
            );
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
