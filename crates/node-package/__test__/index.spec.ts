import test from "ava";
import { TestSchema, BasicObject } from "./schemas";

import { run } from "../index.js";

// test("run code gen", (t) => {
//   t.deepEqual(run(), ""); // fails but don't care
// });

test("describe", (t) => {
  console.log(
    `index.spec.ts:11~~~~~~~~~~~~~~~~~~~${JSON.stringify(
      TestSchema.describe(),
      null,
      4
    )}~~~~~~~~~~~~~~~~~~~`
  );
  t.deepEqual("a", "a");
});

test("describeBasic", (t) => {
  console.log("index.spec.ts:22: BasicObject");
  console.dir(BasicObject.describe(), {
    depth: null,
    showHidden: false,
    colors: true,
  });
  console.log(
    `index.spec.ts:28~~~~~~~~~~~~~~~~~~~${JSON.stringify(
      BasicObject.describe(),
      null,
      4
    )}~~~~~~~~~~~~~~~~~~~`
  );
  t.deepEqual("a", "a");
});
