<template>
  <b-table
    :data="data"
    :loading="loading"
    show-detail-icon
    hoverable
    paginated
    backend-pagination
    detailed
    custom-detail-row
    :total="count_of_epochs"
    :per-page="NUM_PER_PAGE"
    :current-page="(count_of_epochs - last_epoch) / NUM_PER_PAGE + 1"
    @page-change="set_page"
    @details-open="request_micro_blocks"
    aria-next-label="Next page"
    aria-previous-label="Previous page"
    aria-page-label="Page"
    aria-current-label="Current page"
  >
    <template slot-scope="props">
      <b-table-column field="epoch" label="Epoch">
        <router-link :to="'/block/' + props.row.epoch">{{ props.row.epoch }}</router-link>
      </b-table-column>
      <b-table-column field="block_hash" label="Block hash (previous)">
        <!-- <router-link :to="'/object/' + props.row.hash"> -->

        <Copyable :text="props.row.hash" title="Block hash" :shrinked="props.row.hashShrinked" />
        <!-- </router-link> -->
      </b-table-column>
      <b-table-column field="duration" label="Age" centered>{{ get_duration(props.row.timestamp) }}</b-table-column>
      <b-table-column field="microblocks" label="Micrloblocks count" centered>
        <span
          class="tag"
          :class="
              props.row.numMicroBlocks > 0 ? 'is-success' : 'is-warning'
            "
        >
          {{
          props.row.numMicroBlocks || "No microblocks found"
          }}
        </span>
      </b-table-column>
      <b-table-column field="inputs_len" label="Inputs count" centered>{{ props.row.inputsLen }}</b-table-column>
      <b-table-column field="outputs_len" label="Outputs count" centered>{{ props.row.outputsLen }}</b-table-column>

      <!--  -->
    </template>
    <template slot="detail" slot-scope="props">
      <template v-if="!props.row.numMicroBlocks">
        <tr class="detail">
          <td colspan="7">
            <b-notification :closable="false">
              <strong>No MicroBlocks found</strong>
              <br />It's looks like this MacroBlock was left without
              microblocks artifacts.
            </b-notification>
          </td>
        </tr>
      </template>
      <template v-else>
        <template v-if="!props.row.micro_blocks">
          <tr class="detail">
            <td colspan="7">
              <b-notification :closable="false">
                <p>
                  <strong>Loading epoch microblocks.</strong>
                </p>
                <b-loading :is-full-page="false" active></b-loading>
              </b-notification>
            </td>
          </tr>
        </template>
        <template v-else>
          <tr v-for="item in props.row.micro_blocks" :key="item.name">
            <td></td>
            <td>
              &nbsp;&nbsp;&nbsp;&nbsp;
              <router-link :to="'/block/' + item.epoch + '/' + item.offset">
                {{
                item.epoch + ":" + item.offset
                }}
              </router-link>
            </td>
            <td>
              <!-- <router-link :to="'/object/' + item.hash"> -->
              <Copyable :text="item.hash" title="Block hash" :shrinked="item.hashShrinked" />
              <!-- </router-link> -->
            </td>
            <td lass="has-text-centered">{{ get_duration(item.timestamp) }}</td>
            <td class="has-text-centered">{{ item.transactionsLen }}</td>
            <td class="has-text-centered">{{ item.inputsLen }}</td>
            <td class="has-text-centered">{{ item.outputsLen }}</td>
          </tr>
        </template>
      </template>
    </template>
  </b-table>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

import { get_duration, format_hash } from "@/utils";
import Copyable from "@/components/copyable.vue";
import { request } from "graphql-request";
import { mapGetters } from "vuex";
import { Getter } from "vuex-class";

@Component({
  components: {
    Copyable
  }
})
export default class BlockList extends Vue {
  private data: Array<any> = [];
  private loading: boolean = true;
  @Getter("network_name") network_name;
  @Getter("api_addr") api_addr;

  private NUM_PER_PAGE: number = 30;
  private count_of_epochs: number = 0;
  private last_epoch: number = 0;

  get_duration(value: string) {
    return get_duration(value);
  }

  request_blocks() {
    let epoch = this.last_epoch;
    let query = `{
        blocks(network: "${this.network_name}", startEpoch: ${epoch}, limit: ${this.NUM_PER_PAGE})
        {
          epoch,
          hash,
          timestamp,
          numMicroBlocks,
          inputsLen,
          outputsLen,

        }
      }`;

    request(this.api_addr, query).then(block_list => {
      block_list.blocks.forEach(element => {
        let data = format_hash(element.hash);
        element.hashShrinked = data.text;
      });
      this.loading = false;
      this.data = block_list.blocks;
    });
  }

  request_epochs_count() {
    let query = `{
        currentEpoch(network: "${this.network_name}")
      }`;
    request(this.api_addr, query).then(status => {
      this.set_epoch_from_status(status.currentEpoch);
      this.request_blocks();
    });
  }

  set_page(page: number) {
    this.last_epoch = this.count_of_epochs - (page - 1) * this.NUM_PER_PAGE;
    this.$router.push({
      name: "block_list",
      params: { last_epoch: this.last_epoch }
    } as any);
    this.request_blocks();
  }

  set_epoch_from_status(epoch: number) {
    this.count_of_epochs = epoch;
    this.last_epoch = this.last_epoch || epoch;
  }

  index_by_epoch(epoch: number) {
    return this.last_epoch - epoch;
  }

  request_micro_blocks(row: any) {
    let epoch = row.epoch;
    let new_index = this.index_by_epoch(epoch);
    let query = `{
      microBlocks(network: "${this.network_name}", epoch: ${epoch})
        {
          epoch,
          hash,
          timestamp,
          inputsLen,
          outputsLen,
          offset,
        }
      }`;

    request(this.api_addr, query).then(block_list => {
      row.micro_blocks = block_list.microBlocks;
      row.micro_blocks.forEach(element => {
        let data = format_hash(element.hash);
        element.hashShrinked = data.text;
      });
      this.$set(this.data, new_index, row);
    });
  }
  beforeRouteEnter(to: any, from: any, next: any) {
    next((vm: BlockList) => {
      vm.last_epoch = +to.params.last_epoch;
    });
  }

  created() {
    this.request_epochs_count();
  }
}
</script>
