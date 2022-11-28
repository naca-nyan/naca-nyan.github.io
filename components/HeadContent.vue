<script setup lang="ts">
import imgIcon from "~/assets/icon.png";

const props = defineProps<{
  title: string;
  description: string;
}>();

const longtitle = `${props.title} - naca-nyan.github.io`;
</script>
<template>
  <Title>{{ title }}</Title>
  <Meta name="og:title" :content="longtitle" />
  <Meta name="og:image" :content="imgIcon" />
  <Meta name="twitter:card" content="summary" />
  <Meta name="twitter:description" :content="description" />
  <Meta name="twitter:site" content="@naca-nyan" />
  <Meta name="twitter:creator" content="@naca-nyan" />
</template>
