use std::{env, process::Command};

use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::Connection;

fn get_postgres_connection() -> PgConnection {
    let connection_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("PGUSER").unwrap(),
        env::var("PGPASSWORD").unwrap(),
        env::var("PGHOST").unwrap(),
        env::var("PGPORT").unwrap(),
        env::var("PGDATABASE").unwrap()
    );

    PgConnection::establish(&connection_url).expect("Error connecting to the postgress database")
}

fn get_mysql_connection() {
    MysqlConnection::establish("mysql://mysql:mysql@127.0.0.1:3306")
        .expect("Error connecting to database");
}

fn main() {
    get_mysql_connection();
    get_postgres_connection();
    println!("Hello from rust")
}
