import test from "ava";

import importTitle from "./nitter.mjs";
import * as testUtil from "./test-util.mjs";

test("import Twitter oneshot", async (t) => {
    const proxy = testUtil.makeProxy("test.zanka.html");
    const url =
        "https://unofficialbird.com/yuruyunaZNK/status/1567460472689131521#m";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(
        item.key,
        "manga-twitter-yuruyunaznk-chi-zukashigari-wu-na-meido-sanga-keai-kute-shifang-naio-niangyang-no-baihe"
    );
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "Japanese");
    t.deepEqual(item.added, item.started);
    t.deepEqual(item.completed, item.started);
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "恥ずかしがり屋なメイドさんが可愛くて仕方ないお嬢様の百合(*´-`) (@yuruyunaZNK)"
    );
    t.deepEqual(item.entries.length, 1);
    t.deepEqual(item.entries[0], {
        name: {
            default: "English",
            alternatives: {
                English: "Oneshot",
            },
        },
        number: 1,
        volume: null,
        completed: item.added,
        extra: null,
    });
    t.deepEqual(
        item.extra.external_url,
        "https://twitter.com/yuruyunaZNK/status/1567460472689131521"
    );
    t.deepEqual(
        cover,
        "https://unofficialbird.com/pic/media%2FFcC8-FJagAIIae3.jpg"
    );
});
