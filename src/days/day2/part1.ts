import Present from "./Present";

function part1(presents: Present[]): number {
  return presents.reduce(
    (accumulator, current) =>
      accumulator + current.surfaceArea + current.smallestArea,
    0
  );
}

export default part1;
