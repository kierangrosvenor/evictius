<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  port: number;
  process: string;
  pid: number;
}>();

function killProcess(pid: number) {
  invoke("kill_process", { pid });
}

</script>
<template>
    <div class="row">
      <div class="col-4">
        <p>Port {{ port }} is used by {{ process }} (PID {{ pid }}).</p>
        <button @click="killProcess(parseInt(pid.toString()))">Kill Process</button>
      </div>
    </div>
</template>