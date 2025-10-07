export const spriteLibrary: Record<string, HTMLImageElement> = {};
import playerImg from "@/assets/sprites/player.png";
import ballImg from "@/assets/sprites/ball.png";
import wallImg from "@/assets/sprites/block.png";

export function loadSprites() {
  spriteLibrary["Player"] = new Image();
  spriteLibrary["Player"].src = playerImg;
  spriteLibrary["Player"].onload = () => console.log("player loaded");

  spriteLibrary["Ball"] = new Image();
  spriteLibrary["Ball"].src = ballImg;
  spriteLibrary["Ball"].onload = () => console.log("ball loaded");

  spriteLibrary["Wall"] = new Image();
  spriteLibrary["Wall"].src = wallImg;
  spriteLibrary["Wall"].onload = () => console.log("wall loaded");
}

// optional: auto-load immediately
loadSprites();
