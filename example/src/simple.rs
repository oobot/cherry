use cherry::MySqlTemplate;

#[derive(Cherry)]
#[cherry(mysql, table = "my_user")]
struct User {
    id: u32,
    name: String,
}

struct DataSource;

impl MySqlTemplate for DataSource {
    fn key() -> &'static str {
        "key"
    }
}

async fn _a() {
    let sim = User { id: 1, name: "tom".to_owned() };
    let _ = DataSource.insert(&sim).await;
}

#[cfg(test)]
mod test {
    use cherry::WrapArguments;

    #[test]
    fn test() {

    }
}