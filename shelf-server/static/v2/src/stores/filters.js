import { writable } from "svelte/store";

const defaultFilters = {
    displayed: { "In Progress": true },
    tags: { sfw: false },
};
let filters = { ...defaultFilters };
if (window.localStorage["filters"]) {
    filters = JSON.parse(window.localStorage["filters"]);
}

const store = writable(filters);
export default {
    subscribe(callback) {
        return store.subscribe(callback);
    },

    set(value) {
        store.set(value);
        this.save(value);
    },

    save(value) {
        window.localStorage["filters"] = JSON.stringify(value);
    },
};
