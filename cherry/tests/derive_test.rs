use cherry::Cherry;
use cherry_derive::Cherry;

#[derive(Cherry)]
struct Example {
    id: u32,
}


#[test]
fn test() {
    let table = Example::table();
    assert_eq!("example", table);
    println!("{:?}", Example::columns())
}