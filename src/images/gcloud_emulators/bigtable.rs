use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const CONTAINER_IDENTIFIER: &str = "google/cloud-sdk";
const DEFAULT_TAG: &str = "353.0.0";

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8086;
const CMD: &str = "gcloud beta emulators bigtable start";

#[derive(Debug, Clone)]
pub struct BigtableArgs {
    pub host: String,
    pub port: u16,
}

impl IntoIterator for BigtableArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let mut args = CMD.split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        args.push("--host-port".to_owned());
        args.push(format!("{}:{}", self.host, self.port));
        args.into_iter()
    }
}

#[derive(Debug)]
pub struct Bigtable {
    tag: String,
    arguments: BigtableArgs,
}

impl Default for Bigtable {
    fn default() -> Self {
        Bigtable {
            tag: DEFAULT_TAG.to_string(),
            arguments: BigtableArgs::default(),
        }
    }
}

impl Default for BigtableArgs {
    fn default() -> Self {
        BigtableArgs {
            host: HOST.to_owned(),
            port: PORT,
        }
    }
}

impl Image for Bigtable {
    type Args = BigtableArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        format!("{}:{}", CONTAINER_IDENTIFIER, &self.tag)
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "[bigtable] Cloud Bigtable emulator running on",
        )]
    }

    fn args(&self) -> <Self as Image>::Args {
        self.arguments.clone()
    }

    fn env_vars(&self) -> Self::EnvVars {
        HashMap::new()
    }

    fn volumes(&self) -> Self::Volumes {
        HashMap::new()
    }

    fn with_args(self, arguments: <Self as Image>::Args) -> Self {
        Bigtable { arguments, ..self }
    }

    fn expose_port(&self) -> Option<u16> {
        Some(PORT)
    }
}

impl Bigtable {
    pub fn with_tag(self, tag_str: &str) -> Self {
        Bigtable {
            tag: tag_str.to_string(),
            ..self
        }
    }
}
