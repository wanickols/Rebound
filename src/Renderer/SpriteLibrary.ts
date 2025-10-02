const spriteLibrary: Record<string, HTMLImageElement> = {};

// preload images
function loadSprites() {
  const assets: Record<string, string> = {
    player: "/sprites/player.png",
    ball: "/sprites/ball.png",
    wall: "/sprites/wall.png",
  };

  for (const [kind, path] of Object.entries(assets)) {
    const img = new Image();
    img.src = path;
    spriteLibrary[kind] = img;
  }
}

loadSprites();
