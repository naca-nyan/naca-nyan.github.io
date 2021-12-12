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
      <hr />
      <h4>電子レンジ秒数くん</h4>
      <p>
        <label></label>
        <input type="number" step="100" v-model="watt1" />W の
        <input type="time" step="600" v-model="time1" />は
        <label></label>
        <input type="number" step="100" v-model="watt2" />W の
        <input type="time" step="600" v-model="time2" />
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

  watt1 = 500;
  watt2 = 800;
  time1 = "02:30";
  get sec1(): number {
    let [min, sec] = this.time1.split(":").map((s) => parseInt(s, 10));
    return min * 60 + sec;
  }
  get time2(): string {
    let sec2 = (this.sec1 * this.watt1) / this.watt2;
    let min = ("00" + Math.floor(sec2 / 60)).slice(-2);
    let sec = ("00" + Math.round(sec2 % 60)).slice(-2);
    return `${min}:${sec}`;
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
