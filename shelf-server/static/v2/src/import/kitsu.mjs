// Copyright 2020 David Li <li.davidm96@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

import { formatISO } from "date-fns";
import firstBy from "thenby";

import * as util from "./util.mjs";

const BASE_URL =
    "https://kitsu.io/api/edge/anime?fields[categories]=slug%2Ctitle&include=episodes&filter[slug]=";

export default async function kitsu(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    const itemId = extractId(rawUrl);
    const url = BASE_URL + itemId;
    const resp = await proxy(url);
    const data = JSON.parse(resp);
    if (!data.data || data.data.length == 0) {
        throw new Error(`Anime not found: ${itemId}`);
    } else if (data.data.length > 1) {
        throw new Error(`Multiple results found: ${itemId}`);
    }
    const anime = data.data[0];
    if (anime.type !== "anime") {
        throw new Error(`Unknown type: ${anime.type}`);
    }
    // Get rid of duplicate titles
    const [canonicalTitle, names] = extractTitles(
        anime.attributes.titles,
        anime.attributes.canonicalTitle
    );
    const coverUrl = anime.attributes.posterImage.original;
    const entries = data.included
        .filter((entry) => entry.type === "episodes")
        .map((entry) => {
            const volume = entry.attributes.seasonNumber;
            const number = entry.attributes.number;
            return {
                number,
                volume,
                name: extractTitles(
                    entry.attributes.titles,
                    entry.attributes.canonicalTitle
                )[1],
                completed: null,
                extra: null,
            };
        });
    entries.sort(firstBy((v) => v.volume).thenBy((v) => v.number));
    template.key = "tv-" + util.titleToKey(canonicalTitle);
    template.kind = "TV";
    template.name = names;
    template.added = formatISO(new Date());
    template.entries = entries;
    template.extra = {
        external_url: rawUrl,
    };
    template.status = "InProgress";
    template.publication_status = "Complete";
    template.synopsis = anime.attributes.synopsis;
    return {
        cover: coverUrl,
        item: template,
    };
}

export function extractId(rawUrl) {
    const url = new URL(rawUrl);
    if (url.host != "kitsu.io") {
        throw new Error(`Not a Kitsu URL: ${url} (host was ${url.host})`);
    }
    const match = url.pathname.match(/^\/anime\/(.+)$/);
    if (!match) {
        throw new Error(`Invalid URL: ${url}`);
    }
    return match[1];
}

export function extractTitles(rawTitles, canonicalTitle) {
    // Get rid of duplicate titles
    const titles = Object.entries(
        Object.fromEntries(
            Object.entries(rawTitles).map(([lang, title]) => {
                return [util.langCodeToName(lang), title];
            })
        )
    );
    const defaultTitleLanguage = titles.filter(
        (t) => t[1] === canonicalTitle
    )[0][0];
    return [
        canonicalTitle,
        {
            default: defaultTitleLanguage,
            alternatives: Object.fromEntries(titles),
        },
    ];
}
