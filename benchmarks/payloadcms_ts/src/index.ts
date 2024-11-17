import pLimit from "p-limit";

const site = 'http://localhost:3000';
const userId = 1;
const roomId = 1;

async function getToken(): Promise<string> {
  const res = await fetch(`${site}/api/users/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      email: 'admin@trailbase.io',
      password: 'secret',
    }),
  })

  const json = await res.json()
  return json.token;
}

const token = await getToken();

async function createMessage(i: number): Promise<void> {
  await fetch('http://localhost:3000/api/messages', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      _owner: userId,
      data: `a message ${i}`,
      room: roomId,
    }),
  });
}

// Sanity check
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
