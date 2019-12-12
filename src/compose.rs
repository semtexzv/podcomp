use crate::prelude::*;

use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use crate::GlobalOpts;
use std::str::FromStr;


const DEFAULT_PATHS: &[&'static str] = &[
    "docker-compose.yml",
    "docker-compose.yaml",
    "podman-compose.yml",
    "podman-compose.yaml",
    "container-compose.yml",
    "container-compose.yaml",
];

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ByteAmount {
    Str(String),
    Num(usize),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Mapping {
    Map(HashMap<String, String>),
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StrOrList {
    Str(String),
    List(Vec<String>),
}

#[derive(Debug, Default, Deserialize)]
pub struct Build {
    #[serde(flatten)]
    #[serde(deserialize_with = "string_or_struct")]
    pub data: BuildData
}

#[derive(Debug, Default, Deserialize)]
pub struct BuildData {
    pub context: String,
    pub dockerfile: Option<String>,
    pub args: Option<HashMap<String, String>>,
    pub cache_from: Option<Vec<String>>,
    pub labels: Option<Mapping>,
    pub shm_size: Option<ByteAmount>,
    pub target: Option<String>,
}

impl FromStr for BuildData {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BuildData {
            context: s.to_string(),
            ..Default::default()
        })
    }
}


#[derive(Debug, Deserialize)]
pub enum EndpointMode {
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "dnsrr")]
    Dbnsrr,
}

#[derive(Debug, Deserialize)]
pub enum Mode {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "replicated")]
    Replicated,
}

#[derive(Debug, Deserialize)]
pub struct StackDeploy {
    endpoint_mode: Option<EndpointMode>,
    labels: Option<HashMap<String, String>>,
    mode: Option<Mode>,
    //placement : Option<Placement>,
    replicas: Option<usize>,
    //resurces : OPtion<Resources>,
    //restart_policy : Option<RestartPolicy>,
}

#[derive(Debug, Deserialize)]
pub struct UpDeploy {
    devices: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TmpFs {
    size: ByteAmount,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VolumeRef {
    Simple(String),
    Complex {
        #[serde(rename = "type")]
        typ: String,
        source: Option<String>,
        target: String,
        read_only: bool,
        tmpfs: Option<TmpFs>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    driver: Option<String>,
    driver_opts: Option<Mapping>,
    external: bool,
    labels: Option<Mapping>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    driver: Option<String>,
    driver_opts: Option<Mapping>,
    enable_ipv6: Option<String>,
    internal: bool,
    external: bool,
    labels: Option<Mapping>,
}

#[derive(Debug, Deserialize)]
pub enum Restart {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "on-failure")]
    OnFail,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub build: Option<Build>,
    pub image: Option<String>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub cgroup_parent: Option<String>,
    pub command: Option<StrOrList>,
    // TODO: Config or reference to config
    pub configs: Option<Vec<String>>,
    pub container_name: Option<String>,

    #[serde(default)]
    pub depends_on: Vec<String>,
    pub deploy: Option<UpDeploy>,

    pub volumes: Option<Vec<VolumeRef>>,
    pub volume_driver: Option<String>,
    pub volumes_from: Option<String>,

    pub labels: Option<Mapping>,
    // TODO
    pub logging: Option<yaml::Value>,
    // TODO : Detailed parse
    pub network_mode: Option<String>,
    pub networks: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,

    pub security_opt: Option<Vec<String>>,

    pub dns: Option<StrOrList>,
    pub dns_search: Option<StrOrList>,
    pub entrypoint: Option<StrOrList>,
    pub env_file: Option<StrOrList>,
    pub environment: Option<Mapping>,
    pub expose: Option<Vec<String>>,
    pub external_links: Option<Vec<String>>,
    pub extra_hosts: Option<Vec<String>>,
    // TODO: Healthcheck
    pub healthcheck: Option<yaml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Compose {
    pub version: String,
    pub services: HashMap<String, Service>,

    /// Contains an ordered list of containers on which we want to perform actions
    #[serde(skip_deserializing)]
    pub containers: Vec<String>,
    /// Contains the podman pod name in which we are operating
    #[serde(skip_deserializing)]
    pub pod: Option<String>,
    /// Contains the runtime information for running podman commands
    #[serde(skip_deserializing, default)]
    pub podman: Podman,

}


#[derive(Debug)]
pub struct Podman {
    podman: String,
    dry_run: bool,
}

impl Default for Podman {
    fn default() -> Self {
        Podman {
            podman: "podman".to_string(),
            dry_run: false,
        }
    }
}

impl Podman {
    pub fn run(&self, args: Vec<impl Into<String>>) -> Result<()> {
        let args: Vec<String> = args.into_iter().map(|v| v.into())
            .collect();
        info!("Running {:?} with args : {:?}", self.podman, args);
        let out = std::process::Command::new(&self.podman)
            .args(args)
            .status().unwrap();

        Ok(())
    }
}

impl Compose {
    pub fn resolve_deps(&mut self) -> Result<()> {
        loop {
            let mut modified = false;
            let mut to_add: HashMap<String, Vec<String>> = HashMap::new();
            for (ref k, ref mut s) in self.services.iter() {
                for dep in s.depends_on.iter() {
                    to_add.entry(k.to_string()).or_insert(vec![])
                        .append(&mut self.services.get(dep).unwrap().depends_on.clone());
                }
            }
            for (s, to_add) in to_add.into_iter() {
                let deps = &mut self.services.get_mut(&s).unwrap().depends_on;
                for new in to_add.into_iter() {
                    if !deps.contains(&new) {
                        deps.push(new);
                        modified = true;
                    }
                }
            }

            if !modified {
                break;
            }
        }

        for (name, svc) in self.services.iter() {
            if svc.depends_on.contains(&name) {
                return Err("Cycle detected in service dependencies".into());
            }
        }

        Ok(())
    }
}

pub fn parse_compose(opts: &mut GlobalOpts) -> Result<Compose> {
    let mut data = None;

    if let Some(ref file) = opts.file {
        data = Some(std::fs::read(file)?);
    }

    for f in DEFAULT_PATHS {
        if std::path::Path::new(f).exists() {
            data = Some(std::fs::read(f)?);
        }
    }

    if data.is_none() {
        return Err("No file provided and no default docker-compose.yml could be found".into());
    }

    let data = data.unwrap();
    if opts.project_name.is_none() {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir = current_dir.components().last().unwrap().as_os_str().to_str().unwrap();
        opts.project_name = Some(current_dir.to_string())
    }

    dotenv::dotenv();

    let project_name = opts.project_name.clone().unwrap();

    let mut compose: Compose = yaml::from_slice(&data)?;
    compose.resolve_deps()?;

    let mut ordered_svc_names = compose.services.iter()
        .collect::<Vec<_>>();
    ordered_svc_names.sort_by_cached_key(|(k, v)| v.depends_on.len());


    let mut ordered_svc_names: Vec<_> = ordered_svc_names
        .into_iter()
        .map(|(k, v)| k.to_string())
        .collect();
    println!("Ordered svcs {:?}", ordered_svc_names);

    for svc_name in ordered_svc_names.iter() {
        let svc = compose.services.get_mut(svc_name).unwrap();
        svc.container_name.get_or_insert(format!("{}_{}", project_name, svc_name));
        svc.image.get_or_insert(format!("{}_{}", project_name, svc_name));
    }
    compose.containers = ordered_svc_names;

    if let Some(path) = &opts.podman_path {
        compose.podman.podman = path.clone()
    }

    return Ok(compose);
}

fn container_to_args(compose: &Compose, container: &str, detached: bool, cmd: &str) {
    let cnt: &Service = compose.services.get(container).unwrap();

    let mut podman_args: Vec<String> = vec![cmd.to_string(), format!("--name={}", cnt.container_name.clone().unwrap())];

    if detached {
        podman_args.push("-d".to_string())
    }

    if let Some(pod) = &compose.pod {
        podman_args.push(format!("--pod={}", pod));
    }
}