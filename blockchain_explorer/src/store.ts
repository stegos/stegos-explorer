import Vuex from "vuex";
import Vue from "vue";

Vue.use(Vuex);

const store = new Vuex.Store({
  state: {
    network_name: "stt",
    api_addr: "http://127.0.0.1:3000/graphql"
  },
  mutations: {
    set_mainnet(state) {
      state.network_name = "stg";
    },
    set_testnet(state) {
      state.network_name = "stt";
    },
    set_devnet(state) {
      state.network_name = "dev";
    }
  },
  getters: {
    network_name: (state, getters) => {
      return state.network_name;
    },
    api_addr: (state, getters) => {
      return state.api_addr;
    }
  }
});

export default store;
