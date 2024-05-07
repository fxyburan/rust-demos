use std::time::SystemTime;
use tokio_postgres::{Client, Error, NoTls};

struct PostgresManager {
    client: Client,
}

impl PostgresManager {
    async fn new(connection_string: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(PostgresManager { client })
    }

    async fn insert_record(
        &self,
        table_name: &str,
        name: &str,
        registration_time: SystemTime,
        login_time: SystemTime,
        is_active: bool,
    ) -> Result<(), Error> {
        // Define the SQL query with placeholders for parameters
        let query = format!("INSERT INTO \"{}\" (name, registration_time, login_time, is_active) VALUES ($1, $2, $3, $4)", table_name);

        // Execute the query with the values
        self.client
            .execute(
                &query,
                &[&name, &registration_time, &login_time, &is_active],
            )
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a new instance of PostgresManager
    let manager =
        PostgresManager::new("host=localhost user=clique dbname=clique_mpc_db password=123456")
            .await?;

    // Define the values to be inserted
    let table_name = "user";
    let name = "Alice";
    let registration_time = SystemTime::now();
    let login_time = SystemTime::now();
    let is_active = true;

    // Insert the record into the specified table
    manager
        .insert_record(table_name, name, registration_time, login_time, is_active)
        .await?;

    println!(
        "Inserted values into the '{}' table successfully.",
        table_name
    );

    Ok(())
}
