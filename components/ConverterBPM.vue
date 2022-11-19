<template>
  <p>
    <label>Hz</label>
    <input type="text" v-model="hz" />Hz
    <label>BPM</label>
    <input type="text" v-model="bpm" />
    <label>ms</label>
    <input type="text" v-model="ms" />ms
  </p>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from "vue";

export default defineComponent({
  setup: () => {
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
    return {
      hz,
      bpm,
      ms,
    };
  },
});
</script>
