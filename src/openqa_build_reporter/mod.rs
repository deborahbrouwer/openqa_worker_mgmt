use crate::openqa_worker::*;
use crate::CustomError;
use systemctl::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn start_build_reporter() -> Result<(), Box<dyn std::error::Error>> {
    let mut build_reporter = Unit::new();
    build_reporter.name = "openqa-build-reporter".to_string();

    match build_reporter.start() {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't start openqa-build-reporter.\n{}",
                err
            ))));
        }
    };
    Ok(())
}

pub fn get_scheduled_builds() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = "/home/fedora/my_cloud/openqa-build-reporter/scheduled_builds/scheduled_builds";
    let build_file = File::open(path)?;
    let reader = BufReader::new(build_file);

    let mut builds = Vec::new();
    for line in reader.lines() {
        builds.push(line?);
    }

    let mut builds_unique = Vec::new();
    for line in &builds {
        if !builds_unique.contains(line) {
            builds_unique.push(line.to_string());
        }
    }

    Ok(builds_unique)
}

pub fn get_running_builds() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = "/home/fedora/my_cloud/openqa-build-reporter/running_builds/running_builds";
    let build_file = File::open(path)?;
    let reader = BufReader::new(build_file);

    let mut builds = Vec::new();
    for line in reader.lines() {
        builds.push(line?);
    }

    let mut builds_unique = Vec::new();
    for line in &builds {
        if !builds_unique.contains(line) {
            builds_unique.push(line.to_string());
        }
    }

    Ok(builds_unique)
}

pub fn get_required_builds() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut required_builds = get_scheduled_builds().unwrap();
    required_builds.extend(get_running_builds().unwrap());
    Ok(required_builds)
}

pub fn get_current_builds() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let worker_list = get_workers().unwrap();

    let mut builds = Vec::new();
    for worker in worker_list {
        if let Some(build) = worker.split('#').nth(1) {
            if !build.is_empty() {
                builds.push(build.to_string());
            }
        }
    }
    let mut builds_unique = Vec::new();
    for build in &builds {
        if !builds_unique.contains(build) {
            builds_unique.push(build.to_string());
        }
    }

    Ok(builds_unique)
}

pub fn print_current_builds() -> Result<(), Box<dyn std::error::Error>> {
    let build_list = get_current_builds().unwrap();

    for build in build_list {
        println!("{:?}", build);
    }
    Ok(())
}

pub fn get_builds_to_stop() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cur = get_current_builds()?;
    let req = get_required_builds()?;

    let builds_to_stop = cur
        .iter()
        .filter(|current_build| !req.contains(current_build))
        .cloned()
        .collect();

    Ok(builds_to_stop)
}
