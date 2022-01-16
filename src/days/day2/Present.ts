class Present {
  public readonly surfaceArea: number;
  public readonly smallestArea: number;
  public readonly smallestPerimeter: number;
  public readonly volume: number;

  constructor(length: number, width: number, height: number) {
    this.surfaceArea =
      2 * length * width + 2 * width * height + 2 * length * height;

    this.smallestArea = Math.min(
      length * width,
      width * height,
      length * height
    );

    this.smallestPerimeter = Math.min(
      2 * length + 2 * width,
      2 * width + 2 * height,
      2 * length + 2 * height
    );

    this.volume = length * width * height;
  }
}

export default Present;
