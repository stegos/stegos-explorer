<template>
  <a @click="copy(text, title)">
    {{ shrinked }}
    <b-icon icon="content-copy" size="is-small"></b-icon>
  </a>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { ToastProgrammatic as Toast } from "buefy";

@Component
export default class Copyable extends Vue {
  @Prop()
  private title!: string;
  @Prop()
  private text!: string;
  @Prop()
  private shrinked!: string;
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
}
</script>
