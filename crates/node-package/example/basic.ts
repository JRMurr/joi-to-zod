import { toZod } from "../index.js";
import Joi from "joi";
import { ObjetWithWhen } from "./schemas";

// const NumberWithValid = Joi.number().valid(3, 4);

// console.log(toZod(NumberWithValid)); // prints z.union([z.literal(3), z.literal(4)])

const IntSchema = Joi.number().integer().min(10).max(200).multiple(4);
console.log(
  `basic.ts:11~~~~~~~~~~~~~~~~~~~${JSON.stringify(
    IntSchema.describe(),
    null,
    4
  )}~~~~~~~~~~~~~~~~~~~`
);
console.log(toZod(IntSchema));

// const StringMin = Joi.string().min(1).default("aStr");
// console.log(
//   `basic.ts:11~~~~~~~~~~~~~~~~~~~${JSON.stringify(
//     StringMin.describe(),
//     null,
//     4
//   )}~~~~~~~~~~~~~~~~~~~`
// );
// console.log(toZod(StringMin));

// console.log(
//   `basic.ts:25~~~~~~~~~~~~~~~~~~~${JSON.stringify(
//     ObjetWithWhen.describe(),
//     null,
//     4
//   )}~~~~~~~~~~~~~~~~~~~`
// );
// console.log(toZod(ObjetWithWhen));
