export enum Kind {
  Player = "Player",
  Brick = "Brick",
  Wall = "Wall",
  Ball = "Ball",
  Goal = "Goal",
}

export enum AnimationState {
  Idle = "Idle",
  Moving = "Moving",
  Dashing = "Dashing",
  Shooting = "Shooting",
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
    public is_holding: boolean,
    public animation_state: AnimationState,
    public angle: number,
    public is_static: boolean,
    public kind: Kind,
    public player_id: [number, number] | null = [42, 5],
    public team_id: number | null,
  ) {}
}
