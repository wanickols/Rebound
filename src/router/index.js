import { createRouter, createWebHashHistory } from "vue-router";
import TitleScreen from "../views/TitleScreen.vue";
import GameScreen from "../views/GameScreen.vue";
import JoinScreen from "../views/JoinScreen.vue";
import GameSetup from "../views/GameSetup.vue";

const routes = [
  { path: "/", component: TitleScreen },
  { path: "/game", component: GameScreen },
  { path: "/gameSetup", component: GameSetup },
  { path: "/join", component: JoinScreen },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
  mode: "history",
});

export default router;
