import test from "ava";

import * as itemEdit from "./item-edit.mjs";

test("complete blank item", (t) => {
    const [item] = itemEdit.completeNextEntry({ kind: "Manga", entries: [] });
    t.deepEqual(1, item.entries.length);
    t.deepEqual(null, item.entries[0].volume);
    t.deepEqual(1, item.entries[0].number);
    t.deepEqual("Chapter 1", item.entries[0].name.alternatives["English"]);
});

test("complete existing item", (t) => {
    const [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [{ completed: true }, { completed: false }],
    });
    t.deepEqual(2, item.entries.length);
    t.true(typeof item.entries[1].completed === "string");
});

test("add and complete new item", (t) => {
    const [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [{ completed: true, volume: 1, number: 3 }],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(1, item.entries[1].volume);
    t.deepEqual(4, item.entries[1].number);
    t.deepEqual("Chapter 4", item.entries[1].name.alternatives["English"]);
});

test("follow numbering/naming scheme", (t) => {
    let [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [
            {
                name: {
                    default: "English",
                    alternatives: { English: "Episode 5.1" },
                },
                completed: true,
                volume: 1,
                number: 1,
            },
        ],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(1, item.entries[1].volume);
    t.deepEqual(2, item.entries[1].number);
    t.deepEqual("Episode 6", item.entries[1].name.alternatives["English"]);

    [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [
            {
                name: {
                    default: "English",
                    alternatives: { English: "Volume 5" },
                },
                completed: true,
                volume: 5,
                number: 1,
            },
        ],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(6, item.entries[1].volume);
    t.deepEqual(2, item.entries[1].number);
    t.deepEqual("Volume 6", item.entries[1].name.alternatives["English"]);

    [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [
            {
                name: {
                    default: "English",
                    alternatives: { English: "Episode 25" },
                },
                completed: true,
                volume: 5,
                number: 1,
            },
        ],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(5, item.entries[1].volume);
    t.deepEqual(2, item.entries[1].number);
    t.deepEqual("Episode 26", item.entries[1].name.alternatives["English"]);

    [item] = itemEdit.completeNextEntry({
        kind: "Manga",
        entries: [
            {
                name: {
                    default: "English",
                    alternatives: { English: "Chapter 16: A Bad Omen" },
                },
                completed: true,
                volume: 1,
                number: 1,
            },
        ],
    });
    t.deepEqual(2, item.entries.length);
    t.deepEqual(1, item.entries[1].volume);
    t.deepEqual(2, item.entries[1].number);
    t.deepEqual("Chapter 17", item.entries[1].name.alternatives["English"]);
});

test("add new item", (t) => {
    let [item] = itemEdit.addNextEntry({
        kind: "Manga",
        entries: [],
    });
    t.deepEqual(1, item.entries.length);
    t.deepEqual(null, item.entries[0].volume);
    t.deepEqual(1, item.entries[0].number);
    t.deepEqual("Chapter 1", item.entries[0].name.alternatives["English"]);
    t.deepEqual(false, item.entries[0].completed);

    let entries;
    [item, entries] = itemEdit.addNextEntry(
        {
            kind: "Manga",
            entries: [],
        },
        5
    );
    t.deepEqual(5, item.entries.length);
    t.deepEqual(5, entries.length);
    for (let i = 0; i < 5; i++) {
        t.deepEqual(null, item.entries[i].volume);
        t.deepEqual(i + 1, item.entries[i].number);
        t.deepEqual(
            `Chapter ${i + 1}`,
            item.entries[i].name.alternatives["English"]
        );
        t.deepEqual(false, item.entries[i].completed);
    }

    [item, entries] = itemEdit.addNextEntry(
        {
            kind: "Manga",
            entries: [
                { completed: true, volume: 1, number: 1 },
                { completed: true, volume: 2, number: 2 },
            ],
        },
        5
    );
    t.deepEqual(7, item.entries.length);
    t.deepEqual(5, entries.length);
    for (let i = 0; i < 5; i++) {
        t.deepEqual(2, entries[i].volume);
        t.deepEqual(i + 3, entries[i].number);
        t.deepEqual(
            `Chapter ${i + 3}`,
            entries[i].name.alternatives["English"]
        );
        t.deepEqual(false, entries[i].completed);
    }
});

test("add volume", (t) => {
    let [item] = itemEdit.addNextEntry(
        {
            kind: "Manga",
            entries: [],
        },
        2,
        3
    );
    t.deepEqual(2, item.entries.length);
    t.deepEqual(3, item.entries[0].volume);
    t.deepEqual(1, item.entries[0].number);
    t.deepEqual("Chapter 1", item.entries[0].name.alternatives["English"]);
    t.deepEqual(false, item.entries[0].completed);
});

test("add volume and restart numbering", (t) => {
    let [item] = itemEdit.addNextEntry(
        {
            kind: "Manga",
            entries: [],
        },
        2,
        3
    );
    t.deepEqual(2, item.entries.length);
    t.deepEqual(3, item.entries[0].volume);
    t.deepEqual(1, item.entries[0].number);
    t.deepEqual("Chapter 1", item.entries[0].name.alternatives["English"]);
    t.deepEqual(false, item.entries[0].completed);

    [item] = itemEdit.addNextEntry(item, 2, 4, { restartNumbering: true });
    t.deepEqual(4, item.entries.length);
    t.deepEqual(4, item.entries[2].volume);
    t.deepEqual(3, item.entries[2].number);
    t.deepEqual("Chapter 1", item.entries[2].name.alternatives["English"]);

    t.deepEqual(4, item.entries[3].volume);
    t.deepEqual(4, item.entries[3].number);
    t.deepEqual("Chapter 2", item.entries[3].name.alternatives["English"]);

    [item] = itemEdit.addNextEntry(item, 1, null, { restartNumbering: true });
    t.deepEqual(5, item.entries.length);
    t.deepEqual(4, item.entries[4].volume);
    t.deepEqual(5, item.entries[4].number);
    t.deepEqual("Chapter 3", item.entries[4].name.alternatives["English"]);
});
