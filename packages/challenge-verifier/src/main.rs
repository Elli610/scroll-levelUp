use std::process::{Command, exit};
use std::fs;
use std::path::Path;
use std::fs::remove_dir_all;

fn main() {
    // The GitHub URL and development tool name are inputs
    let github_url = "https://github.com/Cypher-Laboratory/Alice-s-Ring-EVM-verifier";
    let dev_tool = "hardhat"; // Options: "hardhat", "foundry", "brownie"

    // Create the "./repo" directory if it doesn't exist
    let repo_dir = Path::new("./repo");
    if !repo_dir.exists() {
        match fs::create_dir(repo_dir) {
            Ok(_) => {
                eprintln!("Successfully created './repo' directory.");
            }
            Err(e) => {
                eprintln!("Failed to create './repo' directory: {}", e);
                exit(1);
            }
        }
    } else {
        // Ensure the folder is empty: remove the content of ./repo/
        match remove_dir_all(repo_dir) {
            Ok(_) => println!("Successfully removed './repo' directory."),
            Err(e) => eprintln!("Failed to remove './repo' directory: {}", e),
        }
    }

    // Clone the repository into the "./repo" folder
    let clone_status = Command::new("git")
        .args(&["clone", github_url, "./repo"])
        .status()
        .expect("Failed to execute git command");

    if !clone_status.success() {
        eprintln!("Failed to clone repository.");
        exit(1);
    }

    // Navigate to the cloned repository
    let repo_path = Path::new("./repo");

    // Compile contracts based on the specified development tool
    match dev_tool {
        "hardhat" => {
            // Run npm install
            let install_status = Command::new("npm")
                .args(&["install"])
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute npm install command");

            if !install_status.success() {
                eprintln!("Failed to run npm install.");
                exit(1);
            }

            let compile_status = Command::new("npx")
                .args(&["hardhat", "compile"])
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute Hardhat compile command");

            if !compile_status.success() {
                eprintln!("Failed to compile contracts with Hardhat.");
                exit(1);
            }
        }
        "foundry" => {
            let compile_status = Command::new("forge")
                .args(&["build"])
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute Foundry build command");

            if !compile_status.success() {
                eprintln!("Failed to compile contracts with Foundry.");
                exit(1);
            }
        }
        "brownie" => {
            let compile_status = Command::new("brownie")
                .args(&["compile"])
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute Brownie compile command");

            if !compile_status.success() {
                eprintln!("Failed to compile contracts with Brownie.");
                exit(1);
            }
        }
        _ => {
            eprintln!("Unsupported development tool: {}", dev_tool);
            exit(1);
        }
    }

    // Start anvil
    let anvil_status = Command::new("anvil")
        .spawn()
        .expect("Failed to start anvil");

    // Deploy the contract (this part will be specific to your project and tool)
    match dev_tool {
        "hardhat" => {
            let deploy_status = Command::new("npx")
            // todo
            // for security reasons, we should not execute the script blindly. Maybe we should require 
            // the user to make his project comply with a predefined script we wrote
            // second option: we deploy direcly from the bytecode but won't be able to use the user's constructor args. We would use ours
                .args(&["hardhat", "run", "scripts/deploy.ts", "--network", "localhost"]) 
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute Hardhat deploy command");

            if !deploy_status.success() {
                eprintln!("Failed to deploy contracts with Hardhat.");
                exit(1);
            }
        }
        "foundry" => {
            let deploy_status = Command::new("forge")
                .args(&["script", "scripts/Deploy.s.sol", "--fork-url", "http://localhost:8545", "--broadcast"])
                .current_dir(repo_path)
                .status()
                .expect("Failed to execute Foundry deploy command");

            if !deploy_status.success() {
                eprintln!("Failed to deploy contracts with Foundry.");
                exit(1);
            }
        }
        // "brownie" => {
        //     let deploy_status = Command::new("brownie")
        //         .args(&["run", "scripts/deploy.py", "--network", "development"])
        //         .current_dir(repo_path)
        //         .status()
        //         .expect("Failed to execute Brownie deploy command");

        //     if !deploy_status.success() {
        //         eprintln!("Failed to deploy contracts with Brownie.");
        //         exit(1);
        //     }
        // }
        _ => {
            eprintln!("Unsupported development tool: {}", dev_tool);
            exit(1);
        }
    }

    println!("Contract deployment script completed successfully.");

    // Remove the content of ./repo/
    match remove_dir_all(repo_path) {
        Ok(_) => println!("Successfully removed './repo' directory."),
        Err(e) => eprintln!("Failed to remove './repo' directory: {}", e),
    }
}
