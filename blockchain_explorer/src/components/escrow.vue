<template>
  <div>
    <h4 class="title is-5">Staker group information</h4>
    <!--  TODO add current budget awards -->
    <b-table
      :data="escrow"
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

class EscrowEntry {
  validator: string;
  current_stake: number;
  last_day_blocks: number;
  stake_max: number;
}

@Component({
  name: "Escrow",
  components: {
    Copyable
  }
})

// Information about awards history.
export default class Escrow extends Vue {
  // @Prop()
  private escrow: Array<EscrowEntry> = [
    {
      validator: "Account1",
      current_stake: 511,
      last_day_blocks: 4,
      stake_max: 511
    },
    {
      validator: "Account2",
      current_stake: 412,
      last_day_blocks: 6,
      stake_max: 33
    },

    {
      validator: "Vladimir",
      current_stake: 512,
      last_day_blocks: 7,
      stake_max: 5556
    }
  ];
  private columns: Array<any> = [
    {
      field: "validator",
      label: "Validator key"
    },
    {
      field: "current_stake",
      label: "Current stake amount"
    },
    {
      field: "last_day_blocks",
      label: "Blocks mined last dat"
    },
    {
      field: "stake_max",
      label: "Maximum stake"
    }
  ];
}
</script>
