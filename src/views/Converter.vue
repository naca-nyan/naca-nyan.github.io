<template>
  <body>
    <Header />
    <h1>いろいろ変換するやつ</h1>
    <hr />
    <main>
      <h4>ボイチェンの％と半音単位こんばーた</h4>
      <p>
        <label>パーセント</label>
        <input type="text" v-model="percent" />%
        <label>半音単位</label>
        <input type="text" v-model="semitone" />st
      </p>
      <hr />
      <h4>HzとBPMとms行ったり来たり</h4>
      <p>
        <label>Hz</label>
        <input type="text" v-model="hz" />Hz
        <label>BPM</label>
        <input type="text" v-model="bpm" />
        <label>ms</label>
        <input type="text" v-model="ms" />ms
      </p>
    </main>
  </body>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Header from "@/components/Header.vue";

@Options({
  components: {
    Header,
  },
})
export default class Converter extends Vue {
  private internal_semitone = 0;
  private internal_percent = 100;
  ms = 500;
  get semitone(): number {
    return this.internal_semitone;
  }
  set semitone(val: number) {
    this.internal_semitone = val;
    this.internal_percent = Math.pow(2, val / 12) * 100;
  }
  get percent(): number {
    return this.internal_percent;
  }
  set percent(val: number) {
    this.internal_percent = val;
    this.internal_semitone = Math.log2(val / 100) * 12;
  }
  get bpm(): number {
    return this.hz * 60;
  }
  set bpm(val: number) {
    this.hz = val / 60;
  }
  get hz(): number {
    return 1000 / this.ms;
  }
  set hz(val: number) {
    this.ms = 1000 / val;
  }
}
</script>

<style scoped>
body {
  padding-top: 0;
}

img.sns-icon {
  width: 35px;
  height: 35px;
  margin: 0 3px;
}
</style>
