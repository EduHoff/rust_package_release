use std::collections::HashMap;

use clearscreen::clear;
use colored::Colorize;
use rust_package_release::aux::{check_dependency::{DependencyError, check_dependency}, dependencies::{DEPENDENCIES, Dependency}};




fn main() {

    clear().expect("Error: clear failed");


    let mut status_dependencies_map: HashMap<&Dependency, bool> = HashMap::new();
    for dep in DEPENDENCIES {
        
        let result = check_dependency(dep.name, dep.args);

        let status = result.is_ok(); 
        status_dependencies_map.insert(dep, status);
        
        match result {
            Ok(()) => {
                let msg = "is installed and running".green();
                println!("{} {}", dep.label.green(), msg);
            }
            Err(DependencyError::NotFound) => {
                let msg = "is not installed".red();
                eprintln!("{} {}", dep.label.red(), msg);
            }
            Err(DependencyError::ExecutionError) => {
                let msg = "installed, but it's not working correctly".red();
                eprintln!("{} {}", dep.label.red(), msg);
            }
        }
    }
    

    
    
}
