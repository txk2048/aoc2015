function part2(instructions: (1 | -1)[]): number {
  // 1 is up and -1 is down

  let floor = 0;
  for (let index = 0; index < instructions.length; ++index) {
    floor += instructions[index];
    if (floor == -1) return index + 1;
  }

  throw new Error("No result for part 2");
}

export default part2;
