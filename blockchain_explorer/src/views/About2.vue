<template>
  <div class="container">
    <b-field label="Request:">
      <b-input maxlength="20000" type="textarea" placeholder="Request:" v-model="data"></b-input>
      <p class="control">
        <b-button @click="request_data" class="button is-primary">Request</b-button>
      </p>
    </b-field>
  </div>
</template>


<script>
import { get_duration } from "@/utils";
import { request } from "graphql-request";

export default {
  data() {
    return {
      data: "",
      ws: new Websocket(this),
      active: false
    };
  },

  methods: {
    set_active() {
      this.active = true;
    },
    request_data() {
      request("http://127.0.0.1:3000/graphql", this.data).then(data =>
        console.log(data)
      );
    }
  },
  created: function() {
    this.ws.onload(this.set_active);
  }
};
</script>
