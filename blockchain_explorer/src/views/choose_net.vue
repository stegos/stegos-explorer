<template>
  <div>
    <b-navbar wrapper-class="container" fixed-top type="is-dark">
      <template slot="brand">
        <b-navbar-item tag="router-link" :to="'/' + network_name_full">Stegos Explorer</b-navbar-item>
      </template>
      <!-- <template slot="start">
        <b-navbar-item tag="router-link" :to="{ path: '/' + network_name_full + '/blocks' }">Blocks</b-navbar-item>
        <b-navbar-item tag="router-link" :to="{ path: '/' + network_name_full + '/awards' }">Awards</b-navbar-item>
      <b-navbar-item tag="router-link" :to="{ path: '/' + network_name_full + '/escrow' }">Stakers</b-navbar-item>-->
      <!-- </template> -->
      <template slot="end">
        <!-- <b-navbar-item tag="router-link" :to="{ path: '/' + network_name_full + '/api' }">API</b-navbar-item> -->
        <!-- <b-navbar-dropdown label="Settings">
          <b-navbar-item>
            <b-checkbox v-model="hasMicroblocks">Load microblocks</b-checkbox>
          </b-navbar-item>
          <b-navbar-item>
            <b-checkbox v-model="hasError">Show errors</b-checkbox>
          </b-navbar-item>
        </b-navbar-dropdown>-->
        <div class="navbar-item">
          <b-field>
            <b-radio-button v-model="network" native-value="stt" type="is-warning">
              <span>Testnet</span>
            </b-radio-button>

            <b-radio-button v-model="network" native-value="stg" type="is-success">
              <span>Mainnet</span>
            </b-radio-button>
          </b-field>
        </div>
        <!-- <b-field class="navbar-item">
          <b-input
            ref="search"
            v-model="search_value"
            v-on:keyup.enter.native="search"
            @icon-click="search"
            placeholder="Search..."
            icon-clickable
            type="search"
            icon="magnify"
          ></b-input>
        </b-field>-->
      </template>
    </b-navbar>
    <section class="section">
      <div class="container">
        <router-view :key="network" />
      </div>
    </section>
    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          <strong>
            <a href="https://stegos.com">Stegos</a> Blockchain Explorer
          </strong> by
          <a href="https://github.com/vldm">Vladimir Motylenko</a>.
          The source
          code is licensed
          <a
            href="https://opensource.org/licenses/Apache-2.0"
          >Apache 2.0</a>. You
          can download source code at
          <a href="https://github.com/vldm">Github</a>.
        </p>
        <p></p>
      </div>
    </footer>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  data() {
    return {
      hasMicroblocks: true,
      hasError: true,
      search_value: ""
    };
  },
  computed: {
    network: {
      get() {
        return this.network_name();
      },
      set(val) {
        if (val == "stg") {
          this.$router.push("/mainnet");
        } else {
          this.$router.push("/testnet");
        }
      }
    }
  },
  methods: {
    ...mapGetters(["network_name", "network_name_full"]),
    search() {
      this.$router.push({ name: "search", params: { id: this.search_value } });
    }
  }
};
</script>
