use diesel::prelude::*;
use diesel::dsl::sql;
use diesel::sql_types::Text;

pub fn execute_legacy_query(tainted_sql: String) -> String {
    let database_url = ":memory:";
    
    if let Ok(mut conn) = SqliteConnection::establish(database_url) {
        //SINK
        let result = sql::<Text>(&tainted_sql).execute(&mut conn); 
        
        match result {
            Ok(rows) => format!("Legacy query executed, {} rows affected", rows),
            Err(e) => format!("Legacy query failed: {}", e),
        }
    } else {
        "Legacy database connection failed".to_string()
    }
} 