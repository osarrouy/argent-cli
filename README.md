
<h1 align="center">
  <br>
  <a href="http://www.argent.xyz"><img src="https://raw.githubusercontent.com/osarrouy/argent-cli/master/.github/logo.png" alt="argent-cli" width="200"></a>
  <br>
  argent-cli
  <br>
</h1>

<h4 align="center">A Rust rescue CLI for the <a href="http://www.argent.xyz" target="_blank">Argent wallet</a>.</h4>


<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#commands">Commands</a> •
  <a href="#license">License</a>
</p>

## Disclaimer

The `argent-cli` is not developped by Argent. It is a community project. Use at your own risk.

## Key Features

* Fetch the owner  of an Argent wallet
* Fetch the ENS name of an Argent wallet
* Fetch the list of enabled modules on an Argent wallet 
* Fetch the list of guardians of an Argent wallet
* Fetch balances of an Argent wallet
* Lock an Argent wallet [soon]
* Recover an Argent wallet [soon]

## How To Use

You need a [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your computer.

```bash
# clone this repository
$ git clone https://github.com/osarrouy/argent-cli

# cd into the repository
$ cd argent-cli

# compile
$ cargo build --release

# [optionnal] copy the binary into your PATH
$ cp target/release/argent /usr/local/bin
```

## Commands
```bash
$ argent --help


   / \   _ __ __ _  ___ _ __ | |_ 
  / _ \ | '__/ _` |/ _ | '_ \| __|
 / ___ \| | | (_| |  __| | | | |_ 
/_/   \_|_|  \__, |\___|_| |_|\__| -[ CLI
             |___/     
   
argent 1.0
Olivier Sarrouy <osarrouy@protonmail.com>
A CLI for the Argent wallet

USAGE:
    argent <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    balance      Prints the balance of a wallet
    ens          Prints the address and ENS name of a wallet
    guardians    Guardians related commands
    help         Prints this message or the help of the given subcommand(s)
    modules      Modules related commands
    owner        Prints the owner of a wallet
```

## License

MIT