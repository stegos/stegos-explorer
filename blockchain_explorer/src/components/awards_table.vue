<template>
  <div>
    <h4 class="title is-5">Information about awards history</h4>
    <!--  TODO add current budget awards -->
    <b-table
      :data="awards"
      :columns="columns"
      show-detail-icon
      hoverable
      striped
      per-page="10"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
    ></b-table>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { ToastProgrammatic as Toast } from "buefy";
import Copyable from "@/components/copyable.vue";

class AwardsEntry {
  recipient: string;
  epoch: number;
  budget: number;
}

@Component({
  name: "Awards",
  components: {
    Copyable
  }
})

// Information about awards history.
export default class Awards extends Vue {
  // @Prop()
  private awards: Array<AwardsEntry> = [
    {
      recipient: "Test1",
      epoch: 1,
      budget: 100
    },
    {
      recipient: "Test2",
      epoch: 10,
      budget: 10000
    }
  ];
  private columns: Array<any> = [
    {
      field: "recipient",
      label: "Recipient"
    },
    {
      field: "epoch",
      label: "Awards epoch"
    },
    {
      field: "budget",
      label: "Awards budget"
    }
  ];
}
</script>
