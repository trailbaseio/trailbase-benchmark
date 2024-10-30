import pLimit from "p-limit";
import { Client } from "trailbase";

// Hard-coded in the migrations.
const room = "AZH8mYTFd5OexZn4K10jCA==";
const userId = "AZH8mYedc1K7hrsTZgdHBA==";
const password = "secret";

const client = new Client("http://localhost:4000");

await client.login("user@localhost", password);
const api = client.records("message_api");

async function createMessage(i: number): Promise<void> {
  await api.create({
    _owner: userId,
    data: `a message ${i}`,
    room,
  });
}

// Quick sanity check;
await createMessage(-1);

const start = Date.now();

const N = 100000;
const concurrency = 64;
const limit = pLimit(concurrency);

let promises = new Array<Promise<void>>();
for (let i = 0; i < N; i++) {
  promises.push(limit(() => createMessage(i)));
}

await Promise.all(promises);

console.log(
  `Inserted ${N} messages, took ${(Date.now() - start) / 1000
  }s (limit=${concurrency})`,
);
