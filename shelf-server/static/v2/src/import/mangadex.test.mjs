import test from "ava";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

import importTitle from "./mangadex.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = function (url) {
    const filename = url.includes("/cover")
        ? "test.dora-yome.cover.json"
        : "test.dora-yome.json";
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
    t.deepEqual(item.key, "manga-dora-yome");
    t.deepEqual(item.kind, "Manga");
    t.deepEqual(item.name.default, "English");
    t.deepEqual(item.name.alternatives[item.name.default], "Dora Yome");
    t.deepEqual(item.entries.length, 0);
    t.deepEqual(item.extra.mangadex_url, url);
    t.deepEqual(item.started, item.added);
    t.deepEqual(
        cover,
        "https://uploads.mangadex.org/covers/8280d386-8195-40da-8cbb-e8763ba3a93a/3ea14915-4f5a-499a-a94f-1e3078cc1a2c.jpg"
    );
});
