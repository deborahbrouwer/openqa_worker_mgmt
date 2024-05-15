use systemctl::*;

use crate::openqa_build_reporter::*;
use crate::systemd_escape::*;

use crate::CustomError;

fn instance_number_is_available(new_number: String) -> bool {
    let current_workers = get_workers().unwrap();

    for worker in current_workers {
        let current_number = worker.split('#').next().unwrap().to_string();
        if new_number == current_number {
            return false;
        }
    }
    true
}

pub fn start_worker(
    instance: Option<&i32>,
    build_option: Option<&str>,
    arch_option: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut openqa_worker_instance = match instance {
        Some(i) => {
            let mut num = *i;
            while !instance_number_is_available(num.to_string()) {
                num += 1;
            }
            num
        }
        None => {
            let mut num: i32 = 1;
            while !instance_number_is_available(num.to_string()) {
                num += 1;
            }
            num
        }
    };

    let arch = match arch_option {
        Some(arch) => arch,
        None => "x86_64",
    };

    let build = if let Some(build) = build_option {
        build
    } else {
        ""
    };

    let instance_name = openqa_worker_instance.to_string() + "#" + build + "#" + arch;
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

///  sudo systemctl list-units --all 'openqa-worker@*.service'
pub fn get_workers() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    reset_failed_workers()?;
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
    let mut worker_names = Vec::new();
    for unit in worker_list {
        let name_without_prefix = unit.unit_file.strip_prefix("openqa-worker@").unwrap();
        let escaped_name = name_without_prefix.strip_suffix(".service").unwrap();
        let unescaped_name = unescaped_name(&escaped_name).unwrap();
        worker_names.push(unescaped_name);
    }
    Ok(worker_names)
}

pub fn print_workers() -> Result<(), Box<dyn std::error::Error>> {
    let worker_list = get_workers().unwrap();
    println!("\nCurrent workers");
    for worker in worker_list {
        println!("{:?}", worker);
    }
    Ok(())
}

pub fn stop_worker(instance_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let instance_name_escaped = escape_name(&instance_name).unwrap();

    let mut new_worker = Unit::new();
    new_worker.name = format!("openqa-worker@{}", instance_name_escaped);

    // sudo systemctl stop openqa-worker@${INSTANCE}.service
    match new_worker.stop() {
        Ok(exit_status) => exit_status,
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't stop {}.\n{}",
                instance_name, err
            ))));
        }
    };
    Ok(())
}

pub fn get_workers_to_stop() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let builds_to_stop = get_builds_to_stop()?;

    let current_workers = get_workers()?;

    let mut workers_to_stop = Vec::new();

    for worker in current_workers {
        let _instance_number = worker.split('#').next();
        let worker_build = worker.split('#').nth(1);
        if worker_build.clone().unwrap().is_empty() {
            continue;
        }
        for build in &builds_to_stop {
            if worker_build.unwrap() == build {
                workers_to_stop.push(worker.clone());
            }
        }
    }

    Ok(workers_to_stop)
}
