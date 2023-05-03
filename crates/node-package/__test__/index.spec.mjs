import test from "ava";

import { run } from "../index.js";

test("run code gen", (t) => {
  t.deepEqual(run(), ""); // fails but don't care
});
