using System.Text.Json.Serialization;
using System.Text.Json;

public class Message {
  public string _owner { get; }
  public string data { get; }
  public string room { get; }

  public Message(string _owner, string data, string room) {
    this._owner = _owner;
    this.data = data;
    this.room = room;
  }
}

// See https://aka.ms/new-console-template for more information
class Program {
  static readonly string room = "AZH8mYTFd5OexZn4K10jCA==";
  static readonly string userId = "AZH8mYedc1K7hrsTZgdHBA==";

  static readonly int N = 100000;
  static readonly int CONCURRENCY = 64;

  public static async Task InsertBenchmark(TrailBase.Client client) {
    var api = client.Records("message_api");

    var allTasks = new List<Task>();
    var throttler = new SemaphoreSlim(initialCount: CONCURRENCY);
    var started = DateTimeOffset.Now;
    for (int i = 0; i < N; ++i) {
      int x = i;

      // do an async wait until we can schedule again
      await throttler.WaitAsync();

      allTasks.Add(
          Task.Run(async () => {
            try {
              var message = new Message(
                    userId,
                    $"a message {x}",
                    room
              );

              await api.Create(message);
            }
            catch (Exception e) {
              Console.WriteLine(e);
            }
            finally {
              throttler.Release();
            }
          }));
    }

    await Task.WhenAll(allTasks);

    Console.WriteLine(
      $"Inserted {N} messages, took {DateTimeOffset.Now - started} (limit={CONCURRENCY})");
  }

  static void PrintLatencies(List<TimeSpan> latencies) {
    latencies.Sort((a, b) => a.Microseconds.CompareTo(b.Microseconds));

    int len = latencies.Count();
    var p50 = latencies[len / 2];
    var p75 = latencies[(int)Math.Floor(len * 0.75)];
    var p90 = latencies[(int)Math.Floor(len * 0.90)];
    var p95 = latencies[(int)Math.Floor(len * 0.95)];

    Console.WriteLine($@"Latencies:
      p50={p50.Microseconds}us
      p75={p75.Microseconds}us
      p90={p90.Microseconds}us
      p95={p95.Microseconds}us");
  }

  public static async Task ReadBenchmark(TrailBase.Client client) {
    var N = 10000;
    var M = 1000000;


    var api = client.Records("message_api");

    var mutex = new Mutex();
    var messageIds = new List<TrailBase.RecordId>();

    {
      // Setup: inserts.
      var allTasks = new List<Task>();
      var throttler = new SemaphoreSlim(initialCount: CONCURRENCY);

      var insertLatencies = new List<TimeSpan>();
      var started = DateTimeOffset.Now;
      for (int i = 0; i < N; ++i) {
        int x = i;

        // do an async wait until we can schedule again
        await throttler.WaitAsync();

        allTasks.Add(
            Task.Run(async () => {
              try {
                var started = DateTimeOffset.Now;
                var message = new Message(
                      userId,
                      $"a message {x}",
                      room
                );

                var recordId = await api.Create(message);
                var latency = DateTimeOffset.Now - started;

                mutex.WaitOne();
                messageIds.Add(recordId);
                insertLatencies.Add(latency);
                mutex.ReleaseMutex();
              }
              catch (Exception e) {
                Console.WriteLine(e);
              }
              finally {
                throttler.Release();
              }
            }));
      }

      await Task.WhenAll(allTasks);

      Console.WriteLine(
        $"Inserted {N} messages, took {DateTimeOffset.Now - started} (limit={CONCURRENCY})");

      PrintLatencies(insertLatencies);
    }

    if (messageIds.Count() != N) {
      throw new Exception($"Expected {N} messages, got: {messageIds.Count()}");
    }

    {
      // Reads
      var allTasks = new List<Task>();
      var throttler = new SemaphoreSlim(initialCount: CONCURRENCY);

      var readLatencies = new List<TimeSpan>();
      var started = DateTimeOffset.Now;
      for (int i = 0; i < M; ++i) {
        int x = i;

        // do an async wait until we can schedule again
        await throttler.WaitAsync();

        allTasks.Add(
            Task.Run(async () => {
              try {
                var started = DateTimeOffset.Now;

                var recordId = messageIds[i % N];
                await api.Read<Message>(recordId);
                var latency = DateTimeOffset.Now - started;

                mutex.WaitOne();
                readLatencies.Add(latency);
                mutex.ReleaseMutex();
              }
              catch (Exception e) {
                Console.WriteLine(e);
              }
              finally {
                throttler.Release();
              }
            }));
      }

      await Task.WhenAll(allTasks);

      Console.WriteLine(
        $"Read {M} messages, took {DateTimeOffset.Now - started} (limit={CONCURRENCY})");

      PrintLatencies(readLatencies);
    }

  }

  public static async Task Main() {
    var client = new TrailBase.Client("http://localhost:4000", null);
    await client.Login("user@localhost", "secret");

    // await InsertBenchmark(client);
    await ReadBenchmark(client);
  }
}
