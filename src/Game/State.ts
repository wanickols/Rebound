export enum Kind {
  Player = "Player",
  Brick = "Brick",
  Wall = "Wall",
  Ball = "Ball",
  Goal = "Goal",
}

export type Shape =
  | { type: "circle"; radius: number }
  | { type: "rectangle"; w: number; h: number };

export class State {
  constructor(
    public x: number,
    public y: number,
    public vx: number,
    public vy: number,
    public shape: Shape,
    public angle: number,
    public is_static: boolean,
    public kind: Kind,
    public player_id: number | null,
    public team_id: number | null
  ) {}
}
