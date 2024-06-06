<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const cardno = ref("");
const res = ref("");
async function query() {
  if (cardno.value == "") {
      res.value = "卡号呢?"
  } else {
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

}
</script>

<template>
  <form class="row" @submit.prevent="query">
    <input id="cardno-input" v-model="cardno" placeholder="Enter a cardno..." />
    <button type="submit">Query</button>
  </form>

  <p>{{ res }}</p>
</template>
