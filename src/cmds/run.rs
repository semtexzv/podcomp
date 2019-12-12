use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct RunOpts {
    #[structopt(short = "d", long = "detach")]
    detach: bool,
    #[structopt(long = "name")]
    name: Option<String>,
    #[structopt(long = "entrypoint")]
    entrypoint: Option<String>,
    #[structopt(short = "e")]
    env_vars: Vec<String>,
    #[structopt(long = "no-deps", help = "Don't start linked services")]
    no_deps: bool,
    #[structopt(long = "rm")]
    rm: bool,
    #[structopt(short = "p", long = "publish")]
    publish: Vec<String>,
    #[structopt(long = "service-ports")]
    service_ports: bool,
    // TODO: Volume spec parser
    #[structopt(short = "v", long = "volume")]
    volumes: Vec<String>,
    #[structopt(short = "T")]
    no_tty: Vec<String>,
    #[structopt(short = "w", long = "workdir")]
    workdir: Vec<String>,
    #[structopt(help = "name of the service")]
    service: String,
    #[structopt(help = "Command and its arguments")]
    cmd: Vec<String>,
}

impl Command for RunOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
