import test from "ava";

import importTitle from "./dynastyscans.mjs";
import * as testUtil from "./test-util.mjs";

test("import Dynasty Scans oneshot", async (t) => {
    const proxy = testUtil.makeProxy(
        "test.beginning-their-new-life-together.html"
    );
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

test("import Dynasty Scans series", async (t) => {
    const proxy = testUtil.makeProxy("test.march-comes-in-like-a-lion.html");
    const url = "https://dynasty-scans.com/series/march_comes_in_like_a_lion";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-umino-chika-march-comes-in-like-a-lion");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.added, item.started);
    t.deepEqual(null, item.completed);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "March Comes in Like a Lion"
    );
    t.deepEqual(item.entries.length, 33);
    t.deepEqual(item.entries[0].number, 1);
    t.deepEqual(item.entries[0].volume, 1);
    t.deepEqual(item.entries[0].name.default, "English");
    t.deepEqual(
        item.entries[0].name.alternatives.English,
        "Chapter 1: Kiriyama Rei"
    );
    t.deepEqual(item.entries[0].completed, null);

    t.deepEqual(item.entries[22].number, 23);
    t.deepEqual(item.entries[22].volume, 2);
    t.deepEqual(item.entries[22].name.default, "English");
    t.deepEqual(item.entries[22].name.alternatives.English, "Vol 2 Omake");
    t.deepEqual(item.entries[22].completed, null);

    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://dynasty-scans.com/system/tag_contents_covers/000/001/434/medium/3gatsu01.jpg?1359562488"
    );
});

test("import Dynasty Scans series without volumes", async (t) => {
    const proxy = testUtil.makeProxy("test.can-a-guy-like-me-be-a-maid.html");
    const url = "https://dynasty-scans.com/series/can_a_guy_like_me_be_a_maid";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-koshou-can-a-guy-like-me-be-a-maid");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.added, item.started);
    t.deepEqual(null, item.completed);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Can a Guy Like Me Be a Maid?"
    );
    t.deepEqual(item.entries.length, 15);
    t.deepEqual(item.entries[0].number, 1);
    t.deepEqual(item.entries[0].volume, null);
    t.deepEqual(item.entries[0].name.default, "English");
    t.deepEqual(item.entries[0].name.alternatives.English, "Chapter 0");
    t.deepEqual(item.entries[0].completed, null);

    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://dynasty-scans.com/system/tag_contents_covers/000/015/043/medium/[Hachimitsu_Scans]_Bokunare_c01_00.jpg?1604022009"
    );
});
