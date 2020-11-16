import test from "ava";

import importTitle from "./webtoons.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.dating-with-a-tail.html");

test("import Webtoons page", async (t) => {
    const url =
        "https://www.webtoons.com/en/romance/dating-with-a-tail/list?title_no=1263";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-dating-with-a-tail");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Dating with a Tail"
    );
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.external_url, url);
});
