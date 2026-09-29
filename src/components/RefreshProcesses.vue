<script setup lang="ts">
import EvSelect from './EvSelect.vue';

const emit = defineEmits<{
  refreshNow: [];
  setRefreshInterval: [interval: number];
}>();

const props = defineProps<{
  interval: number;
}>();

const options = [
  { value: 1000, label: "1 second" },
  { value: 3000, label: "3 seconds" },
  { value: 5000, label: "5 seconds" },
  { value: 10000, label: "10 seconds" },
  { value: 30000, label: "30 seconds" },
  { value: 60000, label: "1 minute" },
];

function refreshNow() {
  emit("refreshNow");
}

function onIntervalChange(value: string | number) {
  emit("setRefreshInterval", Number(value));
}
</script>

<template>
    <div class="refresh-settings">

      <EvSelect :options="options" :modelValue="props.interval" @update:modelValue="onIntervalChange" /> 
      

       <button @click="refreshNow">Refresh now</button>
    </div>
</template>
<style>
.refresh-settings {
  display: flex;
  gap: 8px;
}
</style>