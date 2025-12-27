export const FACE = {
  BOTTOM: 0,
  RIGHT: 1,
  LEFT: 2,
  TOP: 3,
} as const;

export interface Vec2 {
  x: number;
  y: number;
}

export interface InputFrame {
  // Movement intent (left stick, WASD, etc)
  move_axis: Vec2;

  // Look / aim intent (right stick, mouse delta normalized, etc)
  look: Vec2;

  // Buttons are stateful, not events
  buttons: {
    grab: boolean;
    dash: boolean;
    place: boolean;
  };

  // Optional but powerful:
  // increments every poll so backend can drop old frames
  //frame: number;
}
