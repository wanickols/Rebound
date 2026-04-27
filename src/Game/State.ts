export enum Kind {
  Player = "Player",
  Brick = "Brick",
  Wall = "Wall",
  Ball = "Ball",
  Goal = "Goal",
}

export enum ActionState {
  Idle = "Idle",
  Moving = "Moving",
  Dashing = "Dashing",
}

export type Shape =
  | { type: "circle"; radius: number }
  | { type: "rectangle"; w: number; h: number };

export class State {
  constructor(
    public id: number,
    public x: number,
    public y: number,
    public vx: number,
    public vy: number,
    public angle: number,
    public shape: Shape,

    public action_state: ActionState,
    public is_holding: boolean,
    public is_held: boolean,
    public is_static: boolean,

    public kind: Kind,
    public player_id: [number, number] | null = [42, 5],
    public team_id: number | null,
  ) {}
}
