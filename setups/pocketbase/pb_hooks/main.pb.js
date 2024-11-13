/// <reference path="../pb_data/types.d.ts" />

// function fibonacci(num) {
//   switch (num) {
//     case 0:
//       return 0;
//     case 1:
//       return 1;
//     default:
//       return fibonacci(num - 1) + fibonacci(num - 2);
//   }
// }

routerAdd('GET', '/fibonacci', (c) => {
  const param = c.queryParam("n");
  const n = +(param.length > 0 ? param : 40);

  function fibonacci(num) {
    switch (num) {
      case 0:
        return 0;
      case 1:
        return 1;
      default:
        return fibonacci(num - 1) + fibonacci(num - 2);
    }
  }

	return c.string(200, fibonacci(n));
});
