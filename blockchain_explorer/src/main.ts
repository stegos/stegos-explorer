import Vue from "vue";
import Vuex from "vuex";

import Buefy from "buefy";

import VueClipboard from "vue-clipboard2";

import App from "./App.vue";
import router from "./router";

import store from "./store";

// Vue.component("font-awesome-icon", FontAwesomeIcon);

Vue.use(VueClipboard);
Vue.config.productionTip = false;
Vue.use(Buefy);

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
