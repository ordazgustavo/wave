use std::path::Path;
use std::time::Instant;

use structopt::StructOpt;

mod cli;
mod fs;
mod logger;
mod package;

use cli::Wave;

fn init(name: Option<String>) {
    let name = name.unwrap_or(fs::cwd().unwrap_or(String::new()));
    let package = package::Package {
        name: name.to_owned(),
        ..package::Package::default()
    };

    let path = &Path::new("package.json");

    match fs::echo(&package.to_string(), path) {
        Ok(_) => logger::success("Saved package.json"),
        Err(why) => {
            logger::error("Creating package.json");
            println!("! {:?}", why.kind());
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
