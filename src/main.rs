use web3::contract::Contract;
use web3::types::Address;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // use hex_literal::hex;

    let transport = web3::transports::Http::new("http://localhost:7545")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    //Deploying a Contract
    let publishing_account = hex_literal::hex!("86cd02231387E225a421aDA60BBAefeC9C21C07C").into();

    let binary_code = include_str!("./contracts/output/SimpleStorage.bin").trim_end();

    let contract_result = Contract::deploy(
        web3.eth(),
        include_bytes!("./contracts/output/SimpleStorage.abi.json"),
    );

    match contract_result {
        Ok(contract) => {
            println!("Deploying Contract!");
            let execution_result = contract
                                    .confirmations(0)
                                    .execute(binary_code, (), publishing_account);
            match execution_result.await {
                Ok(contract) => println!("success {:?}", contract),
                Err(e) => println!("error executing deployment: {:?}", e)
            };
        }
        Err(e) => {
            println!("Error Occured Deploying contract: {:?}", e);
        }
    };

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
