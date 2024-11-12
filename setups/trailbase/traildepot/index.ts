import { addRoute } from "trailbase:main";

type Headers = { [key: string]: string };
type Request = {
  uri: string;
  headers: Headers;
  body: string;
};
type Response = {
  headers?: Headers;
  status?: number;
  body?: string;
};

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
