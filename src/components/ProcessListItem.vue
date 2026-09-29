<script setup lang="ts">
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { detectIcon } from "../icons";

const props = defineProps<{
  port: number;
  process: string;
  pid: number;
  command: string;
  shortCommand: string;
}>();

const icon = computed(() => detectIcon(props.process, props.command, props.shortCommand));

function killProcess(pid: number) {
  invoke("kill_process", { pid }).catch(console.error);
}
</script>
<template>
  <div class="list-item">
      <div class="process-cell">
        <span v-if="icon" class="icon" :style="{ color: icon.color }" v-html="icon.svg"></span>
        <div class="process-text">
          <div class="name">{{ process }}</div>
          <div class="command" :title="command">{{ shortCommand }}</div>
        </div>
      </div>
      <div>{{port}}</div> 
      <div>{{ pid }}</div>
      <div class="action">
        <button @click="killProcess(pid)">Kill</button>
      </div>
  </div>
</template>