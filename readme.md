# **Work In Progess** 

# Purpose & Overview 
This project is to demonstrate a workflow environment using Solidity and Rust to build, deploy, and interact with smart contracts on the ethereum blockchain. 

We will be using docker to compile the solidity smart contracts. 
We will be using the web3 crate. https://crates.io/crates/web3
We will be using ganache-cli to run tests on a simulated blockchain

# Set-Up
Install Rust: 
Install Docker
#### Web3 crate: 
Window Users: 
To install web3 dependency properly at the time of this writing you need to disable the IPC feature. Do this be putting this code in your cargo.toml file: 
```
web3 = { version = "0.17.0", default-features = false, features = ["http"] }
```

# Solidity Compiler Options: 
NPM vs Docker
### Docker
Using docker is a simple way to compile your smart contracts into the format the EVM needs to execute your code. You can find more information here: https://docs.soliditylang.org/en/latest/installing-solidity.html#docker

Here are some examples and a breakdown if you aren't familiar with docker: (first you need to install Docker. See above)
```
docker run -v C:\Users\jasek\Apps\blockchain\simple_storage_smart_contract\src\contracts:/sources --rm ethereum/solc:stable  -o /sources/output --abi --bin /sources/SimpleStorage.sol --overwrite --gas
```
```docker run``` : native docker command that will pull a docker image from the docker directory and create a container on your local computer

```-v local_path:container_path``` : -v or --volume is an option for docker run that lets you connect (mount) your specified local directory to a specified directory in your container. The ":" seperates your local path and container path. NOTE: you need to specify the complete path in your local directory rather than the relative path. 

```--rm``` : This is an option for docker run that removes the container when done executing. This is not needed to actually compile the code, but it helps keep a clean working environment. For a complete list of options for docker run; run the command ```docker run --help```

```ethereum/solc:stable``` : This is the actual container that the ```docker run``` command is pulling from the docker hub directory. ```ethereum/solc``` is the container name & ```stable``` is the version (stable pulls the latest stable version, but you could specify a version with the version number e.g. ```0.8.0``` in place of ```stable``` like so: ```ethereum/solc:0.8.0```)

```-o /sources/output``` : This is an option of ```ethereum/solc``` that specifies the directory output of the compiled files. Keep in mind that if you want the files to show up in your local directory you must use the same folder name as specified in the ```-v local_path:container_path``` option 

```--abi``` : This options specifies that you want an abi output (Application Binary Interface: 1 of 2 files need to execute your contract on the EVM (Ethereum Virtual Machine))

```--bin``` : This option specifies that you want a binary executable output. 

```/sources/SimpleStorage.sol``` : This is the file path of the contract you want to compile or recompile. *Must be the file path of the container*

```--overwrite``` : This overwrites files in the directory if the compiler finds identical file names in the folder of its output destination

```--gas``` : This outputs estimated gas to the console for each function in the contract

**For a complete list of options for the ```ethereum/solc``` compiler run ```docker run --rm ethereum/solc:stable --help```**


