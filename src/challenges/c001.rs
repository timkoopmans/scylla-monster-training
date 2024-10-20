use crate::challenges::c002;
use crate::monster::{ask, award, exit, fail, info, pause, redraw, say, warn};
use bollard::Docker;
use bollard::container::ListContainersOptions;
use bollard::network::InspectNetworkOptions;
use tokio::runtime::Runtime;
use std::collections::HashMap;

pub fn setup() {
    redraw();

    say("Great! here's your first challenge");
    pause();

    redraw();

    info("Challenge 1: Setup a single node cluster using docker.");

    say(r"
    Have a go at creating a single node cluster, and call it node1.

    Hints:
    1. Since we will be starting multiple nodes in the coming challenges,
    make sure first create a bridged network so we can communicate:
        docker network create --driver bridge scylla

    2. Since we're just kicking the tires, make sure your run the node with
    the following flags:
        --smp 1
        --memory 1G
    ");

    exit("smt -c 001");
}

pub fn solve() {
    redraw();

    say("Welcome back! Please wait while I check your results");
    pause();

    redraw();

    if !check_docker_setup() {
        return;
    }

    say("Nice one! Looks like you solved the challenge.\n");
    // award();

    if ask("Are you ready for your next challenge?") {
        c002::setup();
    } else {
        warn("No worries, you can come back and try another time!");
    }
}

pub fn check_docker_setup() -> bool {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let docker = Docker::connect_with_local_defaults().unwrap();

        // Check if the Docker network 'scylla' exists
        let network = docker.inspect_network("scylla", None::<InspectNetworkOptions<String>>).await;
        if network.is_err() {
            fail("Docker network 'scylla' does not exist. Please create it using: \n\
            docker network create --driver bridge scylla");
            return false;
        }

        // Check if the container 'node1' is running
        let mut filters = HashMap::new();
        filters.insert("name", vec!["node1"]);
        filters.insert("network", vec!["scylla"]);

        let options = Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        });

        let containers = docker.list_containers(options).await.unwrap();
        if containers.is_empty() {
            fail("Container 'node1' is not running on the 'scylla' network. \n\
            Please start it with the appropriate settings.");
            return false;
        }

        info("Container 'node1' is running on the 'scylla' network.");
        true
    });

    result
}