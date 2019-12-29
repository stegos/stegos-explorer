<template>
  <div class="columns is-gapless is-multiline is-mobile">
    <div class="column is-half">
      <Awards />
    </div>
    <div class="column is-half">
      <Escrow />
    </div>
    <section>
      <div class="column is-full">
        <h4 class="title is-5">Block info:</h4>
        <BlockList />
      </div>
    </section>
  </div>
</template>

<script>
import BlockList from "@/components/block_list";
import Awards from "@/components/awards_table";
import Escrow from "@/components/escrow";
export default {
  data() {
    return {
      data: []
    };
  },
  components: {
    BlockList,
    Awards,
    Escrow
  }
};
</script>
