use crate::challenges::c002;
use crate::monster::{ask, exit, info, pause, redraw, say, warn};
use bollard::Docker;
use tokio::runtime::Runtime;
use crate::checks::docker::{check_docker_container, check_docker_network};
use crate::checks::nodetool::check_nodetool_status;

pub fn setup() {
    redraw();

    say("Great! here's your first challenge.");
    pause();

    redraw();

    info("Challenge 1: Setup a single node cluster using docker.");

    say(r"
    Have a go at creating a single node cluster, and call it node1.
    Make sure that the node is part of a network called 'scylla', and that it is
    able to communicate with itself.

    Hints:
    1. Since we will be starting multiple nodes in the coming challenges,
    make sure first create a bridged network so we can communicate:
        docker network create --driver bridge scylla

    2. Since we're just kicking the tires, make sure your run the node with
    the following arguments:
        docker run --rm -d --name node1 --network scylla scylladb/scylla \
        --smp 1 \
        --memory 1G
    ");

    exit("smt -c 001");
}

pub fn solve() {
    redraw();

    say("Welcome back! Please wait while I check your result ...");

    if !checks() {
        return;
    }

    say("Nice one! Looks like you solved the challenge.\n");

    if ask("Are you ready for your next challenge?") {
        c002::setup();
    } else {
        warn("No worries, you can come back and try another time!");
    }
}

pub fn checks() -> bool {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let docker = Docker::connect_with_local_defaults().unwrap();

        if !check_docker_network(&docker, "scylla").await {
            return false;
        }

        if !check_docker_container(&docker, "node1", "scylla").await {
            return false;
        }

        if !check_nodetool_status(&docker, "node1").await {
            return false;
        }

        true
    });

    result
}