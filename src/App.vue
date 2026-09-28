<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PortRowItem from "./components/PortRowItem.vue";
import RefreshSettings from "./components/RefreshSettings.vue";

type PortInfo = { port: number; pid: number; process: string };

const inUsePorts = ref<PortInfo[]>([]);

async function getInUse() {
  inUsePorts.value = await invoke("get_ports");
}

const refreshInterval = ref<number>(3000);

const setRefreshInterval = (interval: number) => {
  refreshInterval.value = interval;
};

let timer: ReturnType<typeof setInterval> | undefined;

function startTimer() {
  clearInterval(timer);
  timer = setInterval(getInUse, refreshInterval.value);
}

watch(refreshInterval, startTimer);

onMounted(() => {
  getInUse();
  startTimer();
});

onUnmounted(() => clearInterval(timer));
</script>

<template>
  <main class="container">
    <h1>In Use Ports</h1>
    <RefreshSettings
     @refreshNow="getInUse" 
     @setRefreshInterval="setRefreshInterval"
     :interval="refreshInterval"/>
    <PortRowItem 
      v-for="p in inUsePorts" 
      :key="p.port" 
      :port="p.port" 
      :process="p.process" 
      :pid="p.pid"
      />
  </main>
</template>