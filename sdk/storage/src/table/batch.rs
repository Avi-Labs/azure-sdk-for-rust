use crate::BatchOperation;
use uuid::Uuid;

#[derive(Debug)]
pub struct Batch {
    batch_uuid: Uuid,
    change_set_uuid: Uuid,
    batch_operations: Vec<BatchOperation>,
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            batch_uuid: Uuid::new_v4(),
            change_set_uuid: Uuid::new_v4(),
            batch_operations: Vec::new(),
        }
    }
}

impl Batch {
    pub fn add(&mut self, batch_operation: BatchOperation) -> &mut Self {
        self.batch_operations.push(batch_operation);
        self
    }

    pub(crate) fn to_string(&self) -> Result<String, http::header::ToStrError> {
        let mut s = String::new();

        for batch_operation in self.batch_operations.iter() {
            s.push_str(&format!("--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\n", self.change_set_uuid.to_hyphenated_ref()));
            s.push_str(&format!(
                "{} {} HTTP/1.1\n",
                batch_operation.request.method(),
                batch_operation.request.uri()
            ));
            for (header_name, header_value) in batch_operation.request.headers() {
                s.push_str(&format!("{}: {}\n", header_name, header_value.to_str()?));
            }

            s.push('\n');
            if !batch_operation.request.body().is_empty() {
                s.push_str(batch_operation.request.body());
                s.push('\n');
            }
        }

        Ok(format!(
            "--batch_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n{}\n--batch_{}--\n",
            self.batch_uuid.to_hyphenated_ref(),
            self.change_set_uuid.to_hyphenated_ref(),
            s,
            self.batch_uuid.to_hyphenated_ref()
        ))
    }
}
