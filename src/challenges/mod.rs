use crate::config::Challenge;
use crate::monster::{ask, info, pause, redraw, say, warn};
use bollard::Docker;
use tokio::runtime::Runtime;
use crate::checks::docker::{check_docker_container, check_docker_network};
use crate::checks::nodetool::check_nodetool_status;

pub fn run_challenge(challenge: &Challenge) {
    setup(challenge);
    solve(challenge);
}

fn setup(challenge: &Challenge) {
    redraw();
    say(&format!("Great! here's your challenge: {}", challenge.description));
    pause();
    redraw();
    info(&challenge.description);

    for command in &challenge.setup_commands {
        say(command);
    }

    // exit(&format!("smt -c {}", challenge.id));
    // std::process::exit(0);
}

fn solve(challenge: &Challenge) {
    redraw();
    say("Welcome back! Please wait while I check your result ...");

    if !checks(challenge) {
        return;
    }

    say("Nice one! Looks like you solved the challenge.\n");

    if ask("Are you ready for your next challenge?") {
        // Load the next challenge dynamically
    } else {
        warn("No worries, you can come back and try another time!");
    }
}

fn checks(challenge: &Challenge) -> bool {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let docker = Docker::connect_with_local_defaults().unwrap();

        for command in &challenge.check_commands {
            // Execute the check commands dynamically
            // This is a placeholder, you need to implement the actual check logic
            if !execute_check_command(&docker, command).await {
                return false;
            }
        }

        true
    });

    result
}

async fn execute_check_command(docker: &Docker, command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        eprintln!("Invalid check command: {}", command);
        return false;
    }

    let func_name = parts[0];
    let args: Vec<&str> = parts[1..].to_vec();

    match func_name {
        "check_docker_network" => {
            if args.len() == 1 {
                check_docker_network(docker, args[0]).await
            } else {
                eprintln!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_docker_container" => {
            if args.len() == 2 {
                check_docker_container(docker, args[0], args[1]).await
            } else {
                eprintln!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_nodetool_status" => {
            if args.len() == 1 {
                check_nodetool_status(docker,args[0]).await
            } else {
                eprintln!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        _ => {
            eprintln!("Unknown check command: {}", func_name);
            false
        }
    }
}