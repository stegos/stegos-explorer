<template>
  <div id="app">
    <b-navbar wrapper-class="container" fixed-top type="is-dark">
      <template slot="brand">
        <b-navbar-item tag="router-link" :to="{ path: '/' }">Blockchain Explorer</b-navbar-item>
      </template>
      <template slot="end">
        <!-- <b-navbar-dropdown label="Settings">
          <b-navbar-item>
            <b-checkbox v-model="hasMicroblocks">Load microblocks</b-checkbox>
          </b-navbar-item>
          <b-navbar-item>
            <b-checkbox v-model="hasError">Show errors</b-checkbox>
          </b-navbar-item>
        </b-navbar-dropdown>-->
        <b-field class="navbar-item">
          <b-input
            ref="search"
            v-model="search_value"
            v-on:keyup.enter.native="search"
            @icon-click="search"
            placeholder="Search..."
            icon-clickable
            type="search"
            icon="search"
          ></b-input>
        </b-field>
      </template>
    </b-navbar>
    <div class="container">
      <router-view />
    </div>
    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          <strong>Blockchain explorer</strong> by
          <a href="https://github.com/vldm">Vladimir Motylenko</a>. The source
          code is licensed
          <a
            href="http://opensource.org/licenses/mit-license.php"
          >MIT</a>. You
          can download source code at
          <a href="https://github.com/vldm">Github</a>.
        </p>
      </div>
    </footer>
  </div>
</template>

<script>
export default {
  data() {
    return {
      hasMicroblocks: true,
      hasError: true,
      search_value: ""
    };
  },
  methods: {
    search() {
      console.log(this.search_value);
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