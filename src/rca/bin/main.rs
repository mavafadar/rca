use rca::cmd::{CargoCmd, Cmd};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let s = 10;
    let outdated_result = CargoCmd::new("outdated".to_string(), vec![]).execute()?;
    let audit_result = CargoCmd::new("audit".to_string(), vec![]).execute()?;
    let check_result = CargoCmd::new("check".to_string(), vec![]).execute()?;
    println!("# Outdated dependencies:\n{}", outdated_result);
    println!("# Vulnerable depedencies:\n{}", audit_result);
    println!("# Compilation checks:\n{}", check_result);
    Ok(())
}
