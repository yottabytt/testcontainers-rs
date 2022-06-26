use crate::{core::WaitFor, Image};

const NAME: &str = "fsouza/fake-gcs-server";
const TAG: &str = "1.38";

#[derive(Debug, Default)]
pub struct CloudStorage;

impl Image for CloudStorage {
    type Args = ();

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
