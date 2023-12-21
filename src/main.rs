use bls::{PublicKey, SecretKey};
use clap::Parser;
use color_eyre::{
    eyre::{eyre, Result, WrapErr},
    Help,
};
use xor_name::XorName;

use sn_client::{Client, WalletClient};
use sn_registers::RegisterAddress;
use sn_transfers::LocalWallet;

#[derive(Parser, Debug)]
struct Opt {
    #[clap(long)]
    user: String,
}

const HONEYCOMB_PUBLIC_KEY: &str = "98b0807cde78204259d89a7ca510fe41a763774117efbdb0134c125b0a730567e8fb7699a7f6c686e7e5a013bfe6078d";

#[tokio::main]
async fn main() -> Result<()> {
    //
    // Create client simulating end-user
    // with random secret key for now
    // (so different end-user on each run)
    //
    let end_user_sk = SecretKey::random();
    let end_user_client = Client::new(end_user_sk, None, false, None).await?;
    println!(
        "Created end-user client with public key: {:?}",
        end_user_client.signer_pk().to_hex()
    );

    //
    // load end-user's local wallet
    //
    let root_dir = dirs_next::data_dir()
        .ok_or_else(|| eyre!("could not obtain data directory path".to_string()))?
        .join("safe")
        .join("client");

    let wallet = LocalWallet::load_from(&root_dir)
        .wrap_err("Unable to read wallet file in {root_dir:?}")
        .suggestion(
            "If you have an old wallet file, it may no longer be compatible. Try removing it",
        )?;
    let mut wallet_client = WalletClient::new(end_user_client.clone(), wallet);

    //
    // Create a well-known RegisterAddress
    // from an XorName and a `honeycomb` app's public key
    //
    const HONEYCOMB_PUBLIC_KEY: &str = "98b0807cde78204259d89a7ca510fe41a763774117efbdb0134c125b0a730567e8fb7699a7f6c686e7e5a013bfe6078d";
    let app_name = XorName::from_content(b"safe://honeycomb");
    let honeycomb_pk = PublicKey::from_hex(HONEYCOMB_PUBLIC_KEY).unwrap();
    let address = RegisterAddress::new(app_name, honeycomb_pk);
    println!("Honeycomb XorName: {:?}", app_name);
    println!("Honeycomb public key: {:?}", honeycomb_pk.to_hex());
    println!("Honeycomb public RegisterAddress: {:?}", address.to_hex());

    //
    // Load or create the above Honeycomb public RegisterAddress as an end-user
    //
    let honeycomb_register = match end_user_client.get_register(address).await {
        Ok(register) => {
            println!("Register found at {:?}!", register.address().to_hex(),);
            register
        }
        Err(_) => {
            println!("Register not found, creating one at {address:?}");
            let (register, _cost, _royalties_fees) = end_user_client
                .create_and_pay_for_register(app_name, &mut wallet_client, true)
                .await?;

            register
        }
    };

    println!(
        "Honeycomb register address: {:?}",
        honeycomb_register.address().to_hex()
    );
    println!("Owned by: {:?}", honeycomb_register.owner().to_hex());
    println!("Permissions: {:?}", honeycomb_register.permissions());

    Ok(())
}

