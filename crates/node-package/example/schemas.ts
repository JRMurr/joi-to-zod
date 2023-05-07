import Joi from "joi";

// stolen from https://github.com/mrjono1/joi-to-typescript/blob/master/src/__tests__/alternatives/schemas/OneSchema.ts

export const thingSchema = Joi.object({
  thing: Joi.string().required(),
}).meta({ className: "Thing" });

export const otherSchema = Joi.object({
  other: Joi.string().optional(),
}).meta({ className: "Other" });

export const basicSchema = Joi.alternatives()
  .try(Joi.number(), Joi.string())
  .meta({ className: "Basic" })
  .description("a description for basic");

export const TestSchema = Joi.object({
  name: Joi.string().optional(),
  value: Joi.alternatives().try(thingSchema, otherSchema),
  basic: basicSchema,
})
  .meta({ className: "Test" })
  .description("a test schema definition");

export const TestListOfAltsSchema = Joi.array()
  .items(Joi.alt().try(Joi.bool(), Joi.string()))
  .required()
  .meta({ className: "TestList" })
  .label("aLabel")
  .description("A list of Test object");

export const AlternativesConditionalSchema = Joi.object({
  label: Joi.string(),
  someId: Joi.alternatives().conditional("label", {
    is: "abc",
    then: Joi.string().hex().required().length(24),
    otherwise: Joi.forbidden(),
  }),
}).meta({ className: "SomeSchema" });

export const BasicObject = Joi.object({
  // basic types
  name: Joi.string().optional().description("Test Schema Name"),
  propertyName1: Joi.boolean().required(),
  dateCreated: Joi.date(),
  count: Joi.number(),
  int: Joi.number().integer().optional(),
  obj: Joi.object(),
});

export const StringWithValid = Joi.string().valid("foo", "bar");

export const NumberWithValid = Joi.number().valid(3, 4);

export const StringWithInValid = Joi.string().invalid("bar", "quz");

export const ObjetWithWhen = Joi.object({
  condVal: Joi.string().valid("foo", "bar").default("foo"),
  username: Joi.string().when("condVal", {
    not: "bar",
    then: Joi.required(),
    otherwise: Joi.optional(),
  }),
  password: Joi.string().when("condVal", {
    is: "bar",
    then: Joi.required(),
    otherwise: Joi.forbidden(),
  }),
});
