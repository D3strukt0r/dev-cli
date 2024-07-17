use crate::utils::docker_compose::DockerCompose;

pub fn run(docker_compose: DockerCompose, service: Option<String>, user: Option<String>, command: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    docker_compose.exec(service, user, command)
}
