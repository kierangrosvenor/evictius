<script lang="ts" setup>
import { computed, ref } from 'vue';
import { useFloating, autoUpdate, offset, flip, shift, size } from '@floating-ui/vue';

type EvSelectOption = {
    label: string,
    value: string | number
}

const props = defineProps<{
    options: EvSelectOption[],
    modelValue: string | number,
    placeholder?: string,
}>();

const emit = defineEmits<{
    'update:modelValue': [value: string | number]
}>();

const opener = ref<HTMLElement | null>(null);
const menu = ref<HTMLElement | null>(null);

const open = ref(false);

const { floatingStyles } = useFloating(opener, menu, {
    open,
    placement: 'bottom-start',
    strategy: 'fixed',
    whileElementsMounted: autoUpdate,
    middleware: [
        offset(4),
        flip(),
        shift({ padding: 8 }),
        size({
            apply({ rects, elements }) {
                elements.floating.style.minWidth = `${rects.reference.width}px`;
            }
        }),
    ],
});

const selected = computed(() => props.options.find(o => o.value === props.modelValue));

function toggle() {
    open.value = !open.value;
}

function close() {
    open.value = false;
}

function choose(option: EvSelectOption) {
    emit('update:modelValue', option.value);
    close();
}
</script>

<template>
    <div class="ev-select" v-click-outside="close" @keydown.esc="close">
        <button
            ref="opener"
            type="button"
            class="ev-select-opener"
            aria-haspopup="listbox"
            :aria-expanded="open"
            @click="toggle"
        >
            {{ selected?.label ?? placeholder ?? 'Select…' }}
            <span class="ev-select-caret" :class="{ open }"></span>
        </button>

        <div v-if="open" ref="menu" class="ev-select-menu" role="listbox" :style="floatingStyles">
            <button
                v-for="item in props.options"
                :key="item.value"
                type="button"
                role="option"
                class="ev-select-option"
                :class="{ selected: item.value === modelValue }"
                :aria-selected="item.value === modelValue"
                @click="choose(item)"
            >
                {{ item.label }}
            </button>
        </div>
    </div>
</template>
