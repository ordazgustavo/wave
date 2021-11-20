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
        #[structopt(parse(from_str = parse_key_val))]
        packages: Vec<(String, String)>,
    },
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
        (key.to_string(), val.to_string())
    } else {
        (s.to_string(), "latest".to_string())
    }
}
