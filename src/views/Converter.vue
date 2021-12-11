<template>
  <body>
    <header>
      <div class="logo-container">
        <a href="/"><img class="logo" src="../assets/icon.png" alt="icon" /></a>
      </div>
    </header>
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
import { Vue } from "vue-class-component";

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

export default class Converter extends Vue {}
</script>

<style scoped>
body {
  padding-top: 0;
}
header {
  display: flex;
  border-bottom: solid #fbfbfb 5px;
}
header .name {
  font-weight: 600;
  margin-bottom: 16px;
}
header a:hover {
  border-bottom: none;
}
.logo-container {
  width: 50px;
  height: 50px;
  position: relative;
  overflow: hidden;
  border-radius: 50%;
  margin: 15px 10px 10px 20px;
}
img.logo {
  display: inline;
  margin: 0 auto;
  height: 100%;
  width: auto;
}
img.sns-icon {
  width: 35px;
  height: 35px;
  margin: 0 3px;
}
</style>