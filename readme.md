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

Solidity Compiler Options: 
NPM vs Docker