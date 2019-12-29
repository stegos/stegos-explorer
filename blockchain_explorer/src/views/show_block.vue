<template>
  <section>
    <div class="columns is-gapless is-multiline is-mobile">
      <div class="column is-half">
        <table class="table">
          <tbody>
            <tr v-for="item in main.left" v-bind:key="item.name">
              <th data-label="Field Name" class="has-text-left" renderhtml="true">
                <span>{{ item.name + ":" }}</span>
              </th>
              <td data-label="Field data" class="has-text-right is-size-7" renderhtml="true">
                <span v-if="item.copyable" style="word-wrap: anywhere">
                  <Copyable :text="item.copyable" :title="item.title" :shrinked="item.value" />
                </span>

                <span v-else style="word-wrap: anywhere">{{ item.value }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="column is-half">
        <table class="table">
          <tbody>
            <tr v-for="item in main.right" v-bind:key="item.name">
              <th data-label="Field Name" class="has-text-left" renderhtml="true">
                <span>{{ item.name + ":" }}</span>
              </th>
              <td data-label="Field data" class="has-text-right is-size-7" renderhtml="true">
                <span v-if="item.copyable" style="word-wrap: anywhere">
                  <Copyable :text="item.copyable" :title="item.title" :shrinked="item.value" />
                </span>

                <span v-else style="word-wrap: anywhere">{{ item.value }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    <Outputs
      v-if="main.public_outputs.length"
      v-bind:outputs="main.public_outputs"
      id="public-outputs"
      title="Public outputs"
    />

    <Outputs
      v-if="main.stake_outputs.length"
      v-bind:outputs="main.stake_outputs"
      id="stake-outputs"
      title="Stake outputs"
    />
    <Outputs
      v-if="main.payment_outputs.length"
      v-bind:outputs="main.payment_outputs"
      id="payment-outputs"
      title="Payment outputs"
    />
  </section>
</template>



<script lang="ts">
// TODO:
// 1 add id of output.
// 2 add microblocks processing.
import Vue from "vue";
import { MacroBlock, MicroBlock, block_fields, format_hash } from "@/utils";
import { mapGetters } from "vuex";

import { request } from "graphql-request";
import { Route } from "vue-router";
import Outputs from "@/components/output.vue";

import Copyable from "@/components/copyable.vue";

import { ToastProgrammatic as Toast } from "buefy";

class Item {
  name: string;
  value: string;
  copyable?: string;
  title?: string;
  constructor(name: string, value: string) {
    this.name = name;
    this.value = value;
  }
}

export class BlockList extends Vue {
  epoch: number;
  offset: number | null;
  left: Array<Item>;
  right: Array<Item>;
  payment_outputs: Array<any>;
  public_outputs: Array<any>;
  stake_outputs: Array<any>;

  constructor() {
    super();
    this.epoch = 0;
    this.offset = null;
    this.left = [];
    this.right = [];
    this.payment_outputs = [];
    this.public_outputs = [];
    this.stake_outputs = [];
  }
}

export default Vue.extend({
  data() {
    return {
      main: new BlockList()
    };
  },
  beforeRouteEnter(to: Route, from: Route, next: any) {
    next((vm: any) => {
      vm.main.epoch = +to.params.epoch;
      vm.main.offset = +to.params.offset || null;

      vm.request_block();
    });
  },
  beforeRouteUpdate(to: Route, from: Route, next: any) {
    next();
  },
  methods: {
    ...mapGetters(["network_name", "api_addr"]),

    on_request(type: string, obj: any): void {
      console.log(type, obj);
      // save block for future usage.
      let outputs = [];
      if (obj.outputs !== undefined) {
        outputs = obj.outputs;
      } else {
        outputs = [];
      }
      let tmp;
      if (type === "macro") {
        tmp = new MacroBlock();
      } else {
        tmp = new MicroBlock();
      }

      let count = Math.ceil(Object.entries(tmp).length / 2);
      var counter = 0;

      // Traverse temporary object, by keys, to get limit number of keys.
      for (let key in tmp) {
        var value = (obj.block as any)[key];
        let field = block_fields()[key];
        let name = field.name;

        const item = new Item(name, value);
        if (field.truncate != null) {
          let output = field.truncate(item.value);
          if (output.copyable) {
            item.copyable = item.value;
            item.title = name;
          }
          item.value = output.text;
        }

        if (counter < count) {
          this.main.left.push(item);
        } else {
          this.main.right.push(item);
        }
        counter += 1;
      }
      this.main.stake_outputs = [];
      this.main.public_outputs = [];
      this.main.payment_outputs = [];

      for (let item of outputs) {
        if (item.outputHash) {
          let value = format_hash(item.outputHash);
          item.hashShrinked = value.text;
        }
        if (item.recipient) {
          let value = format_hash(item.recipient);
          item.recipientShrinked = value.text;
        }

        if (item.spentInBlock) {
          let value = format_hash(item.spentInBlock);
          item.spentShrinked = value.text;
        }
        if (item.outputType === "public_payments_output") {
          this.main.public_outputs.push(item);
        } else if (item.outputType === "stake_output") {
          this.main.stake_outputs.push(item);
        } else {
          this.main.payment_outputs.push(item);
        }
      }
      this.$forceUpdate();
    },

    request_block(): void {
      if (this.main.offset === null) {
        let fields = Object.keys(new MacroBlock()).reduce(
          (acc, x) => acc.concat(x + ", "),
          ""
        );
        let query = `{
          macroBlock(network: "stt", epoch:${this.main.epoch},) {
            block {
              numMicroBlocks,${fields}
            }
            outputs {
              amount,
              recipient,
              spentInBlock,
              outputType,
              outputHash,
            }
          }
        }`;
        request(this.api_addr(), query).then(data =>
          this.on_request("macro", data.macroBlock)
        );
      } else {
        let fields = Object.keys(new MicroBlock()).reduce(
          (acc, x) => acc.concat(x + ", "),
          ""
        );
        //TODO:: add transactions list
        let query = `{
        microBlock(network: "stt", epoch:${this.main.epoch},offset:${this.main.offset}) {
            block {
              ${fields}
            }
          }
      }`;

        request(this.api_addr(), query).then(data =>
          this.on_request("micro_block", data.microBlock)
        );
      }
    },
    copy(text: string, what: string) {
      console.log(text);
      (this as any).$copyText(text).then(
        function(e: any) {
          Toast.open(what + " " + text + " was copied to your buffer.");
        },
        function(e: any) {
          Toast.open({
            message: `Can't copy text, check is copy alowed in your browser.`,
            type: "is-danger"
          });
        }
      );
    }
  },
  components: {
    Outputs,
    Copyable
  }
});
</script>
