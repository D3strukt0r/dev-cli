mod utils;
mod commands;

use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version, about = "A CLI for managing local Docker development environments", long_about = None)]
struct Cli {
    /// The name of the service to run the command in. If omitted, the first service in the project will be used.
    #[arg(short, long)]
    service: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    exec_command: Vec<String>,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum Commands {
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
    /// Execute a shell command in the container for a service.
    Exec {
        #[arg(short, long)]
        service: Option<String>,

        command: Vec<String>
    },
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
    // Completely stop all projects and containers
    //Poweroff,
    // Restart a project or several projects.
    //Restart,
    // Add or remove, enable or disable extra services
    //Service,
    // Create a database snapshot for one or more projects.
    //Snapshot,
    // Starts a shell session in the container for a service
    //Ssh,
    /// Starts a dev-cli project
    Start,
    /// Stop and remove the containers of a project. Does not lose or harm anything unless you add --remove-data.
    Stop {
        #[arg(long, default_value("false"))]
        remove_data: bool,
    },
}

// Global constants for config file names
const CONFIG_FILE_NAME_LOCAL: &str = ".dev-cli.yml";
const CONFIG_FILE_NAME_PROJECT: &str = ".dev-cli.dist.yml";

#[tokio::main]
async fn main() -> Result<sysexits::ExitCode, Box<dyn std::error::Error>> {
    // Parse the command line arguments and stop here if there's an error
    let cli = Cli::parse();

    // Find .dev-cli.yml/.dev-cli.dist.yml in the current directory or any
    // parent directory to determine the project root
    let cwd = std::env::current_dir()?;
    let mut project_root_tmp: Option<std::path::PathBuf> = None;
    let local_config_path = match utils::path::find_recursively(&cwd, CONFIG_FILE_NAME_LOCAL) {
        Some(filepath) => {
            if project_root_tmp.is_none() {
                project_root_tmp = Some(filepath.parent().unwrap().to_path_buf());
            }
            Some(filepath)
        },
        None => None,
    };
    let project_config_path = match utils::path::find_recursively(&cwd, CONFIG_FILE_NAME_PROJECT) {
        Some(filepath) => {
            if project_root_tmp.is_none() {
                project_root_tmp = Some(filepath.parent().unwrap().to_path_buf());
            }
            Some(filepath)
        },
        None => None,
    };
    let project_root = match project_root_tmp {
        Some(path) => path,
        None => {
            println!("Could not find a project root. Please add a {} or {} to your project root", CONFIG_FILE_NAME_LOCAL, CONFIG_FILE_NAME_PROJECT);
            sysexits::ExitCode::OsErr.exit()
        },
    };
    // TODO: Get config based on global, user, project, and local config files

    // Connect to Docker
    // TODO: Check if command requires a docker connection before connecting
    let docker = bollard::Docker::connect_with_local_defaults()?;
    match docker.ping().await {
        Ok(result) => result,
        Err(error) => {
            println!("Docker doesn't seem to be turned on ({})", error);
            sysexits::ExitCode::OsErr.exit()
            //Err(anyhow::anyhow!("Docker doesn't seem to be turned on ({})", error))
        },
    };

    // Find and read the docker `compose.yml` file
    // TODO: Check if command requires knowledge of the compose config
    let docker_compose_config_path = std::path::Path::new(&project_root).join("compose.yml");
    let docker_compose = if docker_compose_config_path.is_file() {
        utils::docker_compose::DockerCompose::new(docker_compose_config_path)
    } else {
        println!("Could not find a docker compose file in the project root ({})", docker_compose_config_path.display());
        sysexits::ExitCode::OsErr.exit()
    };
    let docker_compose_config = match docker_compose.config() {
        Ok(config) => config,
        Err(error) => {
            println!("Could not read the docker compose file ({})", error);
            sysexits::ExitCode::OsErr.exit()
        },
    };

    use Commands::*;
    match cli.command {
        Some(command) => {
            match &command {
                Exec { service, command } => commands::exec::run(docker_compose, service.to_owned(), command.to_vec()),
                //Stop { remove_data: false } => println!("Stopping without removing data..."),
                //Stop { remove_data: true } => println!("Stopping with removing data..."),
                _ => {
                    println!("Command not implemented yet: {:?}", command);
                    sysexits::ExitCode::OsErr.exit()
                },
            }
        },
        None => {
            commands::exec::run(docker_compose, cli.service.to_owned(), cli.exec_command);
        },
    }

    Ok(sysexits::ExitCode::Ok)
}
