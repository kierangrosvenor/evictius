<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from "vue"
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import ProcessListItem from "./components/ProcessListItem.vue";
import RefreshProcesses from "./components/RefreshProcesses.vue";

type PortInfo = { port: number; pid: number; process: string; command: string; shortCommand: string };

const isDark = ref(false);
let mediaQuery = null as unknown as MediaQueryList;
let processChangedUnlistener: UnlistenFn | undefined;
const searchValue = ref<string>("");
const inUsePorts = ref<PortInfo[]>([]);
const refreshInterval = ref<number>(3000);

const sortKey = ref<keyof PortInfo | null>();
const sortDirection = ref<string>("asc");
  

const updateTheme = (e: MediaQueryListEvent) => {
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDark.value = savedTheme === 'dark'
  } else {
    isDark.value = e ? e.matches : mediaQuery.matches
  }

  if (isDark.value) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

async function getInUse() {
  inUsePorts.value = await invoke("get_processes")
}


const setRefreshInterval = (interval: number) => {
  refreshInterval.value = interval;
};

let timer: ReturnType<typeof setInterval> | undefined;

function startTimer() {
  clearInterval(timer);
  timer = setInterval(getInUse, refreshInterval.value);
}

watch(refreshInterval, startTimer);

onMounted(async () => {
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  getInUse();
  startTimer();

  processChangedUnlistener = await listen("processes_changed", () => {
    alert('Fetching...')
    getInUse();
  });

  mediaQuery.addEventListener('change', updateTheme)
});

const filteredPorts = computed(() => {
  const search = searchValue.value.toLowerCase();
  const list = inUsePorts.value.filter((p) => p.process.toLowerCase().includes(search));

  const key = sortKey.value;
  if (!key) return list;

  const dir = sortDirection.value === "asc" ? 1 : -1;
  return list.sort((a, b) => {
    const result = key === "process"
      ? a.process.localeCompare(b.process)
      : Number(a[key]) - Number(b[key]); 
    return result * dir;
  });
});

// "asc" or "desc" on the column being sorted, nothing on the rest.
function sortClass(keyName: string) {
  return sortKey.value === keyName ? sortDirection.value : "";
}

function setSort(keyName: keyof PortInfo) {
  sortKey.value = keyName;
  sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc";
}

onUnmounted(() => {
  clearInterval(timer); 
  processChangedUnlistener?.();
  if(mediaQuery) {
      mediaQuery.removeEventListener('change', updateTheme);
  }

});
</script>

<template>
  <main class="container" :class="{ dark: isDark }">
    <section class="toolbar">
      <input type="text" v-model="searchValue" placeholder="Search by process name" />
      <RefreshProcesses @refreshNow="getInUse" @setRefreshInterval="setRefreshInterval" :interval="refreshInterval" />
    </section>
    <div class="list">
      <div class="list-item list-header">
        <div class="sortable" :class="sortClass('process')" @click="setSort('process')">Process</div>
        <div class="sortable" :class="sortClass('port')" @click="setSort('port')">Port</div>
        <div class="sortable" :class="sortClass('pid')" @click="setSort('pid')">PID</div>
        <div class="action"></div>
      </div>
      <ProcessListItem v-for="p in filteredPorts" :key="p.port" :port="p.port" :process="p.process" :pid="p.pid" :command="p.command" :short-command="p.shortCommand" />
    </div>
  </main>
</template>
<style lang="scss">
@use "./style/common.scss";
</style>