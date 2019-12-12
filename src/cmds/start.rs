use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct StartOpts {
    services: Vec<String>,
}

impl Command for StartOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}