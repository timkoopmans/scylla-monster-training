use crate::challenges::c002;
use crate::monster::{ask, award, exit, fail, info, pause, redraw, say, warn};

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

    if false {
        fail("Looks like you didn't complete the challenge. Here are some hints:");
        return;
    }

    award();

    if ask("Are you ready for your next challenge?") {
        c002::setup();
    } else {
        warn("No worries, you can come back and try another time!");
    }
}
