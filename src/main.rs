use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use structopt::StructOpt;

mod cli;
mod fs;
mod logger;
mod package;

use cli::Wave;

use crate::package::Package;

fn init(name: Option<String>) {
    // Use provided name
    // If the name is not provided attempt to use the cwd name
    // Else, set name as empty string, should we just throw an error?
    let name = name.or(fs::cwd());
    let package = package::Package {
        name,
        ..package::Package::default()
    };
    let package = package.to_json();

    match package {
        Ok(package) => {
            let path = &Path::new("package.json");
            let package_file = fs::echo(&package, path);

            match package_file {
                Ok(_) => logger::success("Saved package.json"),
                Err(error) => {
                    logger::error("Creating package.json");
                    println!("! {:?}", error.kind());
                }
            }
        }
        Err(error) => {
            logger::error("Serializing package.json");
            println!("! {:?}", error.classify());
        }
    }
}

fn install(packages: Vec<String>) {
    let package_path = Path::new("package.json");
    let package = fs::cat(&package_path);
    match package {
        Ok(lines) => {
            let package = Package::from_json(&lines);
            match package {
                Ok(package) => {
                    let mut map = HashMap::<String, String>::new();
                    for key in packages.into_iter() {
                        map.insert(key, "latest".to_string());
                    }
                    if let Some(prev_deps) = package.dependencies {
                        map.extend(prev_deps);
                    }
                    let new_package = Package {
                        dependencies: Some(map),
                        ..package
                    };
                    let package_json = new_package.to_json();

                    match package_json {
                        Ok(package_json) => {
                            let package_file = fs::echo(&package_json, package_path);

                            match package_file {
                                Ok(_) => logger::success("Saved package.json"),
                                Err(error) => {
                                    logger::error("Updating package.json");
                                    println!("! {:?}", error.kind());
                                }
                            }
                        }
                        Err(error) => {
                            logger::error("Serializing package.json");
                            println!("! {:?}", error.classify());
                        }
                    }
                }
                Err(error) => {
                    logger::error("Deserializing package.json");
                    println!("! {:?}", error.classify());
                }
            }
        }
        Err(error) => {
            logger::error("Reading package.json");
            println!("! {:?}", error.kind());
        }
    }
}

fn main() {
    let args = Wave::from_args();

    let now = Instant::now();

    match args {
        Wave::Init { yes: _, name } => init(name),
        Wave::Install {
            development: _,
            exact: _,
            packages,
        } => install(packages),
        // Wave::List { packages } => todo!(),
        // Wave::Uninstall { packages } => todo!(),
        _ => todo!(),
    }

    println!("✨ Done in {}s.", now.elapsed().as_secs_f32());
}
