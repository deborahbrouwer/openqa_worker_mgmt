use crate::openqa_build_reporter::*;
use crate::systemd_escape::*;
use crate::CustomError;
use systemctl::*;

pub fn start_vde(build: &str, arch_option: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let arch = match arch_option {
        Some(arch) => arch,
        None => "x86_64",
    };

    let instance_name = build.to_string() + "#" + arch;
    let instance_name_escaped = escape_name(&instance_name).unwrap();
    let mut new_vde = Unit::new();
    new_vde.name = format!("openqa-vde-switch@{}", instance_name_escaped);

    // sudo systemctl start openqa-vde-switch@${INSTANCE}.service
    match new_vde.start() {
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

pub fn stop_vde(instance_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let instance_name_escaped = escape_name(&instance_name).unwrap();

    let mut new_vde = Unit::new();
    new_vde.name = format!("openqa-vde-switch@{}", instance_name_escaped);

    // sudo systemctl stop openqa-vde-switch@${INSTANCE}.service
    match new_vde.stop() {
        Ok(exit_status) => exit_status,
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't stop {}.\n{}",
                instance_name, err
            ))));
        }
    };

    //TODO delete the switch directory too
    Ok(())
}

/// sudo systemctl reset-failed openqa-vde-switch@*.service;
pub fn reset_failed_vde_switches() -> Result<(), Box<dyn std::error::Error>> {
    match systemctl::reset_failed("openqa-vde-switch@*.service") {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't reset failed vde: {}",
                err
            ))));
        }
    };
    Ok(())
}

/// sudo systemctl list-units --all 'openqa-vde-switch@*.service'
pub fn get_vde_switches() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    reset_failed_vde_switches()?;
    let vde_list =
        match systemctl::list_units(None, None, Some("openqa-vde-switch@*.service"), false) {
            Ok(vde_list) => vde_list,
            Err(err) => {
                return Err(Box::new(CustomError(format!(
                    "Error: can't get vde list: {}",
                    err
                ))));
            }
        };
    let mut vde_names = Vec::new();
    for unit in vde_list {
        let name_without_prefix = unit.unit_file.strip_prefix("openqa-vde-switch@").unwrap();
        let escaped_name = name_without_prefix.strip_suffix(".service").unwrap();
        let unescaped_name = unescaped_name(&escaped_name).unwrap();
        vde_names.push(unescaped_name);
    }
    Ok(vde_names)
}

pub fn print_vde_switches() -> Result<(), Box<dyn std::error::Error>> {
    let vde_list = get_vde_switches().unwrap();
    println!("\nCurrent vde switches");
    for switch in vde_list {
        println!("{:?}", switch);
    }
    Ok(())
}

pub fn get_vde_switches_to_stop() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let builds_to_stop = get_builds_to_stop()?;
    let current_switches = get_vde_switches()?;
    let mut switches_to_stop = Vec::new();

    for switch in current_switches {
        let switch_build = switch.split('#').next();
        for build in &builds_to_stop {
            if switch_build.unwrap() == build {
                switches_to_stop.push(switch.clone());
            }
        }
    }

    Ok(switches_to_stop)
}
