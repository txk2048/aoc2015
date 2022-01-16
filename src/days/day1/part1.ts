function part1(instructions: (1 | -1)[]): number {
  // 1 is up and -1 is down
  // resulting floor is the sum of these values
  return instructions.reduce((a, b) => a + b, 0);
}

export default part1;
