use crate::prelude::*;
use crate::cmds::Command;
use crate::GlobalOpts;
use crate::compose::Compose;

#[derive(Debug, Clone, StructOpt)]
pub struct PushOpts {
    #[structopt(long = "ignore-push-failures")]
    ignore_failures: bool,
    services: Vec<String>,
}

impl Command for PushOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        for (name, svc) in compose.services.iter() {
            if svc.build.is_none() {
                compose.podman.run(vec!["push".to_string(), svc.image.clone().unwrap()]);
            }
        }
        Ok(())
    }
}