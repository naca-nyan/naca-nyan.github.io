<template>
  <div>
    <LogoHeader />
    <h1>いろいろ変換するやつ</h1>
    <hr />
    <main>
      <h4>ボイチェンの％と半音単位こんばーた</h4>
      <ConverterPitch />
      <hr />
      <h4>HzとBPMとms行ったり来たり</h4>
      <ConverterBPM />
      <hr />
      <h4>電子レンジ秒数くん</h4>
      <ConverterMicrowave />
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import LogoHeader from "../components/LogoHeader.vue";
import ConverterPitch from "../components/ConverterPitch.vue";
import ConverterBPM from "../components/ConverterBPM.vue";
import ConverterMicrowave from "../components/ConverterMicrowave.vue";

export default defineComponent({
  components: { LogoHeader, ConverterPitch, ConverterBPM, ConverterMicrowave },
  setup() {
    document.title = "いろいろ変換するやつ | naca-nyan.github.io";
  },
});
</script>

<style scoped>
@import "../css/sakura-earthly.css";
</style>
