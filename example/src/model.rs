#[derive(Cherry)]
#[cherry(mysql, table = "my_user")]
pub struct User {
    pub id: u32,
    pub name: String,
}
