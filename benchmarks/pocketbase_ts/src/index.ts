import PocketBase from 'pocketbase'
import { RecordModel } from 'pocketbase'
import pLimit from 'p-limit';

const pb = new PocketBase('http://127.0.0.1:8090');
pb.autoCancellation(false);

const userData = await pb.collection('users').authWithPassword('user@bar.com', '1234567890');
console.log('test', userData);

const api = pb.collection('message');
// const messages = await pb.collection('message').getList();
// console.log(messages);

const rooms = await pb.collection('room').getList();
const r = new Map(rooms.items.map(i => [i.name, i]));
const roomId = r.get('room0')!.id;
console.log(roomId);

const N = 100000;
const concurrency = 64;
const limit = pLimit(concurrency);

const createMessage = (i: number) =>
  api.create({
    owner: userData.record.id,
    room: roomId,
    data: `a message ${i}`,
  });

const start = Date.now();
let promises = new Array<Promise<RecordModel>>();
for (let i = 0; i < N; i++) {
  promises.push(limit(() => createMessage(i)));
}
await Promise.all(promises);

// Some results:
//
// Inserted 100000 messages, took 62.099s (limit=32)
// Inserted 100000 messages, took 61.33s (limit=64)
// Inserted 100000 messages, took 64.766s (limit=64)  (2024-11-06)
console.log(`Inserted ${N} messages, took ${(Date.now() - start) / 1000}s (limit=${concurrency})`)
