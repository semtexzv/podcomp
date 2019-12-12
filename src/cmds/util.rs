use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use std::error::Error;
use crate::GlobalOpts;


#[derive(Debug, Clone, StructOpt)]
pub struct VersionOpts {}

impl Command for VersionOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<Error>> {
        compose.podman.run(vec!["--version"]).unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
pub struct PsOpts {
    #[structopt(short = "q", long = "quiet")]
    quiet_pull: bool,
}

impl Command for PsOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<Error>> {
        Ok(())
    }
}

#[derive(Debug, Clone, StructOpt)]
pub struct LogsOpts {
    #[structopt(short = "f", long = "follow")]
    follow: bool,
    #[structopt(short = "t", long = "timestamps")]
    timestamps: bool,

    service: String,
}

impl Command for LogsOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<Error>> {
        Ok(())
    }
}
