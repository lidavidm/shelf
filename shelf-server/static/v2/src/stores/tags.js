import { writable } from "svelte/store";

import PrefixTrie from "../prefixtrie.mjs";

const store = writable(new PrefixTrie());
export default {
    subscribe(callback) {
        return store.subscribe(callback);
    },

    update() {
        // TODO: debounce
        window
            .fetch("/tag")
            .then((r) => r.json())
            .then((tags) => store.set(new PrefixTrie(tags)));
    },
};
