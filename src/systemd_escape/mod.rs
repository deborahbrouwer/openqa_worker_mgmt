use crate::CustomError;
use std::io::Read;
use std::process::Child;

const SYSTEMD_ESCAPE_PATH: &str = "/usr/bin/systemd-escape";

fn spawn_child(args: Vec<&str>) -> std::io::Result<Child> {
    std::process::Command::new(SYSTEMD_ESCAPE_PATH)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
}

/// Equivalent to INSTANCE=$(systemd-escape ${OPENQA_WORKER_INSTANCE}#${BUILD}#${ARCH});
pub fn escape_name(service_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = spawn_child(vec![service_name])?;
    child.wait()?;

    let mut escaped_name_vec: Vec<u8> = Vec::new();

    let mut child_stdout = match child.stdout {
        Some(child_stdout) => child_stdout,
        None => {
            return Err(Box::new(CustomError(format!(
                "Error: no result in standard output."
            ))));
        }
    };

    child_stdout.read_to_end(&mut escaped_name_vec)?;
    let escaped_name = String::from_utf8(escaped_name_vec)?;
    Ok(escaped_name.trim().to_string())
}

pub fn unescaped_name(escaped_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = spawn_child(vec!["-u", escaped_name])?;
    child.wait()?;

    let mut service_name_vec: Vec<u8> = Vec::new();

    let mut child_stdout = match child.stdout {
        Some(child_stdout) => child_stdout,
        None => {
            return Err(Box::new(CustomError(format!(
                "Error: no result in standard output."
            ))));
        }
    };

    child_stdout.read_to_end(&mut service_name_vec)?;
    let service_name = String::from_utf8(service_name_vec)?;
    Ok(service_name.trim().to_string())
}
