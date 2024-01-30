pub fn run(docker_compose: crate::utils::docker_compose::DockerCompose, service: Option<String>, command: Vec<String>) {
    docker_compose.exec(service, command);
}
