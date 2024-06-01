use anyhow::Result;
use tokio::sync::mpsc;
use axum::{extract::State, routing::{post, get}};
use tokio_rusqlite as sqlite;

const DEFAULT_PORT: u32 = 11257;

#[derive(Debug)]
enum IngestMessage {
    NewData(String),
}

#[derive(Clone)]
struct CharcticState {
    conn: sqlite::Connection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = sqlite::Connection::open("test.sqlite3").await?;

    // Where we're going we don't need migrations.
    conn.call(|conn| {
        conn.execute("
            CREATE TABLE IF NOT EXISTS ingest (
                raw TEXT,
                timestamp INT NOT NULL GENERATED ALWAYS AS
                    (json_extract(raw, '$.timestamp')) VIRTUAL
            );
        ", ())?;

        Ok(())
    }).await?;

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/ingest", post(ingest))
        .with_state(CharcticState { conn, });

    let listen = format!("0.0.0.0:{DEFAULT_PORT}");
    let listener = tokio::net::TcpListener::bind(&listen).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// return the json in each row as a json array string
// select json_array(json(raw)) from ingest where timestamp > (unixepoch('subsec) - 1800);
// all unique keys:
// SELECT DISTINCT j.key FROM ingest t, json_each(raw) j;
async fn index(
    State(state): State<CharcticState>,
) -> String {
    state.conn.call(|conn| {
        let mut stmt = conn.prepare("
            SELECT raw FROM ingest WHERE timestamp > (unixepoch('subsec') - 1800);
        ")?;
        let mut rows = stmt.query([])?;
        let mut keys: Vec<String> = Vec::new();

        while let Some(row) = rows.next()? {
            keys.push(row.get(0)?);
        }

        Ok(keys)
    }).await.unwrap()
}

async fn ingest(
    State(state): State<CharcticState>,
    body: String,
) {
    state.conn.call(|conn| {
        conn.execute("
            INSERT INTO ingest (raw) VALUES (json(?1));
        ", (body,))?;

        Ok(())
    }).await.unwrap();
}
