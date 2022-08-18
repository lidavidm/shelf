import test from "ava";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

import importTitle from "./mangadex.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = function (url) {
    let filename = null;
    if (url.includes("8280d386-8195-40da-8cbb-e8763ba3a93a")) {
        filename = "test.dora-yome.json";
    } else if (url.includes("3f1453fb-9dac-4aca-a2ea-69613856c952")) {
        filename = "test.tamen-de-gushi.json";
    } else {
        throw new Error(`Unknown file ${url}`);
    }
    const filepath = path.resolve(
        path.dirname(fileURLToPath(import.meta.url)),
        filename
    );
    return new Promise((resolve, reject) => {
        fs.readFile(filepath, "utf8", (err, data) => {
            if (err) {
                reject(err);
            } else {
                resolve(data);
            }
        });
    });
};

test("import MangaDex page", async (t) => {
    const url =
        "https://mangadex.org/title/8280d386-8195-40da-8cbb-e8763ba3a93a/dora-yome";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-korai-ayumu-dora-yome");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.name.alternatives[item.name.default], "Dora Yome");
    t.deepEqual(item.name.alternatives["Japanese"], "ドラよめ");
    t.deepEqual(item.name.alternatives["English 1"], "DoreYome");
    t.deepEqual(item.name.alternatives["English 2"], "Dragon Wife");
    t.deepEqual(
        item.synopsis,
        "A dragon's hormones are not a trifling thing. Siegfried rolled a 20 for a chance to defeat the dragon (who was hunting for a mate) and accidentally hit the dragon with {Charm}, next thing you know they are married."
    );
    t.deepEqual(item.tags, ["Comedy", "Fantasy", "Oneshot", "Romance"]);
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.mangadex_url, url);
    t.deepEqual(item.started, item.added);
    t.deepEqual(
        cover,
        "https://uploads.mangadex.org/covers/8280d386-8195-40da-8cbb-e8763ba3a93a/3ea14915-4f5a-499a-a94f-1e3078cc1a2c.jpg"
    );
});

test("import Tamen de Gushi", async (t) => {
    const url =
        "https://mangadex.org/title/3f1453fb-9dac-4aca-a2ea-69613856c952/tamen-de-gushi";
    const { cover, item } = await importTitle(url, {
        template: { entries: [] },
        proxy,
    });
    t.deepEqual(item.key, "manga-tan-jiu-tanjiu-tamen-de-gushi");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.name.alternatives[item.name.default], "Tamen de Gushi");
    t.deepEqual(item.name.alternatives["English 1"], "SQ: Begin W/Your Name!");
    t.deepEqual(
        item.synopsis,
        "The funny romantic story of how Qiu Tong and Sun Jing met and fell in love.\n\nAlso contains insert art of the characters by the author.\n\n---\n\n- [Original Manhua <AC.QQ>](https://ac.qq.com/Comic/comicInfo/id/630157)"
    );
    t.deepEqual(item.tags, [
        "Comedy",
        "Drama",
        "Romance",
        "School",
        "Slice of Life",
        "Yuri",
    ]);
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.mangadex_url, url);
    t.deepEqual(item.started, item.added);
    t.deepEqual(
        cover,
        "https://uploads.mangadex.org/covers/3f1453fb-9dac-4aca-a2ea-69613856c952/d8c446bf-6433-49f9-bd21-3b6c85669721.jpg"
    );
});
