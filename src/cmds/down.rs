use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct DownOpts {
    #[structopt(long = "rmi")]
    pull: Option<String>,
    #[structopt(long = "volumes")]
    pull_always: Vec<String>,
}

impl Command for DownOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        for cnt in compose.containers.iter() {
            compose.podman.run(vec!["stop", "-t=1", cnt])?;
        }
        for cnt in compose.containers.iter() {
            compose.podman.run(vec!["rm", "-t=1", &cnt])?;
        }
        Ok(())
    }
}