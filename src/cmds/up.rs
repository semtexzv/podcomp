use crate::prelude::*;
use crate::cmds::Command;
use crate::compose::Compose;
use crate::GlobalOpts;

#[derive(Debug, Clone, StructOpt)]
pub struct UpOpts {
    #[structopt(short = "d", long = "detach")]
    detach: bool,
    #[structopt(long = "quiet-pull", help = "Pull without printing progress")]
    quiet_pull: bool,
    #[structopt(long = "no-deps", help = "Don't start linked services")]
    no_deps: bool,
    #[structopt(long = "force-recreate")]
    force_recreate: bool,
    #[structopt(long = "always-recreate-deps")]
    always_recreate_deps: bool,
    #[structopt(long = "no-build")]
    no_build: bool,
    #[structopt(long = "no-start")]
    no_start: bool,
    #[structopt(long = "build")]
    build: bool,
    #[structopt(long = "abort-on-container-exit")]
    abort_on_container_exit: bool,
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    timeout: f32,
    #[structopt(short = "V", long = "renew-anon-volumes")]
    renew_anon_volumes: bool,
    #[structopt(long = "remove-orphans")]
    remove_orphans: bool,
    #[structopt(long = "scale")]
    scale: Vec<String>,
    #[structopt(long = "exit-code-from")]
    exit_code_from: Option<String>,

    services: Vec<String>,
}

impl Command for UpOpts {
    fn run(compose: Compose, global: GlobalOpts, opts: Self) -> Result<(), Box<dyn Error>> {
        let podman_cmd = if opts.detach { "create" } else { "run" };
        let mut handles = vec![];
        for cnt in compose.containers.iter() {
            handles.push(std::thread::spawn(|| {

            }))
        }


        Ok(())
    }
}