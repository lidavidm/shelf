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

    async patch(newItem) {
        if (!newItem.key) {
            throw new Error("Item is missing a key");
        }
        store.update((items) => {
            items[newItem.key] = newItem;
            return items;
        });
        const response = await window.fetch(`/item/${newItem.key}`, {
            method: "POST",
            body: JSON.stringify(newItem),
            headers: {
                "Content-Type": "application/json",
            },
        });
        if (response.status >= 200 && response.status < 300) {
            this.update();
        } else {
            throw new Error(`Error ${response.status}: ${response.statusText}`);
        }
    },
};
