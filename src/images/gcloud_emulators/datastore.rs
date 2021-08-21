use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const CONTAINER_IDENTIFIER: &str = "google/cloud-sdk";
const DEFAULT_TAG: &str = "353.0.0";

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8081;
const CMD: &str = "gcloud beta emulators datastore start";

#[derive(Debug, Clone)]
pub struct DatastoreArgs {
    pub project: String,
    pub host: String,
    pub port: u16,
}

impl IntoIterator for DatastoreArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let mut args = CMD.split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        args.push("--project".to_owned());
        args.push(self.project);
        args.push("--host-port".to_owned());
        args.push(format!("{}:{}", self.host, self.port));
        args.into_iter()
    }
}

#[derive(Debug)]
pub struct Datastore {
    tag: String,
    arguments: DatastoreArgs,
}

impl Default for Datastore {
    fn default() -> Self {
        Datastore {
            tag: DEFAULT_TAG.to_string(),
            arguments: DatastoreArgs::default(),
        }
    }
}

impl Default for DatastoreArgs {
    fn default() -> Self {
        DatastoreArgs {
            project: "test-project".to_owned(),
            host: HOST.to_owned(),
            port: PORT,
        }
    }
}

impl Image for Datastore {
    type Args = DatastoreArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        format!("{}:{}", CONTAINER_IDENTIFIER, &self.tag)
    }

    // TODO: Logs had "please migrate to the emulator health check endpoint (/)"
    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "[datastore] Dev App Server is now running",
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
        Datastore { arguments, ..self }
    }

    fn expose_port(&self) -> Option<u16> {
        Some(PORT)
    }
}

impl Datastore {
    pub fn with_tag(self, tag_str: &str) -> Self {
        Datastore {
            tag: tag_str.to_string(),
            ..self
        }
    }
}
