<template>
  <p>
    <label>パーセント</label>
    <input type="text" v-model="percent" />%
    <label>半音単位</label>
    <input type="text" v-model="semitone" />st
  </p>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from "vue";
export default defineComponent({
  setup: () => {
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
    return {
      percent,
      semitone,
    };
  },
});
</script>
