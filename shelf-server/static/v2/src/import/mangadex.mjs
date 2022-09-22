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
import * as jsdom from "jsdom";
import * as util from "./util.mjs";

export default async function mangadex(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    const url = new URL(rawUrl);
    const uuid = url.pathname.match(/\/(?:manga|title)\/([a-zA-Z0-9-]+)/)[1];
    const body = await proxy(
        `https://api.mangadex.org/manga/${uuid}?includes[]=author&includes[]=cover_art`
    );
    const document = JSON.parse(body);

    const rawTitles = document.data.attributes.title;
    const title = rawTitles[Object.keys(rawTitles)[0]];
    const names = {
        default: "English",
        alternatives: {
            English: title,
        },
    };

    let counter = 1;
    for (const title of document.data.attributes.altTitles) {
        const langCode = Object.keys(title)[0];
        let langName = langCode;
        switch (langCode) {
            case "en":
                langName = "English";
                break;
            case "ja":
                langName = "Japanese";
                break;
            case "ja-ro":
                langName = "Japanese (Romaji)";
                break;
            case "zh":
                langName = "Chinese";
                break;
            case "zh-hk":
                langName = "Chinese (Traditional)";
                break;
            case "zh-ro":
                langName = "Chinese (Pinyin)";
                break;
            case "es":
            case "es-la":
            case "fr":
            case "he":
            case "id":
            case "ne":
            case "ru":
            case "vi":
                continue;
            default:
                break;
        }

        if (names.alternatives[langName]) {
            langName = `${langName} ${counter++}`;
        }
        names.alternatives[langName] = title[langCode];
    }

    const key = "manga-" + util.titleToKey(title);

    const entries = [];
    template.key = key;
    template.kind = "Manga";
    template.name = names;
    template.added = formatISO(new Date());
    template.started = formatISO(new Date());
    template.extra = {
        mangadex_url: rawUrl,
    };
    template.status = "InProgress";
    template.publication_status =
        document.data.attributes.status === "completed"
            ? "Complete"
            : "Publishing";
    template.tags = [];

    for (const tag of document.data.attributes.tags) {
        if (!tag.attributes.name || !tag.attributes.name.en) continue;
        const name = tag.attributes.name.en;
        if (
            name === "Anthology" ||
            name === "Comedy" ||
            name === "Drama" ||
            name === "Fantasy" ||
            name === "Oneshot" ||
            name === "Romance" ||
            name === "Slice of Life"
        ) {
            template.tags.push(name);
        } else if (name === "Girls' Love") {
            template.tags.push("Yuri");
        } else if (name === "School Life") {
            template.tags.push("School");
        }
    }
    switch (document.data.attributes.publicationDemographic) {
        case "Josei":
            template.tags.push("Josei");
        default:
            break;
    }
    template.tags.sort();

    if (document.data.attributes.description.en) {
        template.synopsis = document.data.attributes.description.en;
    }

    let coverArt = null;
    let author = null;
    for (const relationship of document.data.relationships) {
        if (relationship.type === "cover_art") {
            coverArt = `https://uploads.mangadex.org/covers/${uuid}/${relationship.attributes.fileName}`;
        } else if (relationship.type === "author") {
            author = relationship.attributes.name;
        }
    }
    if (coverArt === null) {
        throw new Error("Could not find cover art!");
    }
    if (author !== null) {
        template.key = `manga-${util.titleToKey(author)}-${util.titleToKey(
            title
        )}`;
    }

    return {
        cover: coverArt,
        item: template,
    };
}
