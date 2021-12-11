import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../views/Home.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
    meta: { title: "Home" },
  },
  {
    path: "/converter",
    name: "Converter",
    component: () => import("../views/Converter.vue"),
    meta: { title: "いろいろ変換" },
  },
  {
    path: "/bpm",
    name: "BPM",
    component: () => import("../views/BPM.vue"),
    meta: { title: "BPM計るやつ" },
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
