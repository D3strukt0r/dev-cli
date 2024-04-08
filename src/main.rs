#[macro_use]
extern crate lazy_static;

mod commands;
mod utils;

use std::path::PathBuf;
use clap::Parser;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use crate::utils::app_config::AppConfig;
use crate::utils::path::find_recursively; // Used for writing assertions

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

#[derive(Debug, clap::Parser)]
#[command(version, about = "A CLI for managing local Docker development environments", long_about = None)]
struct Cli {
    /// The name of the service to run the command in. If omitted, the first service in the project will be used.
    #[arg(short, long)]
    service: Option<String>,

    /// Run the command in offline mode. This will prevent dev-cli from trying to connect to the internet.
    offline: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,

    exec_command: Vec<String>,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum Commands {
    /// Initialize a new project for dev-cli using pre-defined templates
    Init,
    /// Starts a docker compose project
    Start,
    /// Stop and remove the containers of a project. Does not lose or harm anything unless you add --remove-data.
    Stop {
        #[arg(long, default_value("false"))]
        remove_data: bool,
    },
    /// Stops, removes and starts a project again
    Restart,
    /// Stop all projects and dev-cli containers (Traefik, etc.)
    Poweroff,
    /// Execute a shell command in the container for a service.
    Exec {
        #[arg(short, long)]
        service: Option<String>,

        #[arg(short, long)]
        user: Option<String>,

        command: Vec<String>,
    },
    /// Run a command defined in the config file
    Run {
        command: Vec<String>,
    },
    /// Starts a shell session in the container for a service
    Shell,
    /// Launches the default URL in the default browser
    Launch,
    /// Show the status of the containers of this project
    Status,
    /// Show the status of all projects that ran through dev-cli
    GlobalStatus,


    // Removes items dev-cli has created
    //Clean,
    // Generate the autocompletion script for the specified shell
    //Completion,
    // Create or modify a dev-cli project configuration in the current directory
    //Config,
    // Remove all project information (including database) for an existing project
    //Delete,
    // Get a detailed description of a running dev-cli project
    //Describe,
    // Dump a database to a file or to stdout
    //ExportDb,
    // Get/Download a 3rd party add-on (service, provider, etc.)
    //Get,
    // Manage your hostfile entries.
    //Hostname,
    // Import a SQL dump file into the project
    //ImportDb,
    // Pull the uploaded files directory of an existing project to the default public upload directory of your project
    //ImportFiles,
    // List projects
    //List,
    // Get the logs from your running services.
    //Logs,
    // Add or remove, enable or disable extra services
    //Service,
    // Create a database snapshot for one or more projects.
    //Snapshot,
}

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
    println! {"Global config at {}", CONFIG_FILE_PATH_GLOBAL.clone().into_os_string().into_string().unwrap()};

    // Parse the command line arguments and stop here if there's an error
    let cli = Cli::parse();

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
    // Connect to Docker
    // TODO: Check if command requires a docker connection before connecting
    let docker = bollard::Docker::connect_with_local_defaults()?;
    match docker.ping().await {
        Ok(result) => result,
        Err(error) => {
            println!("Docker doesn't seem to be turned on ({})", error);
            sysexits::ExitCode::OsErr.exit()
            //Err(anyhow::anyhow!("Docker doesn't seem to be turned on ({})", error))
        }
    };

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
