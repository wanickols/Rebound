import { Kind, AnimationState } from "@/Game/State";

function parseKind(value: string): Kind {
  if (!Object.values(Kind).includes(value as Kind)) {
    throw new Error(`Invalid Kind in meta.json: ${value}`);
  }
  return value as Kind;
}

function parseAnimationState(value: string): AnimationState {
  if (!Object.values(AnimationState).includes(value as AnimationState)) {
    throw new Error(`Invalid AnimationState in meta.json: ${value}`);
  }
  return value as AnimationState;
}
export { parseKind, parseAnimationState };
