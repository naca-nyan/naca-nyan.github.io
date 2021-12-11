<template>
  <body>
    <Header />
    <h1>いろいろ変換するやつ</h1>
    <hr />
    <main id="converter">
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

<script>
import { createApp } from "vue";
import { Options, Vue } from "vue-class-component";
import Header from "@/components/Header.vue"

const ConverterConfig = {
  data() {
    return {
      _semitone: 0,
      _percent: 100,
      ms: 500,
    };
  },
  computed: {
    semitone: {
      get() {
        return this._semitone;
      },
      set(val) {
        this._semitone = val;
        this._percent = Math.pow(2, val / 12) * 100;
      },
    },
    percent: {
      get() {
        return this._percent;
      },
      set(val) {
        this._percent = val;
        this._semitone = Math.log2(val / 100) * 12;
      },
    },
    bpm: {
      get() {
        return this.hz * 60;
      },
      set(val) {
        this.hz = val / 60;
      },
    },
    hz: {
      get() {
        return 1000 / this.ms;
      },
      set(val) {
        this.ms = 1000 / val;
      },
    },
  },
};
createApp(ConverterConfig).mount("#converter");

@Options({
  components: {
    Header,
  }
})
export default class Converter extends Vue {}
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