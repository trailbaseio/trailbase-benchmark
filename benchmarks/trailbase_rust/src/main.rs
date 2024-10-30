use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Instant;

const N: i64 = 100000;
const TASKS: i64 = 10;
const N_PER_TASK: i64 = N / TASKS;

// Hard-coded in the migrations.
const ROOM: &str = "AZH8mYTFd5OexZn4K10jCA==";
const USER_ID: &str = "AZH8mYedc1K7hrsTZgdHBA==";
const PASSWORD: &str = "secret";

#[derive(Debug, Serialize)]
struct Message {
    _owner: String,
    data: String,
    room: String,
}

#[derive(Debug, Deserialize)]
struct Tokens {
    auth_token: String,
    #[allow(unused)]
    refresh_token: String,
}

async fn create_message(client: &Client, auth_token: &str, i: i64) {
    let msg = Message {
        _owner: USER_ID.to_string(),
        data: format!("a message {i}"),
        room: ROOM.to_string(),
    };

    const URL: &str = "http://localhost:4000/api/records/v1/message_api";
    let res = client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {auth_token}"))
        .body(serde_json::to_string(&msg).unwrap())
        .send()
        .await
        .unwrap();

    if res.status() != StatusCode::OK {
        panic!("{res:?}");
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Client::new();

    let res = client
        .post("http://localhost:4000/api/auth/v1/login")
        .header("Content-Type", "application/json")
        .body(
            serde_json::json!({
                "email": "user@localhost",
                "password": PASSWORD,
            })
            .to_string(),
        )
        .send()
        .await?;

    let tokens: Tokens = serde_json::from_str(&res.text().await?)?;
    let auth_token = tokens.auth_token;

    // Quick sanity check.
    create_message(&client, &auth_token, -1).await;

    let start = Instant::now();
    let tasks: Vec<_> = (0..TASKS)
        .into_iter()
        .map(|task| {
            let auth_token = auth_token.clone();
            tokio::spawn(async move {
                let client = Client::new();

                for i in 0..N_PER_TASK {
                    let id = task * N_PER_TASK + i;
                    create_message(&client, &auth_token, id).await;
                }
                println!("finished {task}");
            })
        })
        .collect();

    for t in tasks {
        t.await.unwrap();
    }

    println!(
        "Inserted {count} rows in {elapsed:?}",
        elapsed = Instant::now() - start,
        count = N * TASKS
    );

    return Ok(());
}
