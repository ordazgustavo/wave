use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A JavaScript package manager.")]
pub enum Wave {
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
