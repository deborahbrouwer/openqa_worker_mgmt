use openqa_worker_mgmt::{
    openqa_vde::start_vde,
    openqa_worker::{print_workers, start_worker},
};

fn main() {

    //openqa-cli api --host http://54.236.109.193 --json jobs | jq -r '.jobs[] | select(.state == "scheduled" and .settings.ARCH == "x86_64" and .settings.WORKER_CLASS !=  "qemu_x86_64") | .settings.BUILD'

    let build = "Fedora-IoT-41-20240513.0";

    if let Err(err) = start_vde(&build) {
        eprintln! {"{}", err};
        return;
    }

    if let Err(err) = start_worker("5", &build, "x86_64") {
        eprintln! {"{}", err};
        return;
    }

    if let Err(err) = print_workers() {
        eprintln! {"{}", err};
        return;
    }
}
