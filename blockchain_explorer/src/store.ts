import Vuex from "vuex";
import Vue from "vue";

Vue.use(Vuex);

const store = new Vuex.Store({
  state: {
    network_name: "stt",
    api_addr: process.env.VUE_APP_STEGOS_ADDR + "/graphql"
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
    network_name_full: (state, getters) => {
      if (state.network_name == "stg") {
        return "mainnet";
      } else if (state.network_name == "dev") {
        return "devnet";
      } else {
        return "testnet";
      }
    },
    api_addr: (state, getters) => {
      return state.api_addr;
    }
  }
});

export default store;
