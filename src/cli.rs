use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A JavaScript package manager.")]
pub struct Wave {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Init {
        #[structopt(short, long)]
        yes: bool,
        name: Option<String>,
    },
    Install {
        #[structopt(short = "D", long = "save-dev")]
        development: bool,
        #[structopt(short = "E", long = "save-exact")]
        exact: bool,
        packages: Vec<String>,
    },
    List {
        packages: Vec<String>,
    },
    Uninstall {
        packages: Vec<String>,
    },
}
