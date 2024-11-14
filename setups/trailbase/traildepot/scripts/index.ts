import { addRoute, parsePath, stringHandler } from "../trailbase.js";
import type { StringRequestType, ParsedPath } from "../trailbase.d.ts";

function fibonacci(num: number): number {
  switch (num) {
    case 0:
      return 0;
    case 1:
      return 1;
    default:
      return fibonacci(num - 1) + fibonacci(num - 2);
  }
}

addRoute("GET", "/fibonacci", stringHandler(async (req: StringRequestType) => {
  const uri : ParsedPath = parsePath(req.uri);
  const n = +(uri.query.get("n") ?? 40);
  return fibonacci(n).toString();
}));
