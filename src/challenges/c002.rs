use crate::challenges::c002;
use crate::monster::{ask, award, exit, fail, info, pause, redraw, say, warn};

pub fn setup() {
    redraw();

    say("Great! here's your next challenge");
    pause();

    redraw();

    info("Challenge 2: Add 2 more nodes to the cluster.");

    say(r#"
    Add another 2 nodes to your cluster, calling them node2 and node3..
    "#);

    exit("smt -c 002");
}

pub fn solve() {
    redraw();

    say("Welcome back! Please wait while I check your results:");
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
