export default class PrefixTrie {
    constructor(items) {
        this.root = new Node();
        if (items) {
            for (const item of items) {
                this.root.insert(item, item);
            }
        }
    }

    beginsWith(word) {
        let prefix = word;
        let node = this.root;
        while (prefix.length > 0) {
            const c = prefix.slice(0, 1);
            prefix = prefix.slice(1);
            if (node.children.has(c)) {
                node = node.children.get(c);
            } else {
                return new Set();
            }
        }
        return node.words;
    }
}

class Node {
    constructor() {
        this.children = new Map();
        this.words = new Set();
    }

    insert(word, suffix) {
        this.words.add(word);
        if (suffix.length > 0) {
            const prefix = suffix.slice(0, 1);
            const newSuffix = suffix.slice(1);
            if (!this.children.has(prefix)) {
                this.children.set(prefix, new Node());
            }
            this.children.get(prefix).insert(word, newSuffix);
        }
    }
}
