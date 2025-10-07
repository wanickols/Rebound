export class Team {
  constructor(
    public id: number,
    public name: string,
    public color: string, // Assuming your Rust Color serializes to something like "#RRGGBB"
    public score: number
  ) {}
}
