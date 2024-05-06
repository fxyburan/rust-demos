use chrono::{NaiveDateTime, Utc};
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Establish a connection to the database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=clique password=123456 dbname=clique_mpc_db",
        NoTls,
    )
    .await?;

    // Spawn a task to process the connection asynchronously
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Define the SQL query for inserting records
    let query = "INSERT INTO \"user\" (name, registration_time, login_time) VALUES ($1, $2, $3)";

    // Prepare the statement
    let statement = client.prepare(&query).await?;

    // Define the parameters for each record to be inserted
    let records = vec![
        (
            "John",
            NaiveDateTime::parse_from_str("2024-05-06 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-06 12:05:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            "Alice",
            NaiveDateTime::parse_from_str("2024-05-06 12:10:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-06 12:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            "Bob",
            NaiveDateTime::parse_from_str("2024-05-06 12:20:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-06 12:25:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ];

    // Iterate over the records and execute the insert statement for each
    for record in records {
        client
            .execute(&statement, &[&record.0, &record.1, &record.2])
            .await?;
    }

    println!("Records inserted successfully");

    Ok(())
}
