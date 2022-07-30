import test from "ava";

import importTitle from "./sundaywebry.mjs";
import * as testUtil from "./test-util.mjs";

test("import Sunday Web Every oneshot", async (t) => {
    const proxy = testUtil.makeProxy("test.キミの手には.html");
    const url = "https://www.sunday-webry.com/episode/3269754496589975683";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(
        item.key,
        "manga-sundaywebry-kimi-no-shou-niha-xiaoxingzhuling"
    );
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "Japanese");
    t.deepEqual(item.added, item.started);
    t.deepEqual(item.completed, null);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "キミの手には（小形朱嶺）"
    );
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(cover, "https://cdn-img.www.sunday-webry.com/public/series-thumbnail/3269754496589963096-7acf2c99cf3f635a0d95464c4a9aeba4?1636339792");
});
