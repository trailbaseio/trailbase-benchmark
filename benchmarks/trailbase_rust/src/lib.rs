use serde::{Deserialize, Serialize};

pub const N: i64 = 100000;

// Hard-coded in the migrations.
// FIXME: When/if there's a rust TrailBase client.
pub const ROOM: &str = "AZH8mYTFd5OexZn4K10jCA==";
pub const USER_ID: &str = "AZH8mYedc1K7hrsTZgdHBA==";
pub const PASSWORD: &str = "secret";

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub _owner: String,
    pub data: String,
    pub room: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub auth_token: String,
    #[allow(unused)]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordId {
    pub id: String,
}
