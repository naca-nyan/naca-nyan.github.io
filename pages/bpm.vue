<template>
  <Title>BPM計るやつ</Title>
  <div>
    <LogoHeader />
    <h1>BPM計るやつ</h1>
    <hr />
    <main>
      <p>
        <label>↓ボタンをタップ、もしくはテキストエリアでスペース</label>
        <button @click="tap()">Tap</button>
        <input type="text" @input.prevent="tap()" ref="tapinput" />
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
  </div>
</template>

<script setup lang="ts">
const taps = ref<number[]>([]);
const toBPM = (ms: number): number => {
  return 60000 / ms;
};
const tap = (): void => {
  taps.value.push(new Date().getTime());
};
const reset = (): void => {
  taps.value.length = 0;
};
const diffs = computed(() => {
  if (taps.value.length < 1) {
    return [];
  }
  const diffs: number[] = [];
  let timeBefore: number = taps.value[0];
  for (let i = 1; i < taps.value.length; i++) {
    const timeAfter = taps.value[i];
    diffs.push(timeAfter - timeBefore);
    timeBefore = timeAfter;
  }
  return diffs;
});
const average = computed(() => {
  const sum = diffs.value.reduce((acc, x) => acc + x, 0);
  const len = diffs.value.length;
  return sum / len;
});
</script>

<style scoped>
html {
  touch-action: manipulation;
}
</style>
