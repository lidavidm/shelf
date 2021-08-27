import test from "ava";

import importTitle from "./cubari.mjs";
import * as testUtil from "./test-util.mjs";

test("import Cubari oneshot", async (t) => {
    const proxy = testUtil.makeProxy("test.at-the-end-of-4-minutes.html");
    const url = "https://cubari.moe/read/imgur/oeJ1wKg/1/47/";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(
        item.key,
        "manga-cubari-at-the-end-of-4-minutes-translated-by-reica"
    );
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.added, item.started);
    t.deepEqual(item.added, item.completed);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "At the End of 4 Minutes (Translated by reica)"
    );
    t.deepEqual(item.entries.length, 1);
    t.deepEqual(item.entries[0].number, 1);
    t.deepEqual(item.entries[0].volume, null);
    t.deepEqual(item.entries[0].name.default, "English");
    t.deepEqual(item.entries[0].name.alternatives.English, "Oneshot");
    t.deepEqual(item.entries[0].completed, item.added);
    t.deepEqual(
        item.extra.external_url,
        "https://cubari.moe/read/imgur/oeJ1wKg/1/47/"
    );
    t.deepEqual(cover, "https://i.imgur.com/VNatRvT.jpg");
});
