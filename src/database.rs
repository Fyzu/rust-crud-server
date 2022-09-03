use mongodb::{Database, options::ClientOptions, Client};

pub async fn setup_database(url: String, db_name: String) -> mongodb::error::Result<Database> {
    let mut options = ClientOptions::parse(url)
        .await
        .expect("cannot parse MONGODB_URL");

    options.app_name = Some("Todo CRUD".to_owned());

    let client = Client::with_options(options)?;

    Ok(client.database(db_name.as_str()))
}