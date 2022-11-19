<template>
  <p>
    <label></label>
    <input type="number" step="100" v-model="watt1" />W の
    <input type="time" step="600" v-model="time1" />は
    <label></label>
    <input type="number" step="100" v-model="watt2" />W の
    <input type="time" step="600" v-model="time2" readonly />
  </p>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from "vue";
export default defineComponent({
  setup: () => {
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
      watt1,
      time1,
      watt2,
      time2,
    };
  },
});
</script>
