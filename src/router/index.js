import { createRouter, createWebHashHistory } from "vue-router";
import TitleScreen from "../views/TitleScreen.vue";
import GameScreen from "../views/GameScreen.vue";

const routes = [
  { path: "/", component: TitleScreen },
  { path: "/game", component: GameScreen },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
  mode: "history",
});

export default router;
