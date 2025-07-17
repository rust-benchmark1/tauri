use sqlx::{Connection, SqliteConnection};

pub fn execute_user_query(tainted_sql: String) -> String {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        if let Ok(mut conn) = SqliteConnection::connect("sqlite::memory:").await {
            //SINK
            let result = sqlx::query(&tainted_sql).execute(&mut conn).await; 
            
            match result {
                Ok(_) => "Query executed successfully".to_string(),
                Err(e) => format!("Query failed: {}", e),
            }
        } else {
            "Database connection failed".to_string()
        }
    })
} 