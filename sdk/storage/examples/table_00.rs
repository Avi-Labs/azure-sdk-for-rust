#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::table::prelude::*;
use azure_storage::{
    clients::{AsStorageClient, StorageAccountClient},
    table::clients::AsTableClient,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyEntity {
    #[serde(rename = "PartitionKey")]
    pub city: String,
    pub name: String,
    #[serde(rename = "RowKey")]
    pub surname: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let table_name = std::env::args()
        .nth(1)
        .expect("please specify the table name as first command line parameter");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let table_service = storage_account_client
        .as_storage_client()
        .as_table_service_client()?;

    let table = table_service.as_table_client(table_name);
    let response = table.create().execute().await?;
    println!("response = {:?}\n", response);

    let entity = MyEntity {
        city: "Milan".to_owned(),
        name: "Francesco".to_owned(),
        surname: "Cogno".to_owned(),
    };

    let response = table.insert().return_entity(false).execute(&entity).await?;
    println!("response = {:?}\n", response);

    let mut stream = Box::pin(table_service.list().top(2).stream());
    while let Some(response) = stream.next().await {
        println!("response = {:?}\n", response);
    }

    let response = table.delete().execute().await?;
    println!("response = {:?}\n", response);

    Ok(())
}
