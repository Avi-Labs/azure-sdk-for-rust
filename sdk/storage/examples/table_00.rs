#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::clients::{AsStorageClient, StorageAccountClient};
use azure_storage::table::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let table_service = storage_account_client
        .as_storage_client()
        .as_table_service_client()?;

    let mut stream = Box::pin(table_service.list().top(2).stream());
    while let Some(response) = stream.next().await {
        println!("response = {:?}\n", response);
    }

    Ok(())
}
