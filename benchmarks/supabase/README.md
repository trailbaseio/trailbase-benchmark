# Supabase 100k insert benchmark

## Setup

https://supabase.com/docs/guides/self-hosting/docker

```
# Get the code
git clone --depth 1 https://github.com/supabase/supabase

# Go to the docker folder
cd supabase/docker

# Copy the fake env vars
cp .env.example .env

# Pull the latest images
docker compose pull

# Start the services (in detached mode)
docker compose up -d
```

Admin das at http://localhost:8000

username: supabase
password: this_password_is_insecure_and_should_be_updated

NOTE: We left out the room-membership RLS check compared to the TrailBase and
PocketBase setups, which is in SupaBase's performance favor.
NOTE: We didn't find a quick and easy way to automatically bootstrap the table
schemas. It would certainly be possible falling back to the underlying postgres
and using e.g. `pg_restore`.
For the time being, tables have to be set up manually as follows:

* Table "message"

  Name        Description      Data Type                 Format
  id          No description   bigint                    int8
  created_at  No description   timestamp with time zone  timestamptz
  owner       No description   uuid                      uuid
  room        No description   bigint                    int8
  data        No description   text                      text

* Table "room"

  Name        Description      Data Type                 Format
  id          No description   bigint                    int8
  created_at  No description   timestamp with time zone  timestamptz
  name        No description   text                      text

* Table "room_members"

  Name        Description      Data Type                 Format
  id          No description   bigint                    int8
  created_at  No description   timestamp with time zone  timestamptz
  user        No description   uuid                      uuid
  room        No description   bigint                    int8

### Policy

```sql
alter policy "my_message_policy"
on "public"."message"
to authenticated
with check (
  true
);
```

## Version

Following above instructions benchmarks were run on a shallow checkout from
22-06-2024 ("fix: Updating EU tax ids (#27440)").

## Results

See `./results`.

## System Utilization

Utilization was measured using docker via:

    $ docker stats --format json > utilization

And then converted for graphing using `./results/process.ts`.


CONTAINER ID   NAME                             CPU %     MEM USAGE / LIMIT     MEM %     NET I/O           BLOCK I/O         PIDS                                                      [0/2707]
c3112ce1bdb6   supabase-storage                 0.58%     51.87MiB / 30.07GiB   0.17%     55.5kB / 75.6kB   4.6MB / 0B        11
4e1db06bb56c   supabase-studio                  0.00%     86.79MiB / 30.07GiB   0.28%     6.69MB / 7.01MB   0B / 0B           11
4521fe6b8be5   supabase-meta                    0.56%     89.07MiB / 30.07GiB   0.29%     5.29MB / 5.86MB   0B / 0B           11
f96ad99d4915   realtime-dev.supabase-realtime   0.64%     242.9MiB / 30.07GiB   0.79%     3.17MB / 3.68MB   0B / 1.09MB       57
1b2c422ebb9a   supabase-kong                    0.18%     1.329GiB / 30.07GiB   4.42%     8.61MB / 9.01MB   0B / 16.4kB       17
3a3d1188233e   supabase-edge-functions          0.00%     22.07MiB / 30.07GiB   0.07%     261kB / 11.7kB    8.31MB / 2.56MB   35
b719a2cc8208   supabase-rest                    0.11%     88.12MiB / 30.07GiB   0.29%     1.59MB / 952kB    0B / 0B           54
2c34bf264f8f   supabase-auth                    1.49%     14.03MiB / 30.07GiB   0.05%     73.5kB / 89.6kB   0B / 0B           18
a0d54cad0d41   supabase-analytics               4.61%     1.805GiB / 30.07GiB   6.00%     25.2MB / 12.2MB   0B / 1.08MB       56
d51a43a41035   supabase-db                      0.41%     176.1MiB / 30.07GiB   0.57%     18.7MB / 31.6MB   0B / 907MB        34
0a1a8cd00e75   supabase-vector                  1.66%     107MiB / 30.07GiB     0.35%     220kB / 1.2MB     0B / 0B           17
47b761a7d8cb   supabase-imgproxy                5.61%     23.21MiB / 30.07GiB   0.08%     15kB / 0B         0B / 0B           21

## During the insert

CONTAINER ID   NAME                             CPU %     MEM USAGE / LIMIT     MEM %     NET I/O           BLOCK I/O         PIDS                                                       [0/442]
c3112ce1bdb6   supabase-storage                 0.37%     51.43MiB / 30.07GiB   0.17%     56.6kB / 76.7kB   4.6MB / 0B        11
4e1db06bb56c   supabase-studio                  0.00%     86.86MiB / 30.07GiB   0.28%     6.69MB / 7.01MB   0B / 0B           11
4521fe6b8be5   supabase-meta                    0.34%     89.04MiB / 30.07GiB   0.29%     5.29MB / 5.86MB   0B / 0B           11
f96ad99d4915   realtime-dev.supabase-realtime   0.30%     242.7MiB / 30.07GiB   0.79%     3.26MB / 3.81MB   0B / 1.09MB       57
1b2c422ebb9a   supabase-kong                    30.94%    1.815GiB / 30.07GiB   6.04%     120MB / 137MB     0B / 16.4kB       17
3a3d1188233e   supabase-edge-functions          0.00%     22.61MiB / 30.07GiB   0.07%     261kB / 11.7kB    8.31MB / 2.56MB   35
b719a2cc8208   supabase-rest                    679.40%   107.2MiB / 30.07GiB   0.35%     307MB / 196MB     0B / 0B           98  <<<---
2c34bf264f8f   supabase-auth                    0.00%     14.04MiB / 30.07GiB   0.05%     80.5kB / 96.7kB   0B / 0B           18
a0d54cad0d41   supabase-analytics               71.26%    2.485GiB / 30.07GiB   8.26%     60.7MB / 40.9MB   0B / 1.08MB       56
d51a43a41035   supabase-db                      80.17%    245.2MiB / 30.07GiB   0.80%     224MB / 244MB     0B / 7.59GB       45
0a1a8cd00e75   supabase-vector                  17.18%    119.9MiB / 30.07GiB   0.39%     302kB / 30.5MB    0B / 0B           17
47b761a7d8cb   supabase-imgproxy                0.00%     24.97MiB / 30.07GiB   0.08%     15.1kB / 0B       6.59MB / 0B       21

## Old Result

Inserted 100000 messages, took 119.17s (limit=64)
