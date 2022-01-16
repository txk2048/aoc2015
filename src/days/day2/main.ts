import Results from "../../Results";
import part1 from "./part1";
import part2 from "./part2";
import Present from "./Present";

function main(rawInput: Buffer): Results {
  const presents = rawInput
    .toString("utf-8")
    .trim()
    .split("\n")
    .map((line) => {
      const match = /^(\d+)x(\d+)x(\d+)$/.exec(line.trim());
      if (match == null) throw new Error("Invalid input");

      const length = Number.parseInt(match[1], 10);
      const width = Number.parseInt(match[2], 10);
      const height = Number.parseInt(match[3], 10);

      return new Present(length, width, height);
    });

  const result1 = part1(presents);
  const result2 = part2(presents);

  return new Results(result1.toString(), result2.toString());
}

export default main;
