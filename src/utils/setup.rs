use std::collections::HashMap;
use anyhow::Result;
use bollard::network::{CreateNetworkOptions, ListNetworksOptions};

pub async fn check_and_install(docker: &bollard::Docker) -> Result<()> {
    println!("Checking and installing dependencies...");

    // Check that Homebrew is installed
    //let command = "brew --version";
    //let homebrew_check = subprocess::Exec::shell(command).capture().unwrap();
    //if !homebrew_check.exit_status.success() {
    //    println!("Homebrew is not installed");
    //    println!("Installing Homebrew...");
    //    let command = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    //    let homebrew_install = subprocess::Exec::cmd(command).capture().unwrap();
    //    if homebrew_install.exit_status.success() {
    //        println!("Homebrew installed successfully");
    //    } else {
    //        println!("Homebrew installation failed");
    //        std::process::exit(match homebrew_install.exit_status {
    //            subprocess::ExitStatus::Exited(code) => code as i32,
    //            subprocess::ExitStatus::Signaled(code) => code as i32,
    //            subprocess::ExitStatus::Other(code) => code,
    //            subprocess::ExitStatus::Undetermined => 1,
    //        });
    //    }
    //}

    // Check that Docker is installed
    //let command = "docker --version";
    //let docker_check = subprocess::Exec::shell(command).capture().unwrap();
    //if !docker_check.exit_status.success() {
    //    println!("Docker is not installed");
    //    println!("Installing Docker...");
    //    let command = "brew install docker";
    //    let docker_install = subprocess::Exec::shell(command).capture().unwrap();
    //    if docker_install.exit_status.success() {
    //        println!("Docker installed successfully");
    //    } else {
    //        println!("Docker installation failed");
    //        std::process::exit(match docker_install.exit_status {
    //            subprocess::ExitStatus::Exited(code) => code as i32,
    //            subprocess::ExitStatus::Signaled(code) => code as i32,
    //            subprocess::ExitStatus::Other(code) => code,
    //            subprocess::ExitStatus::Undetermined => 1,
    //        });
    //    }
    //}

    // Check that Git is installed
    //let command = "git --version";
    //let git_check = subprocess::Exec::shell(command).capture().unwrap();
    //if !git_check.exit_status.success() {
    //    println!("Git is not installed");
    //    println!("Installing Git...");
    //    let command = "brew install git";
    //    let git_install = subprocess::Exec::shell(command).capture().unwrap();
    //    if git_install.exit_status.success() {
    //        println!("Git installed successfully");
    //    } else {
    //        println!("Git installation failed");
    //        std::process::exit(match git_install.exit_status {
    //            subprocess::ExitStatus::Exited(code) => code as i32,
    //            subprocess::ExitStatus::Signaled(code) => code as i32,
    //            subprocess::ExitStatus::Other(code) => code,
    //            subprocess::ExitStatus::Undetermined => 1,
    //        });
    //    }
    //}

    // Check that jq is installed
    //let command = "jq --version";
    //let jq_check = subprocess::Exec::shell(command).capture().unwrap();
    //if !jq_check.exit_status.success() {
    //    println!("jq is not installed");
    //    println!("Installing jq...");
    //    let command = "brew install jq";
    //    let jq_install = subprocess::Exec::shell(command).capture().unwrap();
    //    if jq_install.exit_status.success() {
    //        println!("jq installed successfully");
    //    } else {
    //        println!("jq installation failed");
    //        std::process::exit(match jq_install.exit_status {
    //            subprocess::ExitStatus::Exited(code) => code as i32,
    //            subprocess::ExitStatus::Signaled(code) => code as i32,
    //            subprocess::ExitStatus::Other(code) => code,
    //            subprocess::ExitStatus::Undetermined => 1,
    //        });
    //    }
    //}

    // Check that the docker network "dev-cli-web" exists using bollard
    let mut list_networks_filters = HashMap::new();
    list_networks_filters.insert("name", vec!["dev-cli-web"]);
    let config = ListNetworksOptions {
        filters: list_networks_filters,
    };
    let networks = docker.list_networks(Some(config)).await?;

    if networks.is_empty() {
        println!("Creating the network 'dev-cli-web'...");
        let config = CreateNetworkOptions {
            name: "dev-cli-web",
            ..Default::default()
        };

        docker.create_network(config);
    }


    println!("{:?}", networks);
    sysexits::ExitCode::OsErr.exit()
}
