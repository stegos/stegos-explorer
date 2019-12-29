import Vue from "vue";
import VueRouter from "vue-router";
import Search from "../views/search.vue";
import About2 from "../views/About2.vue";
import Block from "../views/show_block.vue";
import MainPage from "../views/main_page.vue";

Vue.use(VueRouter);

const routes = [
  {
    path: "/:last_epoch?",
    name: "block_list",
    component: MainPage
  },
  {
    path: "/block/:epoch",
    name: "macro_block",
    component: Block
  },
  {
    path: "/block/:epoch/:offset",
    name: "micro_block",
    component: Block
  },
  {
    path: "/search/:id",
    name: "search",
    component: Search
  },
  {
    path: "/test/:id",
    name: "test graphql api object by id",
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
  routes
});

export default router;
