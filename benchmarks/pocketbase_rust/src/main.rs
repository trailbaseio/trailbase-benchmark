use pocketbase_sdk::admin::Admin;
use pocketbase_sdk::client::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

const N: i64 = 10000;
const TASKS: i64 = 16;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct User {
    id: String,
    email: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Room {
    id: String,
    name: String,
}

#[derive(Clone, Serialize)]
struct Message {
    owner: String,
    data: String,
    room: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let admin_client =
        Admin::new("http://localhost:8090").auth_with_password("admin@bar.com", "1234567890")?;

    let email = "user@bar.com";
    let users = admin_client.records("users").list().call::<User>()?;
    let user = users.items.iter().find(|u| u.email == email).unwrap();

    let room_name = "room0";
    let rooms = admin_client.records("room").list().call::<Room>()?;
    let room = rooms.items.iter().find(|r| r.name == room_name).unwrap();

    let start = Instant::now();
    let tasks: Vec<_> = (0..TASKS)
        .into_iter()
        .map(|task| {
            let user_id = user.id.clone();
            let room_id = room.id.clone();

            tokio::spawn(async move {
                let client = Client::new("http://localhost:8090")
                    .auth_with_password("users", email, "1234567890")
                    .unwrap();

                for i in 0..N {
                    let id = task * N + i;

                    let msg = Message {
                        owner: user_id.clone(),
                        data: format!("a message {id}"),
                        room: room_id.clone(),
                    };
                    let _create_response = client.records("message").create(msg).call().unwrap();
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
