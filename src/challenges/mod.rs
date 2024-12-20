use crate::config::Challenge;
use crate::monster::{ask, exit, info, redraw, say, warn};
use bollard::Docker;
use tokio::runtime::Runtime;
use tracing::error;
use crate::checks::cql::{check_keyspace, check_table};
use crate::checks::docker::{check_docker_container, check_docker_network};
use crate::checks::nodetool::check_nodetool_status;
use crate::CONFIG;

pub fn setup(challenge: &Challenge) {
    redraw();
    info(&challenge.description);

    say(&challenge.setup);

    exit(&format!("smt --solve {}", challenge.id));
    std::process::exit(0);
}

pub fn solve(challenge: &Challenge) {
    redraw();
    say("Welcome back! Please wait while I check your results ...");

    if !checks(challenge) {
        return;
    }

    if ask("Are you ready for your next challenge?") {
        let next_challenge_id = format!("{:03}", challenge.id.parse::<u32>().unwrap() + 1);
        if let Some(next_challenge) = CONFIG.challenges.iter().find(|c| c.id == next_challenge_id) {
            setup(next_challenge);
        } else {
            warn("No more challenges available!");
        }
    } else {
        warn("No worries, you can come back and try another time!");
    }
}

fn checks(challenge: &Challenge) -> bool {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let docker = Docker::connect_with_local_defaults().unwrap();

        for command in &challenge.solve {
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
        error!("Invalid check command: {}", command);
        return false;
    }

    let func_name = parts[0];
    let args: Vec<&str> = parts[1..].to_vec();

    match func_name {
        "check_docker_network" => {
            if args.len() == 1 {
                check_docker_network(docker, args[0]).await
            } else {
                error!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_docker_container" => {
            if args.len() == 2 {
                check_docker_container(docker, args[0], args[1]).await
            } else {
                error!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_nodetool_status" => {
            if args.len() == 1 {
                check_nodetool_status(docker,args[0]).await
            } else {
                error!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_keyspace" => {
            if args.len() == 2 {
                check_keyspace(docker, args[0], args[1]).await
            } else {
                error!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        "check_table" => {
            if args.len() == 3 {
                check_table(docker, args[0], args[1], args[2]).await
            } else {
                error!("Invalid arguments for {}: {:?}", func_name, args);
                false
            }
        }
        _ => {
            error!("Unknown check command: {}", func_name);
            false
        }
    }
}