use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct PullOpts {
    #[structopt(long = "ignore-pull-failures")]
    ignore_failures: bool,
    #[structopt(long = "parallel")]
    parallel: bool,
    #[structopt(long = "no-parallel")]
    no_parallel: bool,
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    #[structopt(long = "include-deps")]
    include_deps: bool,

    services: Vec<String>,
}

impl Command for PullOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        for (name, svc) in compose.services.iter() {
            if svc.build.is_none() {
                compose.podman.run(vec!["pull".to_string(), svc.image.clone().unwrap()]);
            }
        }
        Ok(())
    }
}