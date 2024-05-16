use openqa_worker_mgmt::{
    openqa_vde::*,
    openqa_worker::*,
    openqa_build_reporter::*,
};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <openqa-build-reporter path>", args[0]);
        return;
    }
    let path = &args[1];

    if let Err(err) = start_build_reporter() {
        eprintln! {"{}", err};
        return;
    }
    //TODO handle proxy errors
    println!("Waiting for openqa-cli api queries.");
    thread::sleep(Duration::from_secs(60));

    // Since the switches to stop is calculated using the workers to stop,
    // stop the switches before stopping the workers.
    println!("Stopping vde switches:");
    let vde_switches_to_stop = match get_vde_switches_to_stop(path) {
        Ok(vde_switches_to_stop) => vde_switches_to_stop,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    for switch in &vde_switches_to_stop {
        println!("\t{}", switch);
        if let Err(err) = stop_vde(switch) {
            eprintln! {"{}", err};
            return;
        }
    }

    println!("Stopping parallel workers:");
    let workers_to_stop = match get_workers_to_stop(path) {
        Ok(workers_to_stop) => workers_to_stop,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    for worker in &workers_to_stop {
        println!("\t{}", worker);
        if let Err(err) = stop_worker(worker) {
            eprintln! {"{}", err};
            return;
        }
    }

    println!("Starting vde switches:");
    let builds_to_start = match get_builds_to_start(path) {
        Ok(builds_to_start) => builds_to_start,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    for build in &builds_to_start {
        println!("\t{}", build);
        if let Err(err) = start_vde(build.as_str(), None) {
            eprintln! {"{}", err};
            return;
        }
    }

    // start three workers for each new build e.g. support_server needs three
    // TODO refine this based on test
    println!("Starting workers:");
    //Deliberately start parallel workers at two to avoid -1 = 0 errors
    let mut count:i32 = 2;
    for build in &builds_to_start {
        println!("\t{}", build);
        if let Err(err) = start_worker(Some(&count), Some(build.as_str()), None) {
            eprintln! {"{}", err};
            return;
        }
        count +=1;
        if let Err(err) = start_worker(Some(&count), Some(build.as_str()), None) {
            eprintln! {"{}", err};
            return;
        }
        count +=1;
        if let Err(err) = start_worker(Some(&count), Some(build.as_str()), None) {
            eprintln! {"{}", err};
            return;
        }
    }
    println!("See all switches and workers with: podman ps -a --format \"{{{{.Names}}}}\"");

}
