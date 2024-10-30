import pLimit from 'p-limit';

import { createClient, PostgrestSingleResponse } from '@supabase/supabase-js'
import { Database } from './types/supabase.js'

const SUPABASE_URL = 'http://localhost:8000';
const SUPABASE_ANON_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyAgCiAgICAicm9sZSI6ICJhbm9uIiwKICAgICJpc3MiOiAic3VwYWJhc2UtZGVtbyIsCiAgICAiaWF0IjogMTY0MTc2OTIwMCwKICAgICJleHAiOiAxNzk5NTM1NjAwCn0.dc_X5iR_VP_qT0zsiyj_I_OZ2T9FtRU2BBNWN8Bu4GE';

const supabase = createClient<Database>(SUPABASE_URL, SUPABASE_ANON_KEY);
const r = await supabase.auth.signInWithPassword({
  email: 'foo@bar.com',
  password: '123456789',
});

const roomId = 1;

console.log('login', r);

// // Quick sanity check;
const result = await supabase.from('message').insert({
  data: 'message',
  room: roomId,
});

if (result.status != 201) {
  console.error(result);
  process.exit()
}

const start = Date.now();

const N = 100000;
const concurrency = 64;
const limit = pLimit(concurrency);

let promises = new Array<Promise<PostgrestSingleResponse<null>>>();
for (let i = 0; i < N; i++) {
  promises.push(limit(() => supabase.from('message').insert({
    data: `a message ${i}`,
    room: roomId,
  })));
}

await Promise.all(promises);

// Some results:
//  Inserted 100000 messages, took 119.17s (limit=64)
console.log(`Inserted ${N} messages, took ${(Date.now() - start) / 1000}s (limit=${concurrency})`)
