use lazy_static::lazy_static;
use reqwest::{Client, StatusCode};
use std::time::Instant;
use tokio::sync::Semaphore;

use trailbase_benchmark_runner_rust::{Message, Tokens, LIMIT, N, PASSWORD, ROOM, USER_ID};

pub const LIMIT: usize = 16;

async fn create_message(client: &Client, auth_token: &str, i: i64) {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    let msg = Message {
        _owner: USER_ID.to_string(),
        data: format!("a message {i}"),
        room: ROOM.to_string(),
    };

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

fn main() -> Result<(), anyhow::Error> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    lazy_static! {
        static ref client: Client = Client::new();
        static ref throttler: Semaphore = Semaphore::new(LIMIT);
    }

    // Log in.
    let tokens = runtime.block_on(async {
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

        // Quick sanity check.
        create_message(&client, &tokens.auth_token, -1).await;

        return Ok::<Tokens, anyhow::Error>(tokens);
    })?;

    runtime.block_on(async {
        let mut tasks = vec![];

        let start = Instant::now();
        for id in 0..N {
            let auth_token = tokens.auth_token.clone();

            let handle = throttler.acquire().await.unwrap();

            tasks.push(runtime.spawn(async move {
                create_message(&client, &auth_token, id).await;
                drop(handle);
            }));
        }

        let _ = throttler.acquire_many(LIMIT as u32).await.unwrap();
        let elapsed = Instant::now() - start;

        println!("Inserted {N} rows in {elapsed:?}");
    });

    return Ok(());
}
