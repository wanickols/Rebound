export const spriteLibrary: Record<string, HTMLImageElement> = {};
import playerImg from "@/assets/sprites/player.png";
import ballImg from "@/assets/sprites/ball.png";
import wallImg from "@/assets/sprites/block.png";

export function loadSprites() {
  spriteLibrary["player"] = new Image();
  spriteLibrary["player"].src = playerImg;
  spriteLibrary["player"].onload = () => console.log("player loaded");

  spriteLibrary["ball"] = new Image();
  spriteLibrary["ball"].src = ballImg;
  spriteLibrary["ball"].onload = () => console.log("ball loaded");

  spriteLibrary["wall"] = new Image();
  spriteLibrary["wall"].src = wallImg;
  spriteLibrary["wall"].onload = () => console.log("wall loaded");
}

// optional: auto-load immediately
loadSprites();
