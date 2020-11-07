import test from "ava";

import importTitle from "./kobo.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.orsinian-tales.html");

test("import MangaDex page", async (t) => {
    const url = "https://www.kobo.com/us/en/ebook/orsinian-tales";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "novel-orsinian-tales");
    t.deepEqual(item.kind, "Novel");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.name.alternatives[item.name.default], "Orsinian Tales");
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(item.extra.isbn, "9781598534962");
    t.deepEqual(
        cover,
        "https://kbimages1-a.akamaihd.net/d410419c-63cb-4df2-9d9c-85e38eb556b4/353/569/90/False/orsinian-tales.jpg"
    );
});
