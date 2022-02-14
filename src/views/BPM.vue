<template>
  <body>
    <Header />
    <h1>BPM計るやつ</h1>
    <hr />
    <main>
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

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Header from "@/components/Header.vue";

@Options({
  components: {
    Header,
  },
})
export default class BPM extends Vue {
  created(): void {
    document.title = "BPM計るやつ | naca-nyan.github.io";
  }
  taps: number[] = [];
  toBPM(ms: number): number {
    return 60000 / ms;
  }
  tap(): void {
    this.taps.push(new Date().getTime());
  }
  reset(): void {
    this.taps.length = 0;
  }
  get diffs(): number[] {
    if (this.taps.length < 1) {
      return [];
    }
    const diffs: number[] = [];
    let timeBefore: number = this.taps[0];
    for (let i = 1; i < this.taps.length; i++) {
      const timeAfter = this.taps[i];
      diffs.push(timeAfter - timeBefore);
      timeBefore = timeAfter;
    }
    return diffs;
  }
  get average(): number {
    const sum = this.diffs.reduce((acc, x) => acc + x, 0);
    const len = this.diffs.length;
    return sum / len;
  }
}
</script>

<style scoped>
@import "../css/sakura-earthly.css";

body {
  padding-top: 0;
}
html {
  touch-action: manipulation;
}
</style>
