<template>
  <body>
    <header>
      <div class="logo-container">
        <a href="/"><img class="logo" src="../assets/icon.png" alt="icon" /></a>
      </div>
    </header>
    <h1>BPM計るやつ</h1>
    <hr />
    <main id="BPMApp">
      <p>
        <label>↓ボタンをタップ、もしくはテキストエリアでスペース</label>
        <button @click="tap()">Tap</button>
        <input
          type="text"
          @input="
            tap();
            $event.target.value = '';
          "
          ref="tapinput"
        />
        <button @click="reset()">Reset</button>
      </p>
      <div v-if="taps.length === 1">First Tap</div>
      <div v-if="taps.length >= 2">
        Average: {{ toBPM(average) }}({{ average }}ms)
      </div>
      <ol>
        <li v-for="time in diffs" :key="time">
          {{ toBPM(time) }} ({{ time }}ms)
        </li>
      </ol>
    </main>
  </body>
</template>

<script>
import { createApp } from "vue";
import { Vue } from "vue-class-component";

const BPMConf = {
  data() {
    return {
      taps: [],
    };
  },
  methods: {
    toBPM(ms) {
      return 60000 / ms;
    },
    tap() {
      this.taps.push(new Date());
    },
    reset() {
      this.taps.length = 0;
    },
  },
  computed: {
    diffs() {
      if (this.taps.length < 1) {
        return [];
      }
      let diffs = [];
      let y = this.taps[0];
      for (let i = 1; i < this.taps.length; i++) {
        const x = this.taps[i];
        diffs.push(x - y);
        y = x;
      }
      return diffs;
    },
    average() {
      const sum = this.diffs.reduce((acc, x) => acc + x, 0);
      const len = this.diffs.length;
      return sum / len;
    },
  },
};

createApp(BPMConf).mount("#BPMApp");

export default class BPM extends Vue {}
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
html {
  touch-action: manipulation;
}
</style>