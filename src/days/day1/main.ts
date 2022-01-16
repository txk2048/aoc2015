import Results from "../../Results";
import part1 from "./part1";
import part2 from "./part2";

function main(rawInput: Buffer): Results {
  const instructions = rawInput
    .toString("utf-8")
    .split("")
    .map((char) => {
      switch (char) {
        case "(":
          return 1; // up
        case ")":
          return -1; // down
        default:
          throw new Error("Invalid input");
      }
    });

  const result1 = part1(instructions);
  const result2 = part2(instructions);

  return new Results(result1.toString(), result2.toString());
}

export default main;
