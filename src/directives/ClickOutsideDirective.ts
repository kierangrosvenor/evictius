import type { Directive } from 'vue';

type ClickOutsideElement = HTMLElement & { clickOutsideEvent?: (event: MouseEvent) => void };

const ClickOutsideDirective: Directive<ClickOutsideElement, (event: MouseEvent) => void> = {
    beforeMount(el, binding) {
        el.clickOutsideEvent = function(event) {
            if (!(el === event.target || el.contains(event.target as Node))) {
                binding.value(event);
            }
        };
        document.addEventListener('click', el.clickOutsideEvent);
    },
    unmounted(el) {
        document.removeEventListener('click', el.clickOutsideEvent!);
    },
};

export {ClickOutsideDirective}
