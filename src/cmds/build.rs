use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::{Compose, Service};
use std::error::Error;
use crate::GlobalOpts;
use std::path::Path;

#[derive(Debug, Clone, StructOpt)]
pub struct BuildOpts {
    #[structopt(long = "pull")]
    pull: bool,
    #[structopt(long = "pull-always")]
    pull_always: bool,

    services: Vec<String>,
}

pub fn build_one(compose: &Compose, cnt: &Service, global: &GlobalOpts, opts: &BuildOpts) -> Result<()> {
    if cnt.build.is_none() {
        return Ok(());
    }

    let mut build = &cnt.build.as_ref().unwrap();

    let ctx = Path::new(&build.data.context);
    let dockerfile = ctx.join(build.data.dockerfile.clone().unwrap());
    info!("Dockerfile : {:?}", dockerfile);
    if !dockerfile.exists() {
        return Err(format!("Dockerfile does not exist in {:?}", ctx).into());
    }

    let mut build_args = vec!["build", "--no-cache",
                              "-t", &cnt.image.as_ref().unwrap(),
                              "-f", dockerfile.to_str().unwrap()];

    if opts.pull_always {
        build_args.push("--pull-always")
    } else if opts.pull {
        build_args.push("--pull")
    }

    compose.podman.run(build_args)
}

impl Command for BuildOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        for n in &compose.containers {
            let cnt = &compose.services[n];

            // Only build if there is something to build
            if cnt.build.is_some() {
                println!("--Building {}", n);
                build_one(&compose, &cnt, &global, &opts)?;
            }
        }
        Ok(())
    }
}