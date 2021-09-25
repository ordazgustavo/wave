use std::path::Path;
use std::time::Instant;

use structopt::StructOpt;

mod cli;
mod fs;
mod logger;
mod package;

use cli::Wave;

fn init(name: Option<String>) {
    // Use provided name
    // If the name is not provided attempt to use the cwd name
    // Else, set name as empty string, should we just throw an error?
    let name = name.unwrap_or(fs::cwd().unwrap_or(String::new()));
    let package = package::Package {
        name: name.to_owned(),
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
            logger::error("Creating package.json");
            println!("! {:?}", error.classify());
        }
    }
}

fn main() {
    let args = Wave::from_args();

    let now = Instant::now();

    match args {
        Wave::Init { yes: _, name } => init(name),
        // Wave::Install {
        //     development,
        //     exact,
        //     packages,
        // } => todo!(),
        // Wave::List { packages } => todo!(),
        // Wave::Uninstall { packages } => todo!(),
        _ => todo!(),
    }

    println!("✨ Done in {}s.", now.elapsed().as_secs_f32());
}
