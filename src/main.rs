use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A JavaScript package manager.")]
enum Wave {
    Init {
        #[structopt(short, long)]
        yes: bool,
        name: String,
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

fn main() {
    let args = Wave::from_args();
    print!("{:?}", args)
}
