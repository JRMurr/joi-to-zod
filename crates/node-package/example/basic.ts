import { toZod } from "../index.js";
import Joi from "joi";
import { StringWithValid, NumberWithValid, StringWithInValid } from "./schemas";

console.log(
  `basic.ts:6~~~~~~~~~~~~~~~~~~~${JSON.stringify(
    Joi.disallow().describe(),
    null,
    4
  )}~~~~~~~~~~~~~~~~~~~`
);
// console.log(toZod(NumberWithValid));
