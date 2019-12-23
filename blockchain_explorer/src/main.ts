import Vue from "vue";
import Vuex from "vuex";

import Buefy from "buefy";
import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faUserSecret,
  faArrowUp,
  faAngleLeft,
  faAngleRight,
  faSearch,
  faCopy
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import VueClipboard from "vue-clipboard2";

import App from "./App.vue";
import router from "./router";
import my_crypto from "./crypto";

import store from "./store";

library.add({
  faUserSecret,
  faArrowUp,
  faAngleLeft,
  faAngleRight,
  faSearch,
  faCopy
});

Vue.component("font-awesome-icon", FontAwesomeIcon);

Vue.use(VueClipboard);
Vue.config.productionTip = false;
Vue.use(Buefy, {
  defaultIconComponent: "font-awesome-icon",
  defaultIconPack: "fas"
});

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
