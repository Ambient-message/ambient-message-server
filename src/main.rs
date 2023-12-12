use ambient_message_server_lib::adapters::spi::db::db_connection::{DbConnection, DbOptions, DbContext};

use config::{*, ext::*};
use di::*;
use options::{*, ext::*};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let file = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("../../appsettings.json");

    let config = DefaultConfigurationBuilder::new()
        .add_json_file(file)
        .build()
        .unwrap();

    let provider = ServiceCollection::new()
        .apply_config::<DbOptions>(config.section("Data").as_config().into())
        .validate(
            |options| !options.database_url.is_empty(),
            "The database URL must be set.",
        )
        .add(DbConnection::transient())
        .build_provider()
        .unwrap();

    let db = provider.get_required::<dyn DbContext>();
    println!("Connecting to '{}'...", &db.database_url());
    &db.get_pool();

    Ok(())
}
