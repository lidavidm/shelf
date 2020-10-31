import { writable } from "svelte/store";

const defaultOptions = {
    title: "",
    body: "",
    timeoutMs: 5000,
};
const store = writable([]);
let id = 0;

export default {
    subscribe(callback) {
        return store.subscribe(callback);
    },

    push(options) {
        const toast = { ...defaultOptions, ...options, id: id++ };
        window.setTimeout(() => {
            store.update((toasts) => {
                toasts = toasts.filter((other) => other !== toast);
                return toasts;
            });
        }, toast.timeoutMs);
        store.update((toasts) => {
            toasts.push(toast);
            return toasts;
        });
    },
};
