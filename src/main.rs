use structopt::StructOpt;
use std::collections::HashMap;

pub mod prelude;
pub mod compose;
pub mod cmds;

use crate::{
    cmds::{
        stop::StopOpts,
        start::StartOpts,
        util::{PsOpts, LogsOpts, VersionOpts},
        run::RunOpts,
        up::UpOpts,
        build::BuildOpts,
        push::PushOpts,
        pull::PullOpts,
        restart::RestartOpts,
        down::DownOpts,
    },
    compose::Compose,
};
use crate::cmds::Command;
use std::error::Error;

#[derive(Debug, Clone, StructOpt)]
#[structopt(about = "Podman compose alternative")]
pub struct Opts {
    #[structopt(flatten)]
    pub global: GlobalOpts,

    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Clone, StructOpt)]
pub struct GlobalOpts {
    #[structopt(short = "f", long = "file")]
    file: Option<String>,
    #[structopt(short = "p", long = "project-name")]
    project_name: Option<String>,
    #[structopt(long = "podman-path")]
    podman_path: Option<String>,
    #[structopt(long = "no-ansi")]
    no_ansi: bool,
    #[structopt(long = "no-cleanup")]
    no_cleanup: bool,
    #[structopt(long = "dry-run")]
    dry_run: bool,
    #[structopt(long = "transform-policy")]
    transform_policy: Option<String>,
}

#[derive(Debug, Clone, StructOpt)]
pub enum SubCommand {
    Version(VersionOpts),
    Pull(PullOpts),
    Push(PushOpts),
    Build(BuildOpts),
    Up(UpOpts),
    Run(RunOpts),
    Down(DownOpts),
    Ps(PsOpts),
    Start(StartOpts),
    Stop(StopOpts),
    Restart(RestartOpts),
    Logs(LogsOpts),
}

impl Command for SubCommand {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        match opts {
            SubCommand::Version(o) => Command::run(compose, global, o),
            SubCommand::Pull(o) => Command::run(compose, global, o),
            SubCommand::Push(o) => Command::run(compose, global, o),
            SubCommand::Build(o) => Command::run(compose, global, o),
            SubCommand::Up(o) => Command::run(compose, global, o),
            SubCommand::Run(o) => Command::run(compose, global, o),
            SubCommand::Down(o) => Command::run(compose, global, o),
            SubCommand::Ps(o) => Command::run(compose, global, o),
            SubCommand::Start(o) => Command::run(compose, global, o),
            SubCommand::Stop(o) => Command::run(compose, global, o),
            SubCommand::Restart(o) => Command::run(compose, global, o),
            SubCommand::Logs(o) => Command::run(compose, global, o),
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let mut opts: Opts = Opts::from_args();
    let mut c = compose::parse_compose(&mut opts.global).unwrap();
    {
//        panic!("c : {:#?}", c);
        let _ = Command::run(c, opts.global.clone(), opts.subcommand.clone()).unwrap();
    }
    /*
    match opts {
        SubCommand::Up { detach, services, .. } => if !services.is_empty() {},
        _ => {
            panic!()
        }
    }
    */
}
