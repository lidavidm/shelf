import test from "ava";

import importTitle from "./mangadex.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.magical-lollipop.html");

test("import MangaDex page", async (t) => {
    const url = "https://mangadex.org/title/7771/magical-lollipop";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-magical-lollipop");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.name.alternatives[item.name.default], "Magical Lollipop");
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.mangadex_url, url);
    t.deepEqual(cover, "https://mangadex.org/images/manga/7771.png?1598323239");
});
