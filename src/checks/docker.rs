use std::collections::HashMap;
use bollard::container::ListContainersOptions;
use bollard::Docker;
use bollard::network::InspectNetworkOptions;
use crate::monster::{fail, pass};

pub async fn check_docker_network(docker: &Docker, network_name: &str) -> bool {
    let network = docker.inspect_network(network_name, None::<InspectNetworkOptions<String>>).await;
    if network.is_err() {
        fail(&format!("❌ Docker network '{}' does not exist. Please create it using: \n\
        docker network create --driver bridge {}", network_name, network_name));
        return false;
    }
    true
}

pub async fn check_docker_container(docker: &Docker, container_name: &str, network_name: &str) -> bool {
    let mut filters = HashMap::new();
    filters.insert("name", vec![container_name]);
    filters.insert("network", vec![network_name]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await.unwrap();
    if containers.is_empty() {
        fail(&format!("❌ Container '{}' is not running on the '{}' network. \n\
        Please start it with the appropriate settings.", container_name, network_name));
        return false;
    }

    pass(&format!("✅ Container '{}' is running on the '{}' network.", container_name, network_name));
    true
}