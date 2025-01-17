use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use trailbase_client::{Client, RecordApi};

use trailbase_benchmark_runner_rust::{print_latencies, Message, N, PASSWORD, ROOM, USER_ID};

pub const LIMIT: usize = 16;

#[inline]
async fn create_message(api: &RecordApi, i: i64) -> String {
    return api
        .create(Message {
            _owner: USER_ID.to_string(),
            data: format!("a message {i}"),
            room: ROOM.to_string(),
        })
        .await
        .unwrap();
}

#[inline]
async fn read_message(api: &RecordApi, record_id: &str) -> serde_json::Value {
    return api.read(record_id).await.unwrap();
}

async fn insert_benchmark(client: &Client) {
    lazy_static! {
        static ref throttler: Semaphore = Semaphore::new(LIMIT);
    }

    let api = client.records("message_api");

    let start = Instant::now();
    for id in 0..N {
        let api = api.clone();
        let handle = throttler.acquire().await.unwrap();

        tokio::spawn(async move {
            let _id = create_message(&api, id).await;
            drop(handle);
        });
    }

    let _ = throttler.acquire_many(LIMIT as u32).await.unwrap();
    let elapsed = Instant::now() - start;

    println!("Inserted {N} rows in {elapsed:?}");
}

async fn read_benchmark(client: &Client) {
    const N: i64 = 10000;
    const M: usize = 1000000;

    let api = client.records("message_api");

    let record_ids = Arc::new(SegQueue::<String>::new());

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
            let latencies = insert_latencies.clone();
            let record_ids = record_ids.clone();
            let api = api.clone();

            let handle = throttler.acquire().await.unwrap();
            tokio::spawn(async move {
                let start = Instant::now();
                let record_id = create_message(&api, id).await;
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
            queue.push(record_ids[idx % record_ids.len()].clone());
        }

        let read_latencies = Arc::new(SegQueue::<std::time::Duration>::new());

        let start = Instant::now();
        for idx in 0..M {
            let id = record_ids[idx % record_ids.len()].clone();
            let latencies = read_latencies.clone();
            let api = api.clone();

            let handle = throttler.acquire().await.unwrap();
            tokio::spawn(async move {
                let start = Instant::now();
                let _ = read_message(&api, &id).await;
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let site: &str = "http://localhost:4000";

    let client: Client = Client::new(site, None);
    client.login("user@localhost", PASSWORD).await.unwrap();

    insert_benchmark(&client).await;
    read_benchmark(&client).await;

    return Ok(());
}
