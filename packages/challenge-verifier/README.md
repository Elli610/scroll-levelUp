## Challenge Verifier
A simple rust script to verify the challenge submissions from [LevelUp Web3](https://www.levelup.xyz/)


## Usage
Run the script with:
```bash
cargo run
```
Build the script for production with:
```bash
cargo build --release
```


## How it works
This script will use the github url and ethereum development tool name provided by the user to clone the repository, install the dependencies and deploy the contracts on anvil. 
It will then run the tests and return True if all the tests pass, otherwise it will return False.


> Note:
> - if the user use hardhat, they should use ts, not js