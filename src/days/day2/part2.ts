import Present from "./Present";

function part2(presents: Present[]): number {
  return presents.reduce(
    (accumulator, current) =>
      accumulator + current.smallestPerimeter + current.volume,
    0
  );
}

export default part2;
