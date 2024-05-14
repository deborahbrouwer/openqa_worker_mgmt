use std::io::Read;
use std::process::Child;

use crate::CustomError;

const SYSTEMD_ESCAPE_PATH: &str = "/usr/bin/systemd-escape";

fn spawn_child(service_name: &String) -> std::io::Result<Child> {
    std::process::Command::new(SYSTEMD_ESCAPE_PATH)
        .arg(service_name)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
}

/// Equivalent to INSTANCE=$(systemd-escape ${OPENQA_WORKER_INSTANCE}#${BUILD}#${ARCH});
pub fn escape_name(service_name: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = spawn_child(service_name)?;
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
    Ok(escaped_name)
}
