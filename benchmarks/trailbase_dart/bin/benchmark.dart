import 'dart:isolate';

import 'package:pool/pool.dart';
import 'package:trailbase/trailbase.dart';
import 'package:dio/dio.dart';

// Hard-coded in the migrations.
const room = 'AZH8mYTFd5OexZn4K10jCA==';
const userId = 'AZH8mYedc1K7hrsTZgdHBA==';
const password = 'secret';

const concurrency = 64;

/// Simple insertion benchmark used for the article.
///
/// Inserts N messages with limited concurrency from the main isolate.
Future<void> insertBenchmark(Client client) async {
  const N = 100000;

  final api = client.records('message_api');

  Future<void> createMessage(int i) async => await api.create({
        '_owner': userId,
        'data': 'a message $i',
        'room': room,
      });

  // Quick sanity check;
  await createMessage(-1);

  final pool = Pool(concurrency);
  final futures = <Future<void>>[];
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

/// Simple insertion+read benchmark measuring latencies and used for the article.
//
/// First insert N messages, then read them M times.
Future<void> readBenchmark(Client client) async {
  const N = 10000;
  const M = 1000000;

  final api = client.records('message_api');

  // Setup
  final List<RecordId> messageIds = await () async {
    final latencies = <Duration>[];
    Future<RecordId> createMessage(int i) async {
      final watch = Stopwatch()..start();
      final id = await api.create({
        '_owner': userId,
        'data': 'a message $i',
        'room': room,
      });
      latencies.add((watch..stop()).elapsed);
      return id;
    }

    // Quick sanity check.
    await createMessage(-1);

    final futures = <Future<RecordId>>[];
    final pool = Pool(concurrency);
    final start = DateTime.now();

    for (int i = 0; i < N; i++) {
      futures.add(pool.withResource(() => createMessage(i)));
    }
    pool.close();

    final List<RecordId> messageIds = await Future.wait(futures);
    print(
        'Inserted $N messages, took ${(DateTime.now().difference(start))} (limit=$concurrency)');

    printLatencies(latencies);

    return messageIds;
  }();

  assert(messageIds.length == N);

  final latencies = <Duration>[];
  Future<Map<String, dynamic>> readMessage(RecordId id) async {
    final watch = Stopwatch()..start();
    final data = await api.read(id);
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

Future<void> benchmarkHigherConcurrencyImpl(int size, int offset) async {
  final client = Client('http://localhost:4000');
  await client.login('user@localhost', password);
  final api = client.records('message_api');

  Future<void> createMessage(int i) async => await api.create({
        '_owner': userId,
        'data': 'a message $i',
        'room': room,
      });

  final pool = Pool(32);
  final futures = <Future<void>>[];

  for (int i = offset; i < offset + size; i++) {
    futures.add(pool.withResource(() => createMessage(i)));
  }
  pool.close();

  await Future.wait(futures);
}

/// Same as insertion benchmark just with more isolates to experiment with saturation.
Future<void> benchmarkHigherConcurrency() async {
  const N = 100000;
  const isolates = 4;
  const M = N ~/ isolates;

  final start = DateTime.now();

  final futures = <Future<void>>[];
  for (int i = 0; i < isolates; i++) {
    futures.add(Isolate.run(() => benchmarkHigherConcurrencyImpl(M, i * M)));
  }

  await Future.wait(futures);

  print(
      'Parallel-insertion of $N messages, took ${(DateTime.now().difference(start))} (limit=4x 32)');
}

Future<void> fibonacciJsBenchmark() async {
  const N = 1000;

  final dio = Dio();

  Future<Response<String>> getResult(int i) async {
    final response = await dio.get<String>('http://localhost:4000/fibonacci');
    assert(response.data == '832040');
    return response;
  }

  // Quick sanity check;
  final response = await getResult(-1);
  if (int.parse(response.data!) != 832040) {
    throw Exception('Unexpected result: ${response.data}');
  }

  final pool = Pool(concurrency);
  final futures = <Future<void>>[];
  final start = DateTime.now();

  for (int i = 0; i < N; i++) {
    futures.add(pool.withResource(() => getResult(i)));
  }
  pool.close();

  await Future.wait(futures);

  print(
      'Called "/fibonacci" $N times, took ${(DateTime.now().difference(start))} (limit=$concurrency)');
}

Future<void> main(List<String> arguments) async {
  final client = Client('http://localhost:4000');

  await client.login('user@localhost', password);

  await insertBenchmark(client);
  // await readBenchmark(client);
  // await benchmarkHigherConcurrency();
  // await fibonacciJsBenchmark();
}
