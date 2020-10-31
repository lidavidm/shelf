import { writable } from "svelte/store";

const store = writable([]);
export default {
    subscribe(callback) {
        return store.subscribe(callback);
    },

    update() {
        // TODO: debounce
        window
            .fetch("/item")
            .then((r) => r.json())
            .then((items) => {
                const result = {};
                for (const item of items) {
                    result[item.key] = item;
                }
                store.update(() => result);
            });
    },

    patch(newItem) {
        // TODO: also make REST call
        store.update((items) => {
            items[newItem.key] = newItem;
            return items;
        });
    },
};
