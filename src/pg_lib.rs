extern crate postgres;
use self::postgres::{Connection, TlsMode};

use std::env;

pub fn establish_connection() -> Connection {
    let env_user = env::var("PGUSER").expect("missing user variable");
    let env_password = env::var("PGPASSWORD").expect("missing pwd variable");
    let env_port = env::var("PGPORT").expect("missing user variable");
    let env_db = env::var("PGDATABASE").expect("missing user variable");

    let database_url = format!("postgres://{}:{}@localhost:{}/{}",
    	env_user, env_password, env_port, env_db);

    Connection::connect(database_url, TlsMode::None).unwrap()
}
