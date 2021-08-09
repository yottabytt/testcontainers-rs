use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const CONTAINER_IDENTIFIER: &str = "vectorized/redpanda";
const DEFAULT_TAG: &str = "v21.7.6";

#[derive(Debug, Default, Clone)]
pub struct RedpandaArgs;

impl IntoIterator for RedpandaArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        vec![].into_iter()
    }
}

#[derive(Debug)]
pub struct Redpanda {
    tag: String,
    arguments: RedpandaArgs,
}

impl Default for Redpanda {
    fn default() -> Self {
        Redpanda {
            tag: DEFAULT_TAG.to_string(),
            arguments: RedpandaArgs {},
        }
    }
}

impl Image for Redpanda {
    type Args = RedpandaArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        format!("{}:{}", CONTAINER_IDENTIFIER, &self.tag)
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "Successfully started Redpanda!",
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
        Redpanda { arguments, ..self }
    }
}

impl Redpanda {
    pub fn with_tag(self, tag_str: &str) -> Self {
        Redpanda {
            tag: tag_str.to_string(),
            ..self
        }
    }
}
