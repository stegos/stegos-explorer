<template>
  <div>
    <h4 class="title is-5">Information about awards history</h4>
    <!--  TODO add current budget awards -->
    <b-table
      :data="awards"
      :paginated="paginated"
      show-detail-icon
      hoverable
      striped
      :loading="loading"
      :per-page="NUM_PER_PAGE"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
    >
      <template slot-scope="props">
        <b-table-column centered field="epoch" label="Epoch">
          <router-link
            :to="'/' + network_name_full + '/block/' + props.row.epoch"
          >{{props.row.epoch}}</router-link>
        </b-table-column>
        <b-table-column field="validator" label="Validator">
          <Copyable
            :text="props.row.validator"
            title="Validator key"
            :shrinked="props.row.validatorShrinked"
          />
        </b-table-column>
        <b-table-column field="budget" label="Amount" centered>{{ props.row.budget }}</b-table-column>

        <b-table-column field="spent" label="Spent" centered>
          <span v-if="props.row.spentInBlock" class="tag is-warning">
            Spent in:
            <Copyable
              :text="props.row.spentInBlock"
              title="Block hash"
              :shrinked="props.row.spentInBlockShrinked"
            />
          </span>
          <span v-else class="tag is-success">Not spent</span>
        </b-table-column>
        <!-- <b-table-column field="time" label="Time" centered>
        <b-tooltip
          :label="props.row.timestamp"
          type="is-dark"
          animated
        >{{ get_duration(props.row.timestamp) }}</b-tooltip>
        </b-table-column>-->

        <!--  -->
      </template>
    </b-table>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Watch, Vue } from "vue-property-decorator";
import { ToastProgrammatic as Toast } from "buefy";

import { format_pkey, format_hash_small } from "@/utils";
import Copyable from "@/components/copyable.vue";
import { request } from "graphql-request";
import { Getter } from "vuex-class";

interface AwardsEntry {
  validator: string;
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
  @Prop()
  private paginated!: boolean;
  @Prop()
  private count_of_epochs!: number;
  private NUM_PER_PAGE: number = 10;
  private last_epoch: number = 0;
  private loading: boolean = false;
  @Getter("network_name") network_name: any;
  @Getter("network_name_full") network_name_full: any;
  @Getter("api_addr") api_addr: any;

  @Watch("count_of_epochs")
  on_epoch_changed(value: number, oldValue: number) {
    if (value != oldValue) {
      this.last_epoch = this.last_epoch || this.count_of_epochs;

      let query = `{
        awards(network: "${this.network_name}", startEpoch: ${this.last_epoch}, limit: ${this.NUM_PER_PAGE})
        {
          validator,
          epoch,
          budget,
          spentInBlock,
          timestamp
        }
      }`;

      request(this.api_addr, query).then(awards => {
        awards.awards.forEach((element: any) => {
          element.budget /= 1000000;
          
          let data = format_pkey(element.validator);
          element.validatorShrinked = data.text;

          if (element.spentInBlock !== null) {
            let block_data = format_hash_small(element.spentInBlock);
            element.spentInBlockShrinked = block_data.text;
          }
        });
        this.loading = false;
        this.awards = awards.awards;
      });
    }
  }
  // @Prop()
  private awards: Array<AwardsEntry> = [];
}
</script>
