<template>
  <section v-bind:id="id">
    <h2 class="title is-4">
      <a v-bind:href="'#' + id" class>#</a>
      {{ title }}
    </h2>
    <b-table
      :data="outputs"
      show-detail-icon
      hoverable
      paginated
      striped
      :per-page="10"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
    >
      <template slot-scope="props">
        <template v-if="props.row.outputType == 'payment_output'">
          <b-table-column class="is-size-7" field="output_hash" label="Output">
            <span style="word-wrap: anywhere">
              <Copyable
                :text="props.row.outputHash"
                title="Hash"
                :shrinked="props.row.hashShrinked"
              />
            </span>
          </b-table-column>
          <b-table-column class="is-size-7" field="recipient" label="Cloaked recipient:">
            <span style="word-wrap: anywhere">{{ props.row.recipientShrinked }}</span>
          </b-table-column>
          <b-table-column class="is-size-7" field="state" label="Status">
            <span v-if="props.row.spentInBlock" style="word-wrap: anywhere" class="tag is-warning">
              Spent in:
              <Copyable
                :text="props.row.spentInBlock"
                title="Block hash"
                :shrinked="props.row.spentShrinked"
              />
            </span>
            <span v-else style="word-wrap: anywhere" class="tag is-success">Unspent</span>
          </b-table-column>
          <!-- TODO: Add payment certificate check -->
          <!-- <b-table-column class="is-size-7" field="amount" label="Amount">{{ props.row.amount }}</b-table-column>
          <b-table-column class="is-size-7" field="recipient" label="Recipient">
            <span style="word-wrap: anywhere">{{ format_hash(props.row.recipient) }}</span>
          </b-table-column>-->
        </template>
        <template v-else>
          <b-table-column class="is-size-7" field="output_hash" label="Output">
            <span style="word-wrap: anywhere">
              <Copyable
                :text="props.row.outputHash"
                title="Hash"
                :shrinked="props.row.hashShrinked"
              />
            </span>
          </b-table-column>
          <b-table-column class="is-size-7" field="amount" label="Amount">{{ props.row.amount }}</b-table-column>
          <b-table-column class="is-size-7" field="recipient" label="Recipient">
            <span style="word-wrap: anywhere">
              <Copyable
                :text="props.row.recipient"
                title="Address"
                :shrinked="props.row.recipientShrinked"
              />
            </span>
          </b-table-column>
          <b-table-column class="is-size-7" field="state" label="Status">
            <span v-if="props.row.spentInBlock" style="word-wrap: anywhere" class="tag is-warning">
              Spent in:
              <Copyable
                :text="props.row.spentInBlock"
                title="Block hash"
                :shrinked="props.row.spentShrinked"
              />
            </span>
            <span v-else style="word-wrap: anywhere" class="tag is-success">Unspent</span>
          </b-table-column>
        </template>
      </template>
    </b-table>
  </section>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { ToastProgrammatic as Toast } from "buefy";
import Copyable from "@/components/copyable.vue";

@Component({
  name: "Outputs",
  components: {
    Copyable
  }
})
export default class Outputs extends Vue {
  @Prop()
  private title!: string;
  @Prop()
  private id!: string;
  @Prop()
  private outputs!: Array<any>;
}
</script>
