#[derive(Cherry)]
#[cherry(table = "my_user")]
pub struct User {
    pub id: i32,
    pub name: String,
}
