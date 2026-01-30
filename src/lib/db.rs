use rusqlite::{params, Connection, Result};

pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        
        CREATE TABLE IF NOT EXISTS nodes (
                  id UUID PRIMARY KEY,
                  node_name TEXT NOT NULL,
                    last_checkin TIMESTAMP,
                    first_checkin TIMESTAMP,
                    routable_address TEXT NOT NULL,
                    total_storage BIGINT,
                    used_storage BIGINT,
                    entry_count UINT,                  
                    public_key TEXT NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

pub fn register_node(conn: &Connection) -> Result<()> {
    conn.execute("INSERT INTO nodes (address, port, public_key) VALUES (?1, ?2, ?3)")?;
    Ok(())
}
