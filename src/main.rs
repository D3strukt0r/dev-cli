#[macro_use]
extern crate lazy_static;

mod commands;
mod utils;
mod lib;

use std::path::PathBuf;
use clap::Parser;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use dev_cli::{check_if_docker_required_and_ping, docker_running};
use crate::utils::app_config::AppConfig;
use crate::utils::path::find_recursively; // Used for writing assertions
use lib::{Cli, Commands};
use crate::lib::is_docker_required;

// Parameters for config
// - docker-compose-path: Path to the docker-compose file (default: {project-root}/compose.yml)
// - build-script-path: Path to the build script (ex.: ./docker/build/build.sh)
// - run-commands: List of commands to run in the container (ex.: [
//    "dev": {commands: [{container: "node", user: "node", command: "yarn run dev"}]},
//    "update": {
//      parallel: true,
//      commands: [
//        {container: "node", user: "node", command: "yarn upgrade"},
//        {container: "php", user: "www-data", command: "composer update"},
//    },
//    "clear-cache": {commands: [{container: "php", user: "www-data, command: "rm -rf var/cache/*"}]},
// ])

// Global constants for config file names
const CONFIG_FILE_NAME_LOCAL: &str = ".dev-cli.yml";
const CONFIG_FILE_NAME_PROJECT: &str = ".dev-cli.dist.yml";

lazy_static! {
    static ref CONFIG_FILE_PATH_GLOBAL: std::path::PathBuf = {
        [
            dirs::config_dir().unwrap(),
            std::path::PathBuf::from("dev-cli/"),
            std::path::PathBuf::from(".dev-cli.yml"),
        ]
        .iter()
        .collect()
    };
}

#[tokio::main]
async fn main() -> Result<sysexits::ExitCode, Box<dyn std::error::Error>> {
    // Parse the command line arguments and stop here if there's an error
    let cli = Cli::parse();

    // Connect to Docker
    let docker = bollard::Docker::connect_with_local_defaults()?;

    // Check if command is set and requires a docker connection before connecting or if exec_command is set
    if is_docker_required(&docker, &cli.command, &cli.exec_command) {
        docker_running(&docker).await;
    }

    utils::setup::check_and_install(&docker).await;

    println! {"Global config at {}", CONFIG_FILE_PATH_GLOBAL.clone().into_os_string().into_string().unwrap()};

    // Find .dev-cli.yml/.dev-cli.dist.yml in the current directory or any
    // parent directory to determine the project root
    let cwd = std::env::current_dir()?;
    let mut project_root_tmp: Option<std::path::PathBuf> = None;

    let project_root = match  (
        find_recursively(&cwd, CONFIG_FILE_NAME_LOCAL),
        find_recursively(&cwd, CONFIG_FILE_NAME_PROJECT)
    ) {
        (Some(filepath), _) => filepath,
        (_, Some(filepath)) => filepath,
        (None, None) => {
            eprintln!( "Could not find a project root. Please add a {} or {} to your project root",
                CONFIG_FILE_NAME_LOCAL, CONFIG_FILE_NAME_PROJECT
            );
            sysexits::ExitCode::OsErr.exit()
        }
    };

    println!("project root: {:?}", &project_root);
    let app_config = utils::app_config::AppConfig::merge_from_project_root(&project_root);
    match app_config {
        Ok(conf) => println!("config loaded: {:?}", conf),
        Err(e) => eprintln!("error loading app config: {:?}", e)
    }

    // Find and read the docker `compose.yml` file
    // TODO: Check if command requires knowledge of the compose config
    let docker_compose_config_path = std::path::Path::new(&project_root).join("compose.yml");
    let docker_compose = if docker_compose_config_path.is_file() {
        utils::docker_compose::DockerCompose::new(docker_compose_config_path)
    } else {
        println!(
            "Could not find a docker compose file in the project root ({})",
            docker_compose_config_path.display()
        );
        sysexits::ExitCode::OsErr.exit()
    };
    let docker_compose_config = match docker_compose.config() {
        Ok(config) => config,
        Err(error) => {
            println!("Could not read the docker compose file ({})", error);
            sysexits::ExitCode::OsErr.exit()
        }
    };

    //let images = &docker.list_images(Some(bollard::image::ListImagesOptions::<String> {
    //    all: true,
    //    ..Default::default()
    //})).await.unwrap();
    //for image in images {
    //    println!("-> {:?}", image.id);
    //}

    use Commands::*;
    match cli.command {
        Some(command) => {
            match command {
                Exec { service, user, command } => {
                    commands::exec::run(docker_compose, service, user, command.to_vec())
                }
                //Stop { remove_data: false } => println!("Stopping without removing data..."),
                //Stop { remove_data: true } => println!("Stopping with removing data..."),
                _ => {
                    println!("Command not implemented yet: {:?}", command);
                    sysexits::ExitCode::OsErr.exit()
                }
            }
        }
        None => {
            commands::exec::run(docker_compose, cli.service.to_owned(), None, cli.exec_command);
        }
    }

    Ok(sysexits::ExitCode::Ok)
}

#[test]
fn no_project_root() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::cargo_bin("dev-cli")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not find a project root"));

    Ok(())
}
