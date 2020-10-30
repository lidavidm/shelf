import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

import test from "ava";

import * as kitsu from "./kitsu.mjs";
import importTitle from "./kitsu.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.jitsu-wa-watashi-wa.json");

test("extract Kitsu IDs", t => {
    t.throws(() => {
        kitsu.extractId("http://google.com");
    }, { message: /Not a Kitsu URL:/ });
    t.throws(() => {
        kitsu.extractId("https://kitsu.io/nime/jitsu-wa-watashi-wa");
    }, { message: /Invalid URL:/ });
    t.deepEqual(kitsu.extractId("https://kitsu.io/anime/jitsu-wa-watashi-wa"), "jitsu-wa-watashi-wa");
});

test("import Kitsu page", async t => {
    const url = "https://kitsu.io/anime/jitsu-wa-watashi-wa";
    const { cover, item } = await importTitle(url, {
        template: {},
        proxy,
    });
    t.deepEqual(item.key, "tv-jitsu-wa-watashi-wa");
    t.deepEqual(item.kind, "TV");
    t.deepEqual(item.name.default, "Japanese (Romaji)");
    t.deepEqual(item.name.alternatives[item.name.default], "Jitsu wa Watashi wa");
    t.deepEqual(item.entries.length, 13);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(cover, "https://media.kitsu.io/anime/poster_images/10350/original.jpg?1597697462");
});
