<template>
  <Title>Home</Title>
  <header>
    <div class="image-cropper">
      <img class="profile-pic" src="~/assets/icon.png" alt="icon" />
    </div>
    <div class="name">なかにゃん @naca_nyan</div>
    <div class="sns-list">
      <a v-for="s in snsList" :key="s.name" :href="s.href">
        <img class="sns-icon" :src="s.img" :alt="s.name" />
      </a>
    </div>
  </header>
  <h1>便利なやつら</h1>
  <hr />
  <p>作ったやつ</p>
  <ul>
    <li><a href="converter/">いろいろ変換するやつ</a></li>
    <li><a href="bpm/">ぽちぽちしてBPM計測するやつ</a></li>
  </ul>
</template>

<script setup lang="ts">
import imgTwitter from "~/assets/twitter.svg";
import imgGithub from "~/assets/github.svg";
import imgVrchat from "~/assets/vrchat.png";
import imgYoutube from "~/assets/youtube.svg";
import imgSkeb from "~/assets/skeb.png";

interface SNS {
  name: string;
  href: string;
  img: any;
}

// document.title = "Home | naca-nyan.github.io";
const snsList: SNS[] = [
  {
    name: "twitter",
    href: "https://twitter.com/naca_nyan",
    img: imgTwitter,
  },
  {
    name: "github",
    href: "https://github.com/naca-nyan",
    img: imgGithub,
  },
  {
    name: "vrchat",
    href: "https://vrchat.com/home/user/usr_08cf7780-e093-4e29-a083-2bc7e4957247",
    img: imgVrchat,
  },
  {
    name: "youtube",
    href: "https://www.youtube.com/channel/UCBBy1F0Jb_9DvRTV2kigHpw",
    img: imgYoutube,
  },
  {
    name: "skeb",
    href: "https://skeb.jp/@naca_nyan",
    img: imgSkeb,
  },
];
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
