import { expect, test } from "bun:test";
import { trebuchet } from "./trebuchet";

test("it should return correct sum for example", () => {
  const input = `1abc2
  pqr3stu8vwx
  a1b2c3d4e5f
  treb7uchet`;

  expect(trebuchet(input)).toEqual(142);
});

test("it should handle lines with single digits", () => {
  const input = `1abc
  pqr3stuvwx
  a1b2c3d4e5f`;

  expect(trebuchet(input)).toEqual(59);
});

const testTable: Array<[string, number]> = [
  [`sixsrvldfour4seven\n53hvhgchljnlxqjsgrhxgf1zfoureightmlhvvv`, 95],
];

test.each(testTable)("%o", (input, sum) => {
  expect(trebuchet(input)).toEqual(sum);
});
