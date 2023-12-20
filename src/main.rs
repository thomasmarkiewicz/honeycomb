use bls::SecretKey;
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

const HONEYCOMB_SK: &str = "4fd9baad382f577c5a71b9e8da385423ce51768081ab4ad2dfeab48d716267f6";
const HONEYCOMB_REGISTER_ADDRESS: &str = "63cc0c706dc89b75b8030c0806ce749ee3e2dac5b1f6fa1b1122daba9bb1681d99696624cd75823e359f4c5121fbe30ee6b642d4ec6d79fe1bfdf69e9f431b00767c74b63cf55a89170f8a917c09cc1d";

#[tokio::main]
async fn main() -> Result<()> {
    /*
    // create fixed secret key for honeycomb app
    let honeycomb_app_sk = SecretKey::from_hex(HONEYCOMB_SK).unwrap();
    println!("honeycomb_sk: {:?}", honeycomb_app_sk.to_hex());

    // create honeycomb app client
    let honeycomb_app_client = Client::new(honeycomb_app_sk, None, false, None).await?;
    println!(
        "client public key: {:?}",
        honeycomb_app_client.signer_pk().to_hex()
    );
    */

    //
    // create fixed secret key for honeycomb app
    //
    let sk = SecretKey::random();
    println!("sk: {:?}", sk.to_hex());

    //
    // create client
    //
    let client = Client::new(sk, None, false, None).await?;
    println!(
        "client created, public key: {:?}",
        client.signer_pk().to_hex()
    );

    //
    // load local wallet
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
    let mut wallet_client = WalletClient::new(client.clone(), wallet);

    //
    // Load or create a public Register at a fixed known location
    //
    let meta = XorName::from_content(b"safe://honeycomb");
    println!("XorName meta: {:?}", meta);

    let address = RegisterAddress::from_hex(HONEYCOMB_REGISTER_ADDRESS).unwrap();

    let mut reg_replica = match client.get_register(address).await {
        Ok(register) => {
            println!("Register found at {:?}!", register.address().to_hex(),);
            register
        }
        Err(_) => {
            println!("Register not found, creating one at {address:?}");
            let (register, _cost, _royalties_fees) = client
                .create_and_pay_for_register(meta, &mut wallet_client, true)
                .await?;

            register
        }
    };

    println!("Register address: {:?}", reg_replica.address().to_hex());
    println!("Register owned by: {:?}", reg_replica.owner().to_hex());
    println!("Register permissions: {:?}", reg_replica.permissions());

    Ok(())
}
