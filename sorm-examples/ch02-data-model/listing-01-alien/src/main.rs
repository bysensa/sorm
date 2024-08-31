use serde::{Deserialize, Serialize};
use sorm::*;

fn main() {
    println!("Hello, world!");
    let sc = User::schema();
    let tb = User::table();
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[sorm(table = "user")]
pub struct User {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
}
