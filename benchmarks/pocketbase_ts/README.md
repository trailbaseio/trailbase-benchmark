# PocketBase-TS Benchmark Driver

admin: admin@bar.com
password: 1234567890

user: user@bar.com
password: 1234567890

## Some Results

Inserted 100000 messages, took 67.721s (limit=64)
36.10user 3.75system 1:09.39elapsed 57%CPU (0avgtext+0avgdata 1407784maxresident)k
0inputs+40outputs (0major+392975minor)pagefaults 0swaps

## Server's /usr/bin/time Output

./run.sh
2024/10/07 11:13:18 Server started at http://127.0.0.1:8090
├─ REST API: http://127.0.0.1:8090/api/
└─ Admin UI: http://127.0.0.1:8090/_/
154.01user 24.94system 1:15.26elapsed 237%CPU (0avgtext+0avgdata 103452maxresident)k
515912inputs+5537792outputs (0major+62400minor)pagefaults 0swaps
