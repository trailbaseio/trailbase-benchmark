/// Script for cutting up `top` output, extracting RSS+CPU, and writing it to a JSON file.
///
/// Top should be invoked as:
//    $ watch -n 0.5 "top -n 1 -b -p $(pgrep trail) | tail -n +8 | tee -a out.txt"
import fs from "fs/promises";

const Columns = {
  PID: 0,
  USER: 1,
  PRIORITY: 2,
  NICE: 3,
  VIRT: 4,
  RES: 5,
  SHR: 6,
  STATUS: 7,
  CPU: 8,
  MEM: 9,
  CPU_TIME: 10,  // Not wall time.
  COMMAND: 11,
};

type Output = {
  rss: number;
  cpu: number;
};

async function main(): Promise<void> {
  const inputFilename =
    process.argv.length >= 3 ? process.argv[2] : "output.txt";
  const outputFilename =
    process.argv.length >= 4 ? process.argv[3] : "output.json";

  const file = await fs.open(inputFilename);

  let numLines = 0;
  const entries = new Array<Output>();
  for await (const line of file.readLines()) {
    numLines++;
    const split = line.trim().split(/\s+/);

    // S = 'sleeping', R = 'running', I = 'idle', ...
    const processStatus = split[Columns.STATUS];
    console.assert(["S", "R", "I"].includes(processStatus), `Wrong column offset? ${line}`)

    entries.push({
      cpu: parseFloat(split[Columns.CPU]) / 100,
      rss: parseInt(split[Columns.RES]),
    });
  }

  const output = await fs.open(outputFilename, "w");
  await output.write(JSON.stringify(entries, null, 2));

  console.log(`Data points written '${entries.length}' from lines ${numLines}`);
}

await main();
