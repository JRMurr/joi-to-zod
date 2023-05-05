import { toZod } from "../index.js";
import Joi from "joi";

const NumberWithValid = Joi.number().valid(3, 4);

console.log(toZod(NumberWithValid)); // prints z.union([z.literal(3), z.literal(4)])
