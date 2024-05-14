use systemctl::*;

use crate::CustomError;

pub fn start_vde(build: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_vde = Unit::new();
    new_vde.name = format!("openqa-vde-switch@{}", build.to_owned());

    match new_vde.start() {
        Ok(exit_status) => exit_status,
        Err(err) => {
            return Err(Box::new(CustomError(format!(
                "Error: can't start {}.\n{}",
                new_vde.name, err
            ))));
        }
    };

    Ok(())
}
