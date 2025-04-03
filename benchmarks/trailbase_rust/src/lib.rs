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
    pub ids: Vec<String>,
}

pub fn print_latencies(mut latencies: Vec<std::time::Duration>) {
    latencies.sort();

    let len = latencies.len();
    let p50 = latencies[len / 2];
    let p75 = latencies[(len as f64 * 0.75).floor() as usize];
    let p90 = latencies[(len as f64 * 0.90).floor() as usize];
    let p95 = latencies[(len as f64 * 0.95).floor() as usize];

    println!(
        "Latencies: \n\tp50={:?} \n\tp75={:?} \n\tp90={:?} \n\tp95={:?}",
        p50, p75, p90, p95
    );
}

