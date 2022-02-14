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
        <input type="time" step="600" v-model="time2" readonly />
      </p>
    </main>
  </body>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";
import Header from "../components/Header.vue";

export default defineComponent({
  components: { Header },
  setup() {
    document.title = "いろいろ変換するやつ | naca-nyan.github.io";
    const internal_percent = ref(100);
    const internal_semitone = ref(0);
    const percent = computed({
      get: () => {
        return internal_percent.value;
      },
      set: (val: number) => {
        internal_percent.value = val;
        internal_semitone.value = Math.log2(val / 100) * 12;
      },
    });
    const semitone = computed({
      get: () => {
        return internal_semitone.value;
      },
      set: (val: number) => {
        internal_semitone.value = val;
        internal_percent.value = Math.pow(2, val / 12) * 100;
      },
    });

    const ms = ref(500);
    const hz = computed({
      get: () => {
        return 1000 / ms.value;
      },
      set: (val: number) => {
        ms.value = 1000 / val;
      },
    });
    const bpm = computed({
      get: () => {
        return hz.value * 60;
      },
      set: (val: number) => {
        hz.value = val / 60;
      },
    });

    const watt1 = ref(500);
    const watt2 = ref(800);
    const time1 = ref("02:30");
    const sec1 = computed(() => {
      let [min, sec] = time1.value.split(":").map((s) => parseInt(s, 10));
      return min * 60 + sec;
    });
    const time2 = computed(() => {
      let sec2 = (sec1.value * watt1.value) / watt2.value;
      let min = ("00" + Math.floor(sec2 / 60)).slice(-2);
      let sec = ("00" + Math.round(sec2 % 60)).slice(-2);
      return `${min}:${sec}`;
    });
    return {
      percent,
      semitone,

      hz,
      bpm,
      ms,

      watt1,
      time1,
      watt2,
      time2,
    };
  },
});
</script>

<style scoped>
@import "../css/sakura-earthly.css";

body {
  padding-top: 0;
}

img.sns-icon {
  width: 35px;
  height: 35px;
  margin: 0 3px;
}
</style>
