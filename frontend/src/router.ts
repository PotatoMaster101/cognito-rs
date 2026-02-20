import HomeView from "@/pages/HomeView.vue";
import SignInCallbackView from "@/pages/SignInCallbackView.vue";
import SignOutCallbackView from "@/pages/SignOutCallbackView.vue";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      component: HomeView,
      path: "/",
    },
    {
      component: SignInCallbackView,
      path: "/signin-callback",
    },
    {
      component: SignOutCallbackView,
      path: "/signout-callback",
    },
  ],
});

export default router;
