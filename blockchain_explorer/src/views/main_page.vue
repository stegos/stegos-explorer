<template>
  <div>
    <b-loading is-full-page :active="!count_of_epochs"></b-loading>
    <!-- <div class="columns is-gapless is-multiline is-mobile"> -->
    <div class="column">
      <Awards :count_of_epochs="count_of_epochs" :paginated="false" />
    </div>
    <!-- <div class="column is-half">
        <Escrow :count_of_epochs="count_of_epochs" :paginated="false" />
    </div>-->
    <!-- </div> -->
    <section>
      <div class="column is-full">
        <p class="title">Blocks list</p>
        <BlockList :count_of_epochs="count_of_epochs" :paginated="false" />
      </div>
    </section>
  </div>
</template>

<script>
import BlockList from "@/components/block_list";
import Awards from "@/components/awards_table";
import Escrow from "@/components/escrow";
import { request } from "graphql-request";
import { mapGetters } from "vuex";
export default {
  data() {
    return {
      data: [],
      count_of_epochs: 0
    };
  },
  methods: {
    ...mapGetters(["network_name", "api_addr"]),

    request_epochs_count() {
      let query = `{
          currentEpoch(network: "${this.network_name()}")
        }`;
      request(this.api_addr(), query).then(status => {
        this.set_epoch_from_status(status.currentEpoch);
      });
    },
    set_epoch_from_status(epoch) {
      this.count_of_epochs = epoch;
    }
  },
  created() {
    this.request_epochs_count();
  },
  components: {
    BlockList,
    Awards
    // Escrow
  }
};
</script>
