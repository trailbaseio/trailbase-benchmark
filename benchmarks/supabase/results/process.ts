import fs from 'fs/promises'

type Entry = {
  BlockIO: string;
  CPUPerc: string;
  Container: string;
  ID: string;
  MemPerc: string;
  MemUsage: string;
  Name: string;
  NetIO: string;
  PIDs: string;
};

type Output = {
  cpuPercent: number;
  memUsageKb: number;
}

/// Value of form '85.38MiB / 30.06GiB'
function parseToKb(value: string): number {
  const first = value.split(' / ')[0];

  if (first.endsWith('MiB')) {
    return parseFloat(value) * 1024;
  } else if (first.endsWith('GiB')) {
    return parseFloat(value) * 1024 * 1024;
  } else if (first === '--') {
    return 0;
  } else {
    throw Error(value);
  }
}

async function main(): Promise<void> {
  const file = await fs.open('docker.stats.json');

  const map = new Map<string, Entry[]>();

  for await (const line of file.readLines()) {
    try {
      // There are sometimes funny non-printable characters in front :shrug:
      const pos = line.indexOf("{");
      const trimmed = line.substring(pos)
      const entry = JSON.parse(trimmed) as Entry;

      const series = map.get(entry.Name) ?? [];
      series.push(entry);
      map.set(entry.Name, series);
    } catch (err) {
      console.log(err, line);
      continue;
    }
  }

  const outputs: Record<string, Output[]> = {};
  for (const key of map.keys()) {
    const pos = key.search('supabase');
    if (pos < 0) {
      continue;
    }

    const values = map.get(key)!;
    console.log(key, values.length);

    const output: Output[] = [];
    for (const v of values) {
      output.push({
        cpuPercent: parseFloat(v.CPUPerc) / 100.0,
        memUsageKb: parseToKb(v.MemUsage),
      });
    }

    outputs[key] = output;
  }

  const output = await fs.open('output.json', 'w');
  await output.write(JSON.stringify(outputs, null, 2));
}

await main();
