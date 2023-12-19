<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";

async function listen_to_event() {
  await listen("clip_event", (event: any) => {
    console.log(event.paylod);
  });
}
onMounted(() => {
  listen_to_event();
});
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
