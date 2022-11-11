use rbs::{from_value, to_value, Value};
use serde::{Deserialize, Serialize};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn from(&self) -> String {
        self.to_string()
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("{}<{}>", &self.username, &self.email)
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    #[graphql(skip)]
    pub id: i32,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}

rbatis::crud!(User {}, "users");
rbatis::crud!(NewUser {}, "users");
