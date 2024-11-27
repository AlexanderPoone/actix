use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::User};     // import self

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = include_str!("../sql/get_users.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(results)
}

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {     // remember it's `async fn`
    let _stmt = include_str!("../sql/add_user.sql");    // This is a one-liner to get file content !
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());    // return all columns, no need to hard code return column names
    // but you need to hard code column names to query
    let stmt = client.prepare(&_stmt).await.unwrap();   // Create 'prepared statement' which allows parameters

    client    // tokio_postgres::client::Client
        .query(
            &stmt,
            &[    // Nice, it's parameterised ! $1 $2 $3 $4
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}
