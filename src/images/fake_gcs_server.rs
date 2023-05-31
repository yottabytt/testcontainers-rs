use crate::{core::WaitFor, Image, ImageArgs};

const NAME: &str = "yottabytt/fake-gcs-server";
const TAG: &str = "10e9c4c";

#[derive(Debug, Clone)]
pub struct CloudStorageArgs {
    pub scheme: String,
}

impl Default for CloudStorageArgs {
    fn default() -> Self {
        CloudStorageArgs {
            scheme: "http".to_owned(),
        }
    }
}

impl ImageArgs for CloudStorageArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let mut args = Vec::new();
        args.push("-scheme".to_string());
        args.push(self.scheme);
        Box::new(args.into_iter())
    }
}

#[derive(Debug, Default)]
pub struct CloudStorage;

impl Image for CloudStorage {
    type Args = CloudStorageArgs;

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr("server started at")]
    }
}
