# Payload CMS Benchmark Setup

Built from the starter template, that's why there's a bunch of extra stuff. I
removed some but I but at the end of the day it shouldn't affect performance.
Weirdly, their "blank" template doesn't set up an admin dash :shrug:.

The setup registers collections/tables for:

- messages
- rooms
- room members

Ideally we'd automatically bootstrap the required data for freshly instantiated databases:

- Create a user.
- Inserting ("room0") into rooms
- And (user, rooms) values (1, 1) into room_members.

However, we haven't found a good way to initialize the data. Should probably be
a migration but its unclear how to mix auto-schema and data migrations.
Their own example template registers a "seedHandler" endpoint and has that be
called by the Page.tsx template with docstring instructions to remove it later.
Not my speed.

So to run the benchmark you'll have to log in to the admin dash to create the
first user and to subsequently add the room and the room membership.

WARN: By default the benchmark will perform worse due to Payload CMS not
creating SQLite databases in WAL mode.

We manually enabled WAL mode to give payload a better chance using:

```
sqlite> PRAGMA journal_mode       = WAL;
sqlite> PRAGMA journal_size_limit = 200000000;
sqlite> PRAGMA busy_timeout       = 10000;
```

This led to a 3+x performance improvement. We also filed
https://github.com/payloadcms/payload/issues/9290 with payload to let them
know.
