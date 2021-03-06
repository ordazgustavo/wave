use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A JavaScript package manager.")]
pub struct Wave {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Add {
        #[structopt(short = "D", long = "save-dev")]
        development: bool,
        #[structopt(short = "E", long = "save-exact")]
        exact: bool,
        #[structopt(parse(from_str = parse_key_val))]
        packages: Vec<(String, String)>,
    },
    Init {
        #[structopt(short, long)]
        yes: bool,
        name: Option<String>,
    },
    Install,
    List {
        #[structopt(parse(from_str = parse_key_val))]
        packages: Vec<(String, String)>,
    },
    Uninstall {
        #[structopt(parse(from_str = parse_key_val))]
        packages: Vec<(String, String)>,
    },
}

fn parse_key_val(s: &str) -> (String, String) {
    if let Some((key, val)) = s.rsplit_once('@') {
        (key.to_owned(), val.to_owned())
    } else {
        (s.to_owned(), "latest".to_owned())
    }
}
