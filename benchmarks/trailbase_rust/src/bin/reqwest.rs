use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

use trailbase_benchmark_runner_rust::{
    print_latencies, Message, RecordId, Tokens, N, PASSWORD, ROOM, USER_ID,
};

pub const LIMIT: usize = 16;

async fn create_message(client: &Client, auth_token: &str, i: i64) -> RecordId {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    let msg = Message {
        _owner: USER_ID.to_string(),
        data: format!("a message {i}"),
        room: ROOM.to_string(),
    };

    let res = client
        .post(URL)
        .header("Authorization", format!("Bearer {auth_token}"))
        .json(&msg)
        .send()
        .await
        .unwrap();

    if res.status() != StatusCode::OK {
        panic!("{res:?}");
    }

    return res.json().await.unwrap();
}

async fn read_message(client: &Client, auth_token: &str, record_id: &str) -> bytes::Bytes {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    return client
        .get(&format!("{URL}/{record_id}"))
        .header("Authorization", &format!("Bearer {auth_token}"))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
}

async fn insert_benchmark(
    runtime: &tokio::runtime::Runtime,
    client: &'static Client,
    tokens: &Tokens,
) {
    lazy_static! {
        static ref throttler: Semaphore = Semaphore::new(LIMIT);
    }

    let start = Instant::now();
    for id in 0..N {
        let auth_token = tokens.auth_token.clone();

        let handle = throttler.acquire().await.unwrap();
        runtime.spawn(async move {
            create_message(&client, &auth_token, id).await;
            drop(handle);
        });
    }

    let _ = throttler.acquire_many(LIMIT as u32).await.unwrap();
    let elapsed = Instant::now() - start;

    println!("Inserted {N} rows in {elapsed:?}");
}

async fn read_benchmark(
    runtime: &tokio::runtime::Runtime,
    client: &'static Client,
    tokens: &Tokens,
) {
    const N: i64 = 10000;
    const M: usize = 1000000;

    let record_ids = Arc::new(SegQueue::<RecordId>::new());

    {
        lazy_static! {
            static ref throttler: Semaphore = Semaphore::new(LIMIT);
        }

        let queue = Arc::new(SegQueue::<i64>::new());
        for id in 0..N {
            queue.push(id);
        }

        let insert_latencies = Arc::new(SegQueue::<std::time::Duration>::new());

        let start = Instant::now();
        for id in 0..N {
            let auth_token = tokens.auth_token.clone();
            let latencies = insert_latencies.clone();
            let record_ids = record_ids.clone();

            let handle = throttler.acquire().await.unwrap();
            runtime.spawn(async move {
                let start = Instant::now();
                let record_id = create_message(&client, &auth_token, id).await;
                let elapsed = Instant::now() - start;

                drop(handle);
                record_ids.push(record_id);
                latencies.push(elapsed);
            });
        }

        let _ = throttler.acquire_many(LIMIT as u32).await.unwrap();

        println!(
            "Inserted {N} rows in {elapsed:?}",
            elapsed = Instant::now() - start
        );

        print_latencies(
            Arc::into_inner(insert_latencies)
                .unwrap()
                .into_iter()
                .collect(),
        );
    }

    {
        lazy_static! {
            static ref throttler: Semaphore = Semaphore::new(LIMIT);
        }

        let record_ids: Vec<_> = Arc::into_inner(record_ids).unwrap().into_iter().collect();

        let queue = Arc::new(SegQueue::<String>::new());
        for idx in 0..M {
            queue.push(record_ids[idx % record_ids.len()].ids[0].clone());
        }

        let read_latencies = Arc::new(SegQueue::<std::time::Duration>::new());

        let start = Instant::now();
        for idx in 0..M {
            let id = record_ids[idx % record_ids.len()].ids[0].clone();
            let auth_token = tokens.auth_token.clone();
            let latencies = read_latencies.clone();

            let handle = throttler.acquire().await.unwrap();
            runtime.spawn(async move {
                let start = Instant::now();
                let _ = read_message(&client, &auth_token, &id).await;
                let elapsed = Instant::now() - start;

                drop(handle);
                latencies.push(elapsed);
            });
        }

        let _ = throttler.acquire_many(LIMIT as u32).await.unwrap();

        println!(
            "Read {M} rows in {elapsed:?}",
            elapsed = Instant::now() - start
        );

        print_latencies(
            Arc::into_inner(read_latencies)
                .unwrap()
                .into_iter()
                .collect(),
        );
    }
}

fn main() -> Result<(), anyhow::Error> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    lazy_static! {
        static ref client: Client = Client::new();
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

    runtime.block_on(insert_benchmark(&runtime, &client, &tokens));
    runtime.block_on(read_benchmark(&runtime, &client, &tokens));

    return Ok(());
}
