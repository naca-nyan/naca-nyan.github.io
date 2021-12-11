<template>
  <header>
    <div class="logo-container">
      <router-link to="/">
        <img class="logo" src="../assets/icon.png" alt="icon" />
      </router-link>
    </div>
  </header>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";

export default class Header extends Vue {}
</script>

<style scoped>
header {
  display: flex;
  border-bottom: solid #fbfbfb 5px;
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
</style>