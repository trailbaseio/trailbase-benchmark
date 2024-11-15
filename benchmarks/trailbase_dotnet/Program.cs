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

  public static async Task Main() {
    var client = new TrailBase.Client("http://localhost:4000", null);
    await client.Login("user@localhost", "secret");

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

    // won't get here until all urls have been put into tasks
    await Task.WhenAll(allTasks);

    Console.WriteLine(
      $"Inserted {N} messages, took {DateTimeOffset.Now - started} (limit={CONCURRENCY})");
  }
}
