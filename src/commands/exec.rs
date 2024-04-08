use crate::utils::docker_compose::DockerCompose;

pub fn run(docker_compose: DockerCompose, service: Option<String>, user: Option<String>, command: Vec<String>) {
    docker_compose.exec(service, command);
}
