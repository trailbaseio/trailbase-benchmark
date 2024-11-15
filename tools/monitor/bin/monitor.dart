import 'dart:async';
import 'dart:convert';
import 'dart:io';

class Columns {
  static const int PID = 0;
  static const int USER = 1;
  static const int PRIORITY = 2;
  static const int NICE = 3;
  static const int VIRT = 4;
  static const int RES = 5;
  static const int SHR = 6;
  static const int STATUS = 7;
  static const int CPU = 8;
  static const int MEM = 9;
  static const int CPU_TIME = 10; // Not wall time.
  static const int COMMAND = 11;
}

class Output {
  final int rss;
  final double cpu;
  final Duration elapsed;

  const Output({required this.rss, required this.cpu, required this.elapsed});

  Map<String, dynamic> toJson() => {
        'cpu': cpu,
        'rss': rss,
        'elapsed': elapsed.inMilliseconds,
      };
}

Future<Output?> runCmd(int pid, DateTime started) async {
  // NOTE: we're using `top` since `ps` et all provide aggregate stats.
  // NOTE: this is brittle since we rely on an implicit `top` output format,
  // which isn't stable and can be changed via .toprc. We didn't find a way
  // to specify output format on the command line. Supposedly you can use the
  // COLUMNS environment variable.
  final p =
      await Process.run('top', ['-n 1', '-b', '-p $pid'], runInShell: false);
  final now = DateTime.now();

  if (p.exitCode == 0) {
    final stdout = p.stdout as String;
    final line = stdout.split('\n').lastWhere((line) => line.isNotEmpty);

    final pattern = RegExp(r'\s+');
    final split = line.trim().split(pattern);

    // S = 'sleeping', R = 'running', I = 'idle', ...
    assert(['R', 'S', 'I'].contains(split[Columns.STATUS]));

    try {
      return Output(
        rss: int.parse(split[Columns.RES]),
        cpu: double.parse(split[Columns.CPU]) / 100,
        elapsed: now.difference(started),
      );
    } catch (e) {
      stderr.writeln('Failed to parse output: $e. Process $pid alive?');
    }
  }

  return null;
}

/// A wrapper around `top` to periodically fetch process stats, parse them,
/// add an explicit time axis, and ultimately write them to JSON.
Future<void> main(List<String> arguments) async {
  final pid = int.parse(arguments[1]);
  final started = DateTime.now();

  final outputs = <Output>[];
  final timer = Timer.periodic(Duration(milliseconds: 500), (timer) {
    runCmd(pid, started).then((output) {
      if (output != null) {
        outputs.add(output);
        print(output.toJson());
      }
    });
  });

  ProcessSignal.sigint.watch().listen((signal) async {
    print('caught $signal');

    timer.cancel();
    final json = jsonEncode(outputs);
    await File('output.json').writeAsString(json);

    exit(0);
  });
}
