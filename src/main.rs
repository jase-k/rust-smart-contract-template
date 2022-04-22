use web3::contract::{Options, Contract};
use web3::transports::Http;
// use web3::types::Address;
// use std::time::Duration;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    // use hex_literal::hex;

    // println!("Calling accounts.");
    // let mut accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    //Contract details
    let publishing_account = hex_literal::hex!("86cd02231387E225a421aDA60BBAefeC9C21C07C"); 
    let binary_file = include_str!("./contracts/output/SimpleStorage.bin").trim_end();
    // let abi_file: Box<dyn Sized> = include_bytes!("./contracts/output/SimpleStorage.abi.json");

    deploy_contract(publishing_account, &binary_file).await;
    // println!("Calling balance.");
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).await?;
    //     println!("Balance of {:?}: {}", account, balance);
    // }

    Ok(())
}
/// Accounts will always have the same length so we can expect a byte array with the length of 20 to be consistent
async fn deploy_contract(account: [u8; 20], binary_code: &str) -> Result<Contract<Http>, Box<dyn std::error::Error>> {
    let transport = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(transport);
    //Deploying a Contract

    let contract_result = Contract::deploy(
        web3.eth(),
        include_bytes!("./contracts/output/SimpleStorage.abi.json"),
    );
    println!("Contract result: {:?}", contract_result);
    
    let gas_limit: u128 = 3_000_000;
    
    match contract_result {
        Ok(contract) => {
            println!("Deploying Contract!");
            let execution_result = contract
                                    .confirmations(0)
                                    //gas limit converts to custom type U256 more here: https://docs.rs/web3/0.18.0/web3/types/struct.U256.html#impl-From%3Cu128%3E
                                    .options(Options::with(|opt| opt.gas = Some(gas_limit.into()))) 
                                    .execute(binary_code, (), account.into())
                                    .await?;
            println!("Contract Result: {:?}", execution_result);
            return Ok(execution_result)
        },
        Err(e) => {
            println!("Error Occured Deploying contract: {:?}", e);
            return Err(Box::new(e))
        }
    };
}
