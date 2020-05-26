<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  data() {
    return {
      hasMicroblocks: true,
      hasError: true,
      search_value: "",
      network_name: "Mainnet"
    };
  },
  methods: {
    search() {
      this.$router.push({ name: "search", params: { id: this.search_value } });
    }
  }
};
</script>

<style lang="scss">
// Import Bulma's core
@import "~bulma/sass/utilities/_all";

// Import Bulma and Buefy styles
@import "~bulma";
@import "~buefy/src/scss/buefy";
</style>