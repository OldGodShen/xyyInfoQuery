<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// const greetMsg = ref("");
// const name = ref("");
const cardno = ref("");
const res = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

async function query() {
  try {
  const result = await invoke("cardNoQuery", { cardNo: cardno.value }) as string;
  // 处理成功情况
  console.log("Success:", result);
  res.value = result; // 设置 res 的值为结果
} catch (error) {
  // 处理失败情况
  console.error("Error:", error);
}

}
</script>

<template>
  <!-- <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p> -->
  <form class="row" @submit.prevent="query">
    <input id="cardno-input" v-model="cardno" placeholder="Enter a cardno..." />
    <button type="submit">Query</button>
  </form>

  <p>{{ res }}</p>
</template>
