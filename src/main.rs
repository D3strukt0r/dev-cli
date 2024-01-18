use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "dev-cli", version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Removes items dev-cli has created
    Clean,
    /// Generate the autocompletion script for the specified shell
    Completion,
    /// Create or modify a dev-cli project configuration in the current directory
    Config,
    /// Remove all project information (including database) for an existing project
    Delete,
    /// Get a detailed description of a running dev-cli project
    Describe,
    /// Execute a shell command in the container for a service.
    Exec,
    /// Dump a database to a file or to stdout
    ExportDb,
    /// Get/Download a 3rd party add-on (service, provider, etc.)
    Get,
    /// Manage your hostfile entries.
    Hostname,
    /// Import a SQL dump file into the project
    ImportDb,
    /// Pull the uploaded files directory of an existing project to the default public upload directory of your project
    ImportFiles,
    /// List projects
    List,
    /// Get the logs from your running services.
    Logs,
    /// Completely stop all projects and containers
    Poweroff,
    /// Restart a project or several projects.
    Restart,
    /// Add or remove, enable or disable extra services
    Service,
    /// Create a database snapshot for one or more projects.
    Snapshot,
    /// Starts a shell session in the container for a service
    Ssh,
    /// Starts a dev-cli project
    Start,
    /// Stop and remove the containers of a project. Does not lose or harm anything unless you add --remove-data.
    Stop,
}

fn main() {
    let args = Args::parse();
}
