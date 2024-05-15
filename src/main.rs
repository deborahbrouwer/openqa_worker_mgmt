use openqa_worker_mgmt::{
    openqa_vde::*,
    openqa_worker::*,
    openqa_build_reporter::*,
};
use std::thread;
use std::time::Duration;

fn main() {

    if let Err(err) = start_build_reporter() {
        eprintln! {"{}", err};
        return;
    }
    println!("Waiting for openqa-cli api queries.");
    thread::sleep(Duration::from_secs(30));

    let builds_to_stop = match get_builds_to_stop() {
        Ok(builds_to_stop) => builds_to_stop,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    println!("builds to stop");
    for build in &builds_to_stop {
        println!("{}", build);
    }

    //stop switches before stopping workers since the the builds of workers
    // that are not scheduled or worker are used to figure out which builds should be stopped
    print_vde_switches();
    let vde_switches_to_stop = match get_vde_switches_to_stop() {
        Ok(vde_switches_to_stop) => vde_switches_to_stop,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    for switch in &vde_switches_to_stop {
        if let Err(err) = stop_vde(switch) {
            eprintln! {"{}", err};
            return;
        }
    }
    print_vde_switches();

    print_workers();

    let workers_to_stop = match get_workers_to_stop() {
        Ok(workers_to_stop) => workers_to_stop,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    println!("stopping");
    for worker in &workers_to_stop {
        if let Err(err) = stop_worker(worker) {
            eprintln! {"{}", err};
            return;
        }
    }
    print_workers();

    print_vde_switches();
    let required_builds = match get_required_builds() {
        Ok(required_builds) => required_builds,
        Err(err) => {
            eprintln! {"{}", err};
            return;
        }
    };

    for build in &required_builds {
        if let Err(err) = start_vde(build.as_str(), None) {
            eprintln! {"{}", err};
            return;
        }
    }
    print_vde_switches();

    print_workers();
    // start two workers for each build
    let mut count:i32 = 2;
    for build in &required_builds {
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
    }
    print_workers();
}
