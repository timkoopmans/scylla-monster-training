use bollard::Docker;
use tokio::runtime::Runtime;
use crate::challenges::c002;
use crate::checks::docker::{check_docker_container, check_docker_network};
use crate::checks::nodetool::check_nodetool_status;
use crate::monster::{ask, exit, info, pause, redraw, say, warn};

pub fn setup() {
    redraw();

    say("Great! here's your next challenge.");
    pause();

    redraw();

    info("Challenge 2: Add 2 more nodes to the cluster.");

    say(r#"
    Add another 2 nodes to your cluster, calling them node2 and node3.
    Make sure that all nodes are part of the same network, and that they are
    able to communicate with each other. You should be able to run the following
    command on each node and see the status of the cluster:

        nodetool statusbinary

    Hints: you can also use the following command to check the status of the cluster:

        docker exec -it node1 nodetool status
    "#);

    exit("smt -c 002");
}

pub fn solve() {
    redraw();

    say("Welcome back! Please wait while I check your result ...");

    if !checks() {
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

        if !check_docker_container(&docker, "node2", "scylla").await {
            return false;
        }

        if !check_docker_container(&docker, "node3", "scylla").await {
            return false;
        }

        if !check_nodetool_status(&docker, "node1").await {
            return false;
        }

        true
    });

    result
}
