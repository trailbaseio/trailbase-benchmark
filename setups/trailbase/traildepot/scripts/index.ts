/// <reference path="../../../../../trailbase/trailbase-core/js/dist/index.d.ts" />
import { addRoute } from "trailbase:main";

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

addRoute("GET", "/fibonacci", (_req: Request): Response => {
  return {
    body: fibonacci(30).toString(),
  };
});
