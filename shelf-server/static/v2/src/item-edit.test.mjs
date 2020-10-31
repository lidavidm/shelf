import test from "ava";

import * as itemEdit from "./item-edit.mjs";

test("complete blank item", (t) => {
    const item = itemEdit.completeNextEntry({ kind: "Manga", entries: [] });
    t.deepEqual(1, item.entries.length);
    t.deepEqual(null, item.entries[0].volume);
    t.deepEqual(1, item.entries[0].number);
    t.deepEqual("Chapter 1", item.entries[0].name.alternatives["English"]);
});

test("complete existing item", (t) => {
    const item = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [{ completed: true }, { completed: false }],
    });
    t.deepEqual(2, item.entries.length);
    t.true(typeof item.entries[1].completed === "string");
});

test("add new item", (t) => {
    const item = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [{ completed: true, volume: 1, number: 3 }],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(1, item.entries[1].volume);
    t.deepEqual(4, item.entries[1].number);
    t.deepEqual("Chapter 4", item.entries[1].name.alternatives["English"]);
});
