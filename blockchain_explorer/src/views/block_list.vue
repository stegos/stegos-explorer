<template>
  <section>
    <BlockList />
  </section>
</template>

<script>
import BlockList from "@/components/block_list";
export default {
  data() {
    return {
      data: []
    };
  },
  components: {
    BlockList
  }
};
</script>
