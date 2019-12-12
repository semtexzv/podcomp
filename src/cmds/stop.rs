use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct StopOpts {
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    timeout: f32,
    services: Vec<String>,
}


impl Command for StopOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}