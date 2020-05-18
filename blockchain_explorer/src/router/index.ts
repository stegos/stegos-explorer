import Vue from "vue";
import VueRouter from "vue-router";
import Search from "../views/search.vue";
import About2 from "../views/About2.vue";
import Block from "../views/show_block.vue";
import MainPage from "../views/main_page.vue";
import ChooseNet from "../views/choose_net.vue";
import store from "@/store";

Vue.use(VueRouter);

const routes = [
  {
    path: ":last_epoch?",
    component: MainPage
  },
  {
    path: "block/:epoch",
    component: Block
  },
  {
    path: "block/:epoch/:offset",
    component: Block
  },
  {
    path: "search/:id",
    component: Search
  },
  {
    path: "test/:id",
    component: About2
  }
  // {
  //   path: "/about",
  //   name: "about",
  //   // route level code-splitting
  //   // this generates a separate chunk (about.[hash].js) for this route
  //   // which is lazy-loaded when the route is visited.
  //   component: () =>
  //     import(/* webpackChunkName: "about" */ "../views/About.vue")
  // }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes: [
    {
      path: "/mainnet",
      component: ChooseNet,
      children: routes
    },
    {
      path: "/testnet",
      component: ChooseNet,
      children: routes
    }
    // {
    //   path: "/",
    //   component: ChooseNet,
    //   children: routes
    // }
  ]
});
router.beforeEach((to, from, next) => {
  if (to.path.startsWith("/mainnet")) {
    store.commit("set_mainnet");
    next();
  }
  // else if (to.path.startsWith("/testnet")) {
  //   store.commit("set_testnet");
  //   next();
  // }
  else {
    next("/mainnet");
  }
});
export default router;
