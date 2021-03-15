import { writable } from "svelte/store";

export default function makeStore(endpoint, defaultValue) {
    const store = writable(defaultValue);
    return {
        subscribe(callback) {
            return store.subscribe(callback);
        },

        update() {
            // TODO: debounce
            return window
                .fetch(endpoint)
                .then((r) => r.json())
                .then((items) => {
                    const result = {};
                    for (const item of items) {
                        result[item.key] =
                            item.name.alternatives[item.name.default];
                    }
                    store.update(() => result);
                });
        },
    };
}
