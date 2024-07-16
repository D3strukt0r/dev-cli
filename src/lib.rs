use bollard::Docker;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about = "A CLI for managing local Docker development environments", long_about = None)]
pub struct Cli {
    /// The name of the service to run the command in. If omitted, the first service in the project will be used.
    #[arg(short, long)]
    pub service: Option<String>,

    /// Run the command in offline mode. This will prevent dev-cli from trying to connect to the internet.
    pub offline: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    pub exec_command: Vec<String>,
}

#[derive(Debug, Clone, Subcommand, PartialEq)]
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

impl Commands {
    pub fn requires_docker(&self) -> bool {
        match self {
            Commands::Start
            | Commands::Stop { .. }
            | Commands::Restart
            | Commands::Poweroff
            | Commands::Exec { .. }
            | Commands::Run { .. }
            | Commands::Shell
            | Commands::Status
            | Commands::GlobalStatus => true,
            _ => false
        }
    }
}

pub fn is_docker_required(
    command: &Option<Commands>,
    exec_command: &Vec<String>,
) -> bool {
    let required_by_command = match command {
        Some(command) => command.requires_docker(),
        None => false,
    };
    required_by_command || exec_command.len() > 0
}

pub async fn docker_running(docker: &Docker) -> String {
    match docker.ping().await {
        Ok(result) => result,
        Err(error) => {
            println!("Docker doesn't seem to be turned on ({})", error);
            sysexits::ExitCode::OsErr.exit()
            //Err(anyhow::anyhow!("Docker doesn't seem to be turned on ({})", error))
        }
    }
}
