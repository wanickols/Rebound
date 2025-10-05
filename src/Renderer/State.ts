class State {
  constructor(
    public x: number,
    public y: number,
    public vx: number,
    public vy: number,
    public w: number,
    public h: number,
    public mass: number,
    public is_static: boolean,
    public friction: number,
    public restitution: number,
    public kind: string,
    public player_id: number | null,
    public input: any // or a proper InputState type
  ) {}
}
