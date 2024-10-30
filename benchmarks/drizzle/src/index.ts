import { drizzle } from "drizzle-orm/libsql";
import { sql } from "drizzle-orm";
import { text, sqliteTable } from "drizzle-orm/sqlite-core";
import { createClient } from "@libsql/client";
import pLimit from "p-limit";

const setup = [
  "PRAGMA busy_timeout       = 10000;",
  "PRAGMA journal_mode       = WAL",
  "PRAGMA journal_size_limit = 200000000",
  "PRAGMA synchronous        = NORMAL",
  "PRAGMA foreign_keys       = ON",
  "PRAGMA temp_store         = MEMORY",
  "PRAGMA cache_size         = -16000",
];

const createTables = `
  CREATE TABLE IF NOT EXISTS users (
    id           BLOB PRIMARY KEY DEFAULT (randomblob(16)) NOT NULL,
    email        TEXT DEFAULT '' NOT NULL
  ) strict;

  CREATE TABLE IF NOT EXISTS room (
    id           BLOB PRIMARY KEY DEFAULT (randomblob(16)) NOT NULL,
    name         TEXT DEFAULT '' NOT NULL
  ) strict;

  CREATE TABLE IF NOT EXISTS message (
    id           BLOB PRIMARY KEY DEFAULT (randomblob(16)) NOT NULL,
    owner        BLOB NOT NULL,
    data         TEXT DEFAULT '' NOT NULL,
    room         BLOB DEFAULT '' NOT NULL,

    FOREIGN KEY (owner) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (room) REFERENCES room(id) ON UPDATE CASCADE ON DELETE CASCADE
  ) strict;

  CREATE TABLE IF NOT EXISTS room_members (
    user         BLOB NOT NULL,
    room         BLOB NOT NULL,

    FOREIGN KEY (room) REFERENCES room(id) ON DELETE CASCADE,
    FOREIGN KEY (user) REFERENCES users(id) ON DELETE CASCADE
  ) strict;
`;

const users = sqliteTable("users", {
  id: text("id")
    .notNull()
    .default(sql`(randomblob(16))`),
  email: text("email").notNull(),
});

const rooms = sqliteTable("room", {
  id: text("id")
    .notNull()
    .default(sql`(randomblob(16))`),
  name: text("name").notNull(),
});

const messages = sqliteTable("message", {
  id: text("id")
    .notNull()
    .default(sql`(randomblob(16))`),
  owner: text("owner")
    .notNull()
    .references(() => users.id),
  room: text("room")
    .notNull()
    .references(() => rooms.id),
  data: text("data").notNull(),
});

const N = 100000;
const concurrency = 64;

async function main() {
  const client = createClient({ url: "file:test.db" });

  for (const pragma of setup) {
    await client.execute(pragma);
  }
  await client.executeMultiple(createTables);

  const db = drizzle(client);

  const user = await db
    .insert(users)
    .values({ email: "user@bar.com" })
    .returning();
  const userId = user[0].id;

  const room = await db.insert(rooms).values({ name: "room0" }).returning();
  const roomId = room[0].id;

  const createMessage = (i: number) =>
    db.insert(messages).values({
      owner: userId,
      room: roomId,
      data: `a message ${i}`,
    });

  const limit = pLimit(concurrency);

  const start = Date.now();
  let promises = new Array<Promise<unknown>>();
  for (let i = 0; i < N; i++) {
    promises.push(limit(() => createMessage(i)));
  }

  await Promise.all(promises);

  console.log(
    `Inserted ${N} messages, took ${(Date.now() - start) / 1000}s (limit=${concurrency})`,
  );

  const M = await db.select({ count: sql<number>`count(*)` }).from(messages);
  console.log(`Cross-check ${M[0].count} messages in DB`);
}

main();
