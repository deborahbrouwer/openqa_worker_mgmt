use systemctl::*;

use crate::systemd_escape::*;
use crate::CustomError;

pub fn start_worker(
    openqa_worker_instance: &str,
    build: &str,
    arch: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let instance_name = openqa_worker_instance.to_owned() + "#" + build + "#" + arch;
    let instance_name_escaped = escape_name(&instance_name).unwrap();

    let mut new_worker = Unit::new();
    new_worker.name = format!("openqa-worker@{}", instance_name_escaped);

    // sudo systemctl start openqa-worker@${INSTANCE}.service
    match new_worker.start() {
        Ok(exit_status) => exit_status,
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't start {}.\n{}",
                instance_name, err
            ))));
        }
    };
    Ok(())
}

/// Equivalent to: sudo systemctl reset-failed openqa-worker@*.service;
pub fn reset_failed_workers() -> Result<(), Box<dyn std::error::Error>> {
    match systemctl::reset_failed("openqa-worker@*.service") {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't reset failed workers: {}",
                err
            ))));
        }
    };
    Ok(())
}

/// Equivalent to: sudo systemctl list-units --all 'openqa-worker@*.service'
pub fn get_workers() -> Result<Vec<UnitList>, Box<dyn std::error::Error>> {
    let worker_list =
        match systemctl::list_units(None, None, Some("openqa-worker@*.service"), false) {
            Ok(worker_list) => worker_list,
            Err(err) => {
                return Err(Box::new(CustomError(format!(
                    "Error: can't get worker list: {}",
                    err
                ))));
            }
        };
    Ok(worker_list)
}

pub fn print_workers() -> Result<(), Box<dyn std::error::Error>> {
    match get_workers() {
        Ok(worker_list) => {
            for unit in worker_list {
                println!("{}", unit.unit_file);
            }
        }
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't print workers: {}",
                err
            ))));
        }
    };
    Ok(())
}
