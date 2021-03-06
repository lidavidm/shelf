import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

import test from "ava";

import * as kitsu from "./kitsu.mjs";
import importTitle from "./kitsu.mjs";
import * as testUtil from "./test-util.mjs";

const proxy = testUtil.makeProxy("test.jitsu-wa-watashi-wa.json");

test("extract Kitsu IDs", (t) => {
    t.throws(
        () => {
            kitsu.extractId("http://google.com");
        },
        { message: /Not a Kitsu URL:/ }
    );
    t.throws(
        () => {
            kitsu.extractId("https://kitsu.io/nime/jitsu-wa-watashi-wa");
        },
        { message: /Invalid URL:/ }
    );
    t.deepEqual(
        kitsu.extractId("https://kitsu.io/anime/jitsu-wa-watashi-wa"),
        "jitsu-wa-watashi-wa"
    );
});

test("import Kitsu page", async (t) => {
    const url = "https://kitsu.io/anime/jitsu-wa-watashi-wa";
    const { cover, item } = await importTitle(url, {
        template: {},
        proxy,
    });
    t.deepEqual(item.key, "tv-jitsu-wa-watashi-wa");
    t.deepEqual(item.kind, "TV");
    t.deepEqual(item.name.default, "Japanese (Romaji)");
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Jitsu wa Watashi wa"
    );
    t.deepEqual(item.entries.length, 13);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://media.kitsu.io/anime/poster_images/10350/original.jpg?1597697462"
    );
});

test("import Kitsu page with blank episode titles", async (t) => {
    const url = "https://kitsu.io/anime/jitsu-wa-watashi-wa";
    const { cover, item } = await importTitle(url, {
        template: {},
        proxy: testUtil.makeProxy("test.rikeikoi.json"),
    });
    t.deepEqual(item.key, "tv-rikei-ga-koi-ni-ochita-no-de-shoumei-shitemita");
    t.deepEqual(item.kind, "TV");
    t.deepEqual(item.name.default, "Japanese (Romaji)");
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Rikei ga Koi ni Ochita no de Shoumei shitemita."
    );
    t.deepEqual(item.entries.length, 12);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://media.kitsu.io/anime/poster_images/42297/original.jpg?1597691461"
    );
    t.deepEqual(
        item.entries[0].name.alternatives[item.entries[0].name.default],
        "Season 1 Episode 1"
    );
});

test("import Kitsu movie", async (t) => {
    const url = "https://kitsu.io/anime/hanasaku-iroha-home-sweet-home";
    const { cover, item } = await importTitle(url, {
        template: {},
        proxy: testUtil.makeProxy("test.hanasaku-iroha-home-sweet-home.json"),
    });
    t.deepEqual(item.key, "film-hanasaku-iroha-home-sweet-home");
    t.deepEqual(item.kind, "Film");
    t.deepEqual(item.name.default, "Japanese (Romaji)");
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Hanasaku Iroha: Home Sweet Home"
    );
    t.deepEqual(item.entries.length, 1);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://media.kitsu.io/anime/poster_images/7090/original.jpg?1597697111"
    );
    t.deepEqual(
        item.entries[0].name.alternatives[item.entries[0].name.default],
        "Episode 1"
    );
});

test("import Kitsu OVA", async (t) => {
    const url = "https://kitsu.io/anime/kobayashi-san-chi-no-maid-dragon-ova";
    const { cover, item } = await importTitle(url, {
        template: {},
        proxy: testUtil.makeProxy(
            "test.kobayashi-san-chi-no-maid-dragon-ova.json"
        ),
    });
    t.deepEqual(
        item.key,
        "ova-kobayashisan-chi-no-maid-dragon-valentine-soshite-onsen-amari-kitai-shinaide-kudasai"
    );
    t.deepEqual(item.kind, "OVA");
    t.deepEqual(item.name.default, "Japanese (Romaji)");
    t.deepEqual(
        item.name.alternatives[item.name.default],
        "Kobayashi-san Chi no Maid Dragon: Valentine, Soshite Onsen! - Amari Kitai Shinaide Kudasai"
    );
    t.deepEqual(item.entries.length, 1);
    t.deepEqual(item.extra.external_url, url);
    t.deepEqual(
        cover,
        "https://media.kitsu.io/anime/poster_images/13861/original.jpg?1506380050"
    );
    t.deepEqual(
        item.entries[0].name.alternatives[item.entries[0].name.default],
        "Season 1 Episode 1"
    );
});
