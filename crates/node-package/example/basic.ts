import { toZod } from "../index.js";
import Joi from "joi";
import { ObjetWithWhen } from "./schemas";

// const NumberWithValid = Joi.number().valid(3, 4);

// console.log(toZod(NumberWithValid)); // prints z.union([z.literal(3), z.literal(4)])

// const IntSchema = Joi.number().integer().multiple(4);
// const StringMin = Joi.string().min(1);

// // console.log(toZod(IntSchema));

// const ArrSchema = Joi.array().items(Joi.string());

// console.log("basic.ts:15: ArrSchema");
// console.dir(ArrSchema.validate([undefined]), {
//   depth: null,
//   showHidden: false,
//   colors: true,
// });

// console.log(toZod(ArrSchema));

console.log(
  `basic.ts:25~~~~~~~~~~~~~~~~~~~${JSON.stringify(
    ObjetWithWhen.describe(),
    null,
    4
  )}~~~~~~~~~~~~~~~~~~~`
);
console.log(toZod(ObjetWithWhen));
