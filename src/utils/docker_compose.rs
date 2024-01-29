#[derive(Debug)]
pub struct DockerCompose {
    file: std::path::PathBuf,
}

impl DockerCompose {
    pub fn new(file: std::path::PathBuf) -> Self {
        Self {
            file,
        }
    }

    pub fn config(&self) -> Result<Config, serde_yaml::Error> {
        let output_cmd = if cfg!(target_os = "windows") {
            panic!("Windows is not supported yet")
            //std::process::Command::new("cmd")
            //    .args(["/C", "echo hello"])
            //    .output()
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg("docker compose config")
                .output()
        };
        let output = match output_cmd {
            Ok(output) => output,
            Err(error) => {
                println!("Docker doesn't seem to be turned on ({})", error);
                sysexits::ExitCode::OsErr.exit()
            },
        };
        let config_string = match std::str::from_utf8(&output.stdout) {
            Ok(stdout) => stdout,
            Err(error) => {
                println!("Error: {}", error);
                sysexits::ExitCode::OsErr.exit()
            },
        };
        let config = serde_yaml::from_str::<Config>(config_string);
        config
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    name: String,
    services: std::collections::BTreeMap<String, Service>,
    networks: Option<std::collections::BTreeMap<String, Network>>,
    volumes: Option<std::collections::BTreeMap<String, Volume>>,
    secrets: Option<std::collections::BTreeMap<String, Secret>>,
}

#[derive(Debug, serde::Deserialize)]
struct Service {
    // TODO: Check "https://serde.rs/string-or-struct.html" for how to handle "build"
    //build: Option<String>,
    //command: Option<String>,
    container_name: Option<String>,
    depends_on: Option<std::collections::BTreeMap<String, ServiceDependsOn>>,
    environment: Option<std::collections::BTreeMap<String, String>>,
    image: Option<String>,
    init: Option<bool>,
    labels: Option<std::collections::BTreeMap<String, String>>,
    networks: Option<std::collections::BTreeMap<String, Option<String>>>,
    ports: Option<Vec<ServicePorts>>,
    secrets: Option<Vec<ServiceSecret>>,
    volumes: Option<Vec<ServiceVolume>>,
}

#[derive(Debug, serde::Deserialize)]
struct ServiceDependsOn {
    condition: String,
    required: bool
}

#[derive(Debug, serde::Deserialize)]
struct ServicePorts {
    mode: String,
    target: u16,
    published: String,
    protocol: String,
}

#[derive(Debug, serde::Deserialize)]
struct ServiceSecret {
    source: String,
}

#[derive(Debug, serde::Deserialize)]
struct ServiceVolume {
    #[serde(rename = "type")]
    volume_type: String,
    source: String,
    target: String,
    bind: Option<ServiceVolumeBind>,
    // TODO: Don't know the actual type of this
    volume: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Debug, serde::Deserialize)]
struct ServiceVolumeBind {
    create_host_path: bool,
}

#[derive(Debug, serde::Deserialize)]
struct Network {
    name: String,
    external: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct Volume {
    name: String,
    driver: Option<String>,
    external: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct Secret {
    name: String,
    file: String,
}
