use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use futures::StreamExt;
use crate::monster::{fail, pass};

pub async fn check_keyspace(docker: &Docker, container_name: &str, keyspace: &str) -> bool {
    let exec = docker.create_exec(
        container_name,
        CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["cqlsh", "-e", &format!("DESCRIBE KEYSPACE {}", keyspace)]),
            ..Default::default()
        },
    ).await.unwrap();

    match docker.start_exec(&exec.id, None).await.unwrap() {
        StartExecResults::Attached { mut output, .. } => {
            let mut success = false;
            let mut output_str = String::new();
            while let Some(msg) = output.next().await {
                if let Ok(msg) = msg {
                    output_str.push_str(&msg.to_string());
                    if msg.to_string().contains(keyspace) {
                        success = true;
                    }
                }
            }
            if success {
                pass(format!("✅ Keyspace '{}' exists: {}", keyspace, output_str).as_str());
            } else {
                fail(format!("❌ Keyspace '{}' does not exist: {}", keyspace, output_str).as_str());
            }
            success
        },
        _ => {
            fail(format!("❌ Failed to check keyspace '{}'", keyspace).as_str());
            false
        },
    }
}

pub async fn check_table(docker: &Docker, container_name: &str, keyspace: &str, table: &str) -> bool {
    let exec = docker.create_exec(
        container_name,
        CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["cqlsh", "-e", &format!("DESCRIBE TABLE {}.{}", keyspace, table)]),
            ..Default::default()
        },
    ).await.unwrap();

    match docker.start_exec(&exec.id, None).await.unwrap() {
        StartExecResults::Attached { mut output, .. } => {
            let mut success = true;
            let mut output_str = String::new();
            while let Some(msg) = output.next().await {
                if let Ok(msg) = msg {
                    output_str.push_str(&msg.to_string());
                    if msg.to_string().contains("not found") {
                        success = false;
                    }
                }
            }
            if success {
                pass(format!("✅ Table '{}.{}' exists: {}", keyspace, table, output_str).as_str());
            } else {
                fail(format!("❌ Table '{}.{}' does not exist: {}", keyspace, table, output_str).as_str());
            }
            success
        },
        _ => {
            fail(format!("❌ Failed to check table '{}.{}'", keyspace, table).as_str());
            false
        },
    }
}

pub async fn check_data(docker: &Docker, container_name: &str, keyspace: &str, table: &str, expected_count: usize) -> bool {
    let exec = docker.create_exec(
        container_name,
        CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["cqlsh", "-e", &format!("SELECT COUNT(*) FROM {}.{}", keyspace, table)]),
            ..Default::default()
        },
    ).await.unwrap();

    match docker.start_exec(&exec.id, None).await.unwrap() {
        StartExecResults::Attached { mut output, .. } => {
            let mut success = false;
            let mut output_str = String::new();
            while let Some(msg) = output.next().await {
                if let Ok(msg) = msg {
                    output_str.push_str(&msg.to_string());
                    if let Some(count_str) = msg.to_string().split_whitespace().last() {
                        if let Ok(count) = count_str.parse::<usize>() {
                            if count == expected_count {
                                success = true;
                            }
                        }
                    }
                }
            }
            if success {
                pass(format!("✅ Table '{}.{}' has {} rows as expected: {}", keyspace, table, expected_count, output_str).as_str());
            } else {
                fail(format!("❌ Table '{}.{}' does not have {} rows: {}", keyspace, table, expected_count, output_str).as_str());
            }
            success
        },
        _ => {
            fail(format!("❌ Failed to check data in table '{}.{}'", keyspace, table).as_str());
            false
        },
    }
}
