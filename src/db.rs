use crate::models::{QueryResult, Value};
use base64::{engine::general_purpose, Engine};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::sync::Arc;

const INIT_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS tracks {
        track_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        album TEXT NOT NULL,
        artist TEXT NOT NULL,
        duration INTEGER,
        price DECIMAL(10,2)

};

CREATE INDEX IF NOT EXISTS idx_tracks_name ON tracks(name);
CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);

INSERT ON IGNORE tracks (track_id, name, album, artist, duration, price) VALUES
    (1, 'The Tropper', 'Piece of Mind', 'Iron Maiden', 253, 0.99);

"#;

pub struct DbConnection {
    pub pool: &rc<Pool<SqliteConnectionManager>>
}

impl DbConnection {
    pub fn new() -> Result<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).unwrap();

        // Initialize schema
        let conn = pool.get().unwrap();
        conn.execute_batch(INIT_SQL)?;

        Ok(DbConnection {
            pool: Arc::new(pool),
        })
        
    }

    pub fn execute_query(&self, query:&str) -> Result<QueryResult, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(query)?;
        let cols = stmt.column_count();
        let column_names: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();

        let mut rows = Vec::new();
        let mut stmt_iter = stmt.query([])?;

        while let Some(row) = stmt_iter.next()? {
            let row_values = (0.. cols)
                 .map(|i| {
                    row.get_ref(i).map(|value_ref| match value_ref {
                        rusqlite::types::ValueRef::Null => Value::Null,
                        rusqlite::types::ValueRef::Integer(i) => Value::Int(i),
                        rusqlite::types::ValueRef::Real(r) => Value::Real(r),
                        rusqlite::types::ValueRef::Text(t) => {
                            Value::String(String::from_utf8_lossy(t).to_string())
                        }
                        rusqlite::types::ValueRef::Blob(b) => {
                            Value::String(general_purpose::STANDARD.encode(b))
                        }
                    })
                 })
                 .collect::<Result<Vec<_>>>()?;
                rows.push(row_values);
        }

        Ok(QueryResult {
            columns: column_names,
            rows,
        })
    }
}