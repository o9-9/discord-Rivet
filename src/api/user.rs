use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    //pub discriminator: String,
    pub global_name: Option<String>,
    //pub avatar : Option<String>,
    //pub bot: Option<bool>,
}
