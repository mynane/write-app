<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const tasks = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
  tasks.value = await invoke("get_all_tasks", {});

  console.log(tasks);
}
</script>

<template>
  <div class="card">
    123123
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>

  <ul>
    <li v-for="item of tasks.items">{{ item.name }}</li>
  </ul>
</template>
