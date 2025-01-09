use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::time::Instant;
use ureq::{Agent, AgentBuilder};

use trailbase_benchmark_runner_rust::{
    print_latencies, Message, RecordId, Tokens, N, PASSWORD, ROOM, USER_ID,
};

const TASKS: usize = 10;

fn create_message(agent: &Agent, auth_token: &str, id: i64) -> RecordId {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    let msg = Message {
        _owner: USER_ID.to_string(),
        data: format!("a message {id}"),
        room: ROOM.to_string(),
    };

    let json: RecordId = agent
        .post(URL)
        .set("Authorization", &format!("Bearer {auth_token}"))
        .send_json(ureq::json!(msg))
        .unwrap()
        .into_json()
        .unwrap();

    return json;
}

fn read_message(agent: &Agent, auth_token: &str, record_id: &str) -> String {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    return agent
        .get(&format!("{URL}/{record_id}"))
        .set("Authorization", &format!("Bearer {auth_token}"))
        .call()
        .unwrap()
        .into_string()
        .unwrap();
}

#[allow(unused)]
fn insert_benchmark(agent: &Agent, tokens: &Tokens) -> Result<(), anyhow::Error> {
    let queue = Arc::new(SegQueue::<i64>::new());
    for id in 0..N {
        queue.push(id);
    }

    let start = Instant::now();
    let tasks: Vec<_> = (0..TASKS)
        .into_iter()
        .map(|_| {
            let queue = queue.clone();
            let auth_token = tokens.auth_token.clone();
            let agent = agent.clone();

            return std::thread::spawn(move || {
                while let Some(id) = queue.pop() {
                    create_message(&agent, &auth_token, id);
                }
            });
        })
        .collect();

    for task in tasks {
        task.join().unwrap();
    }

    println!(
        "Inserted {N} rows in {elapsed:?}",
        elapsed = Instant::now() - start
    );

    return Ok(());
}

#[allow(unused)]
fn read_benchmark(agent: &Agent, tokens: &Tokens) -> Result<(), anyhow::Error> {
    const N: i64 = 10000;
    const M: usize = 1000000;

    let record_ids = Arc::new(SegQueue::<RecordId>::new());

    {
        let queue = Arc::new(SegQueue::<i64>::new());
        for id in 0..N {
            queue.push(id);
        }

        let insert_latencies = Arc::new(SegQueue::<std::time::Duration>::new());

        let start = Instant::now();
        let tasks: Vec<_> = (0..TASKS)
            .into_iter()
            .map(|_| {
                let agent = agent.clone();
                let auth_token = tokens.auth_token.clone();
                let queue = queue.clone();
                let latencies = insert_latencies.clone();
                let record_ids = record_ids.clone();

                return std::thread::spawn(move || {
                    while let Some(id) = queue.pop() {
                        let start = Instant::now();
                        let record_id = create_message(&agent, &auth_token, id);
                        let elapsed = Instant::now() - start;

                        latencies.push(elapsed);
                        record_ids.push(record_id);
                    }
                });
            })
            .collect();

        for task in tasks {
            task.join().unwrap();
        }

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
        let record_ids: Vec<_> = Arc::into_inner(record_ids).unwrap().into_iter().collect();

        let queue = Arc::new(SegQueue::<String>::new());
        for idx in 0..M {
            queue.push(record_ids[idx % record_ids.len()].id.clone());
        }

        let read_latencies = Arc::new(SegQueue::<std::time::Duration>::new());

        let start = Instant::now();
        let tasks: Vec<_> = (0..TASKS)
            .into_iter()
            .map(|_| {
                let agent = agent.clone();
                let auth_token = tokens.auth_token.clone();
                let queue = queue.clone();
                let latencies = read_latencies.clone();

                return std::thread::spawn(move || {
                    while let Some(record_id) = queue.pop() {
                        let start = Instant::now();
                        read_message(&agent, &auth_token, &record_id);
                        let elapsed = Instant::now() - start;

                        latencies.push(elapsed);
                    }
                });
            })
            .collect();

        for task in tasks {
            task.join().unwrap();
        }

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

    return Ok(());
}

fn main() -> Result<(), anyhow::Error> {
    let agent = AgentBuilder::new().build();

    let tokens: Tokens = agent
        .post("http://localhost:4000/api/auth/v1/login")
        .send_json(ureq::json!({
            "email": "user@localhost",
            "password": PASSWORD,
        }))?
        .into_json()?;

    // Quick sanity check.
    create_message(&agent, &tokens.auth_token, -1);

    insert_benchmark(&agent, &tokens)?;
    read_benchmark(&agent, &tokens)?;

    return Ok(());
}
