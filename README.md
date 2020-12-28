## YecShell - A command line Ycash light client. 

`yecshell` is a command line Ycash light client. It is a fork of [Zecwallet Light CLI](https://github.com/adityapk00/zecwallet-light-cli). To use it, download the latest binary from the releases page and run `./yecshell`

This will launch the interactive prompt. Type `help` to get a list of commands

## Running in non-interactive mode:
You can also run `yecshell` in non-interactive mode by passing the command you want to run as an argument. For example, `yecshell addresses` will list all wallet addresses and exit. 
Run `yecshell help` to see a list of all commands. 

## Privacy 
* While all the keys and transaction detection happens on the client, the server can learn what blocks contain your shielded transactions.
* The server also learns other metadata about you like your ip address etc...
* Also remember that t-addresses don't provide any privacy protection.

## Notes:
* If you want to run your own server, please see [lightwalletd](https://github.com/yecdev/lightwalletd), and then run `./yecshell --server http://127.0.0.1:9067`.
* The log file is in `~/.zcash/yecshell_debug.log`. Wallet is stored in `~/.ycash/yecshell_wallet.dat`

### Note Management
YecShell does automatic note and utxo management, which means it doesn't allow you to manually select which address to send outgoing transactions from. It follows these principles:
* Defaults to sending shielded transactions, even if you're sending to a transparent address
* Sapling funds need at least 5 confirmations before they can be spent
* Can select funds from multiple shielded addresses in the same transaction
* Will automatically shield your transparent funds at the first opportunity
    * When sending an outgoing transaction to a shielded address, Zecwallet-CLI can decide to use the transaction to additionally shield your transparent funds (i.e., send your transparent funds to your own shielded address in the same transaction)

## Compiling from source

#### Pre-requisites
* Rust v1.37 or higher.
    * Run `rustup update` to get the latest version of Rust if you already have it installed
* Rustfmt
    * Run `rustup component add rustfmt` to add rustfmt
* Build tools
    * Please install the build tools for your platform. On Ubuntu `sudo apt install build-essential gcc`

```
git clone https://github.com/adityapk00/zecwallet-light-cli.git
cargo build --release
./target/release/zecwallet-cli
mv -iv ./target/release/zecwallet-cli ./target/release/yecshell
```

## Options
Here are some CLI arguments you can pass to `yecshell`. Please run `yecshell --help` for the full list. 

* `--server`: Connect to a custom zecwallet lightwalletd server. 
    * Example: `./yecshell --server 127.0.0.1:9067`
* `--seed`: Restore a wallet from a seed phrase. Note that this will fail if there is an existing wallet. Delete (or move) any existing wallet to restore from the 24-word seed phrase
    * Example: `./yecshell --seed "twenty four words seed phrase"`
 * `--recover`: Attempt to recover the seed phrase from a corrupted wallet
 
