use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::time::Instant;
use trailbase_benchmark_runner_rust::{Message, Tokens, N, PASSWORD, ROOM, USER_ID};
use ureq::{Agent, AgentBuilder};

const TASKS: usize = 16;

fn create_message(agent: &Agent, auth_token: &str, i: i64) {
    const URL: &str = "http://localhost:4000/api/records/v1/message_api";

    let msg = Message {
        _owner: USER_ID.to_string(),
        data: format!("a message {i}"),
        room: ROOM.to_string(),
    };

    let _ = agent
        .post(URL)
        .set("Authorization", &format!("Bearer {auth_token}"))
        .send_json(ureq::json!(msg))
        .unwrap();
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

    let elapsed = Instant::now() - start;

    println!("Inserted {N} rows in {elapsed:?}");

    return Ok(());
}
