use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use futures::StreamExt;
use crate::monster::{fail, pass};

pub async fn check_nodetool_status(docker: &Docker, container_name: &str) -> bool {
    let exec = docker.create_exec(
        container_name,
        CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["nodetool", "statusbinary"]),
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
                    if msg.to_string().contains("running") {
                        success = true;
                    }
                }
            }
            if success {
                pass(format!("✅ nodetool statusbinary output: {}", output_str).as_str());
            } else {
                fail(format!("❌ nodetool statusbinary failed: {}", output_str).as_str());
            }
            success
        },
        _ => {
            fail("❌ nodetool statusbinary failed to reach a status of 'running'");
            false
        },
    }
}