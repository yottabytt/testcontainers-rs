use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const CONTAINER_IDENTIFIER: &str = "storytel/gcp-bigtable-emulator";
const DEFAULT_TAG: &str = "352.0.0";

#[derive(Debug, Clone)]
pub struct BigtableArgs {
    pub host: String,
    pub port: String,
}

impl IntoIterator for BigtableArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let mut args = Vec::new();
        args.push("--host".to_owned());
        args.push(self.host);
        args.push("--port".to_owned());
        args.push(self.port);
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
            host: "0.0.0.0".to_owned(),
            port: "8086".to_owned(),
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
        vec![WaitFor::message_on_stdout(
            "Cloud Bigtable emulator running on [::]:8086"
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
}

impl Bigtable {
    pub fn with_tag(self, tag_str: &str) -> Self {
        Bigtable {
            tag: tag_str.to_string(),
            ..self
        }
    }
}
