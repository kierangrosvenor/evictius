<script setup lang="ts">
const emit = defineEmits<{
  refreshNow: [];
  setRefreshInterval: [interval: number];
}>();

defineProps<{
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

function onIntervalChange(event: Event) {
  const value = Number((event.target as HTMLSelectElement).value);
  emit("setRefreshInterval", value);
}
</script>

<template>
    <div class="refresh-settings">
      <select :value="interval" @change="onIntervalChange">
        <option v-for="o in options" :key="o.value" :value="o.value">
          {{ o.label }}
        </option>
      </select>

       <button @click="refreshNow">Refresh now</button>
    </div>
</template>
<style>
.refresh-settings {
  display: flex;
  gap: 8px;
}
</style>