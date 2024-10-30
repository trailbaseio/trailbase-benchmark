import 'package:pocketbase/pocketbase.dart';
import 'package:pool/pool.dart';

const username = 'user@bar.com';
const password = '1234567890';
const concurrency = 64;

Future<void> insertBenchmark(PocketBase pb) async {
  const N = 100000;

  final userData =
      await pb.collection('users').authWithPassword(username, password);
  final userId = userData.record!.id;
  final api = pb.collection('message');

  final rooms = await pb.collection('room').getList();
  final roomNameToId = Map.fromEntries(rooms.items
      .map((record) => MapEntry(record.getStringValue('name'), record.id)));
  final roomId = roomNameToId['room0']!;

  Future<void> createMessage(int i) async {
    await api.create(body: {
      'owner': userId,
      'room': roomId,
      'data': 'a message $i',
    });
  }

  // Quick sanity check;
  await createMessage(-1);

  final futures = <Future<void>>[];
  final pool = Pool(concurrency);
  final start = DateTime.now();

  for (int i = 0; i < N; i++) {
    futures.add(pool.withResource(() => createMessage(i)));
  }
  pool.close();

  await Future.wait(futures);

  print(
      'Inserted $N messages, took ${(DateTime.now().difference(start))} (limit=$concurrency)');
}

void printLatencies(List<Duration> latencies) {
  latencies.sort();

  final p50 = latencies[latencies.length ~/ 2];
  final p75 = latencies[(latencies.length * 0.75).floor()];
  final p90 = latencies[(latencies.length * 0.9).floor()];
  final p95 = latencies[(latencies.length * 0.95).floor()];

  print('''Latencies:
      \tp50=${p50.inMicroseconds}us
      \tp75=${p75.inMicroseconds}us
      \tp90=${p90.inMicroseconds}us
      \tp95=${p95.inMicroseconds}us''');
}

// First insert N messages, then read them M times.
Future<void> readBenchmark(PocketBase pb) async {
  const N = 10000;
  const M = 100000;

  final userData =
      await pb.collection('users').authWithPassword(username, password);
  final userId = userData.record!.id;
  final api = pb.collection('message');

  final rooms = await pb.collection('room').getList();
  final roomNameToId = Map.fromEntries(rooms.items
      .map((record) => MapEntry(record.getStringValue('name'), record.id)));
  final roomId = roomNameToId['room0']!;

  // Setup
  final List<String> messageIds = await () async {
    final latencies = <Duration>[];
    Future<String> createMessage(int i) async {
      final watch = Stopwatch()..start();
      final response = await api.create(body: {
        'owner': userId,
        'room': roomId,
        'data': 'a message $i',
      });
      latencies.add((watch..stop()).elapsed);
      return response.id;
    }

    // Quick sanity check.
    await createMessage(-1);

    final futures = <Future<String>>[];
    final pool = Pool(concurrency);
    final start = DateTime.now();

    for (int i = 0; i < N; i++) {
      futures.add(pool.withResource(() => createMessage(i)));
    }
    pool.close();

    final List<String> messageIds = await Future.wait(futures);
    print(
        'Inserted $N messages, took ${(DateTime.now().difference(start))} (limit=$concurrency)');

    printLatencies(latencies);

    return messageIds;
  }();

  assert(messageIds.length == N);

  final latencies = <Duration>[];
  Future<RecordModel> readMessage(String id) async {
    final watch = Stopwatch()..start();
    final data = await api.getOne(id);
    latencies.add((watch..stop()).elapsed);
    return data;
  }

  // Quick sanity check.
  final data = await readMessage(messageIds[0]);
  print('read message sanity check: $data');

  final start = DateTime.now();

  final pool = Pool(concurrency);
  for (int i = 0; i < M; i++) {
    pool.withResource(() => readMessage(messageIds[i % messageIds.length]));
  }
  await (pool..close()).done;

  print(
      'Read $M messages, took ${(DateTime.now().difference(start))} (limit=$concurrency)');

  assert(latencies.length == M + 1);
  printLatencies(latencies);
}

Future<void> main(List<String> arguments) async {
  final pb = PocketBase('http://127.0.0.1:8090');

  // await insertBenchmark(pb);
  await readBenchmark(pb);
}
