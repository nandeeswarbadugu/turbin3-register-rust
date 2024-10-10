mod programs;

use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     signature::{read_keypair_file, Signer},
//     system_program,
// };

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;


// use crate::programs::wba_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};
// use crate::register::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram}; 
use crate::register::programs::Turbin3_prereq::{WbaPrereqProgram, CompleteArgs,
    UpdateArgs}; 

pub fn submit_register(){

    // let signer = read_keypair_file("[58, 75, 111, 234, 35, 13, 122, 180, 226, 237, 59, 113, 208, 229, 65, 221, 197, 237, 223, 154, 141, 26, 9, 82, 192, 101, 218, 229, 198, 159, 123, 198, 210, 82, 227, 16, 212, 40, 156, 70, 84, 146, 185, 249, 160, 197, 106, 182, 32, 188, 52, 186, 99, 212, 89, 64, 245, 79, 59, 82, 103, 224, 132, 114]").expect("Couldn't find wallet file");
    
    let private_key_bytes: [u8; 64] = [
        58, 75, 111, 234, 35, 13, 122, 180, 226, 237, 59, 113, 208, 229, 65, 221,
        197, 237, 223, 154, 141, 26, 9, 82, 192, 101, 218, 229, 198, 159, 123, 198,
        210, 82, 227, 16, 212, 40, 156, 70, 84, 146, 185, 249, 160, 197, 106, 182,
        32, 188, 52, 186, 99, 212, 89, 64, 245, 79, 59, 82, 103, 224, 132, 114
    ];

    // Create the signer (Keypair) from the byte array
    let signer = Keypair::from_bytes(&private_key_bytes).expect("Invalid private key");

    // Now you can use `signer` for signing transactions or other cryptographic operations
    println!("Public Key: {:?}", signer.pubkey());

    const RPC_URL: &str = "https://api.devnet.solana.com";

    let rpc_client = RpcClient::new(RPC_URL);

    let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",signer.pubkey().to_bytes().as_ref()]);

    let args = CompleteArgs {github: b"nandeeswarbadugu".to_vec() };

    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction =WbaPrereqProgram::complete(&[&signer.pubkey(), &prereq, &system_program::id()], &args,
                            Some(&signer.pubkey()),
                            &[&signer],
                            blockhash );
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

    println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    
}

