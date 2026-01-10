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

export function isInputFrameEqual(a: InputFrame, b: InputFrame): boolean {
  // Compare move_axis
  if (!a.move_axis || !b.move_axis) return false;
  if (a.move_axis.x !== b.move_axis.x || a.move_axis.y !== b.move_axis.y)
    return false;

  // Compare look
  if (!a.look || !b.look) return false;
  if (a.look.x !== b.look.x || a.look.y !== b.look.y) return false;

  // Compare buttons
  if (!a.buttons || !b.buttons) return false;
  for (const key of Object.keys(a.buttons) as (keyof typeof a.buttons)[]) {
    if (a.buttons[key] !== b.buttons[key]) return false;
  }

  return true;
}
