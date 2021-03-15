use azure_core::errors::AzureError;
use serde::Serialize;
use uuid::Uuid;

pub trait ToChangeSet {
    fn to_change_set(&self) -> Result<String, url::ParseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ChangeSet {
    uuid: Uuid,
    payload: String,
}

impl Default for ChangeSet {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            payload: String::default(),
        }
    }
}

impl ChangeSet {
    pub(crate) fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    fn add_operation(&mut self) -> &mut Self {
        self.payload
            .push_str(&format!("--changeset_{}\n", self.uuid.to_hyphenated()));
        self.payload
            .push_str("Content-Type: application/http\nContent-Transfer-Encoding: binary\n\n");
        self
    }

    pub(crate) fn finalize(&self) -> String {
        format!(
            "{}\n\n--changeset_{}--\n",
            self.payload,
            self.uuid.to_hyphenated()
        )
    }

    pub fn insert<E: Serialize>(&mut self, entity: &E) -> Result<&mut Self, AzureError> {
        self.add_operation();

        self.payload.push_str(&format!(
            "{} {} HTTP/1.1\n \
            Content-Type: application/json\n \
            Accept: application/json;odata=minimalmetadata\n \
            Prefer: return-no-content\n\n{}",
            http::Method::POST,
            "url to do",
            serde_json::to_string(entity)?
        ));

        Ok(self)
    }
}
