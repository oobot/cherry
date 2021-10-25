use cherry::MySqlTemplate;

pub struct Primary;
pub struct Secondary;

impl MySqlTemplate for Primary {
    // Datasource key, match db.toml file [mysql.xxx], xxx is the key value.
    fn key() -> &'static str {
        "datasource1"
    }
}

impl MySqlTemplate for Secondary {
    fn key() -> &'static str {
        "datasource2"
    }
}
