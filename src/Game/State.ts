export enum Kind {
  Player = "Player",
  Brick = "Brick",
  Wall = "Wall",
  Ball = "Ball",
  Goal = "Goal",
}

export class State {
  constructor(
    public x: number,
    public y: number,
    public vx: number,
    public vy: number,
    public w: number,
    public h: number,
    public is_static: boolean,
    public kind: Kind,
    public player_id: number | null,
    public team_id: number | null
  ) {}
}
