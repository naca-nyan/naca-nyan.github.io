<template>
  <div>
    <header>
      <div class="image-cropper">
        <img class="profile-pic" src="../assets/icon.png" alt="icon" />
      </div>
      <div class="name">なかにゃん @naca_nyan</div>
      <div class="sns-list">
        <a v-for="s in snsList" :key="s.name" :href="s.href">
          <img class="sns-icon" :src="s.img" :alt="s.name"
        /></a>
      </div>
    </header>
    <h1>便利なやつら</h1>
    <hr />
    <p>作ったやつ</p>
    <ul>
      <li><router-link to="/converter/">いろいろ変換するやつ</router-link></li>
      <li><router-link to="/bpm/">ぽちぽちしてBPM計測するやつ</router-link></li>
    </ul>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";

export default class Home extends Vue {
  snsList = [
    {
      name: "twitter",
      href: "https://twitter.com/naca_nyan",
      img: require("@/assets/twitter.svg"),
    },
    {
      name: "github",
      href: "https://github.com/naca-nyan",
      img: require("@/assets/github.svg"),
    },
    {
      name: "vrchat",
      href: "https://vrchat.com/home/user/usr_08cf7780-e093-4e29-a083-2bc7e4957247",
      img: require("@/assets/vrchat.png"),
    },
    {
      name: "youtube",
      href: "https://www.youtube.com/channel/UCBBy1F0Jb_9DvRTV2kigHpw",
      img: require("@/assets/youtube.svg"),
    },
    {
      name: "skeb",
      href: "https://skeb.jp/@naca_nyan",
      img: require("@/assets/skeb.png"),
    },
  ];
}
</script>

<style scoped>
header {
  display: flex;
  align-items: center;
  flex-direction: column;
}
header .name {
  font-weight: 600;
  margin-bottom: 16px;
}
header a:hover {
  border-bottom: none;
}
.image-cropper {
  width: 250px;
  height: 250px;
  margin-bottom: 20px;
  position: relative;
  overflow: hidden;
  border-radius: 50%;
}
img.profile-pic {
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
