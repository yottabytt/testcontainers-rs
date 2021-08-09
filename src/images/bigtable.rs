use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const CONTAINER_IDENTIFIER: &str = "mechiru/bigtable-emulator";
const DEFAULT_TAG: &str = "273.0.0-alpine";

#[derive(Debug, Default, Clone)]
pub struct BigtableArgs;

impl IntoIterator for BigtableArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        vec![].into_iter()
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
            arguments: BigtableArgs {},
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
            "[bigtable] Cloud Bigtable emulator running on [::]:8086"
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
