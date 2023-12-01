#[derive(Debug, PartialEq, Eq)]
pub struct User<'a> {
    id: i32,
    name: &'a str,
    api_key: &'a str,
}
