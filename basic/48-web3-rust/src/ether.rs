

use ethers::{prelude::*, utils::Ganache};
use eyre::{ContextCompat, Result};
use hex::ToHex;
use std::{convert::TryFrom, sync::Arc, time::Duration};

// Generate the type-safe contract bindings by providing the json artifact
// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact:
// `{"abi": [..], "bin": "..."}`  or `{"abi": [..], "bytecode": {"object": "..."}}`
// this will embedd the bytecode in a variable `GREETER_BYTECODE`
abigen!(Greeter, "ethers-contract/tests/solidity-contracts/greeter.json",);

#[tokio::main]
async fn main() -> Result<()> {
        // Spawn a ganache instance
        let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
        let ganache = Ganache::new().mnemonic(mnemonic).spawn();


        // A provider is an Ethereum JsonRPC client
       // let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
       let provider = Provider::<Http>::try_from(
        "https://goerli.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").expect("could not instantiate HTTP Provider");
    

        // Generate a wallet of random numbers
        //let wallet = LocalWallet::new(&mut thread_rng());
        let wallet: LocalWallet = ganache.keys()[0].clone().into();
        let wallet_address: String = wallet.address().encode_hex();
        println!("Default wallet address: {}", wallet_address);

        // sign message with wallet and print out signature produced.
        let signature = wallet.sign_message(message).await?;
        println!("Produced signature {}", signature);

        // Query the balance of our account
        let first_balance = provider.get_balance(wallet.address, None).await?;
        println!("Wallet first address balance: {}", first_balance);

        // Query the blance of some random account
        let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
        let other_address = other_address_hex.parse::<Address>()?;
        let other_balance = provider.get_balance(other_address, None).await?;
        println!(
            "Balance for address {}: {}",
            other_address_hex, other_balance
        );

        // Create a transaction to transfer 1000 wei to `other_address`
        let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);
        // Send the transaction and wait for receipt
        let receipt = provider
            .send_transaction(tx, None)
            .await?
            .log_msg("Pending transfer")
            .confirmations(1) // number of confirmations required
            .await?
            .context("Missing receipt")?;

        println!(
            "TX mined in block {}",
            receipt.block_number.context("Can not get block number")?
        );
        println!(
            "Balance of {} {}",
            other_address_hex,
            provider.get_balance(other_address, None).await?
        );

        Ok(())


}