import test from "ava";

import { toZod } from "../index.js";
import Joi from "joi";

test("run code gen", (t) => {
  t.deepEqual(toZod(Joi.number()), "z.number()");
});
