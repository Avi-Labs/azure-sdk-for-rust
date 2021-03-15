use crate::table::change_set::{ChangeSet, ToChangeSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Batch {
    uuid: Uuid,
    change_set: ChangeSet,
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            change_set: ChangeSet::default(),
        }
    }
}

impl Batch {
    pub(crate) fn finalize(&self) -> String {
        format!(
            "--batch_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n{}\n--batch_{}--\n",
            self.uuid.to_hyphenated(),
            self.change_set.uuid().to_hyphenated(),
            self.change_set.finalize(),
            self.uuid.to_hyphenated()
        )
    }
}
