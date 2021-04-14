import test from "ava";

import importTitle from "./dynastyscans.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.beginning-their-new-life-together.html");

test("import Dynasty Scans oneshot", async (t) => {
    const url =
        "https://dynasty-scans.com/chapters/beginning_their_new_life_together#1";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(
        item.key,
        "manga-kodama-naoko-beginning-their-new-life-together"
    );
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.added, item.started);
    t.deepEqual(item.added, item.completed);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Beginning Their New Life Together"
    );
    t.deepEqual(item.entries.length, 1);
    t.deepEqual(item.entries[0].number, 1);
    t.deepEqual(item.entries[0].volume, null);
    t.deepEqual(item.entries[0].name.default, "English");
    t.deepEqual(item.entries[0].name.alternatives.English, "Oneshot");
    t.deepEqual(item.entries[0].completed, item.added);
    t.deepEqual(
        item.extra.external_url,
        "https://dynasty-scans.com/chapters/beginning_their_new_life_together"
    );
    t.deepEqual(
        cover,
        "https://dynasty-scans.com/system/releases/000/031/588/00001.jpeg"
    );
});
