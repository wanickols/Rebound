import { Kind, ActionState } from "@/Game/State";

function parseKind(value: string): Kind {
  if (!Object.values(Kind).includes(value as Kind)) {
    throw new Error(`Invalid Kind in meta.json: ${value}`);
  }
  return value as Kind;
}

function parseAnimationState(value: string): ActionState {
  if (!Object.values(ActionState).includes(value as ActionState)) {
    throw new Error(`Invalid AnimationState in meta.json: ${value}`);
  }
  return value as ActionState;
}
export { parseKind, parseAnimationState };
