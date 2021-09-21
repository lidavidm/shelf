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
    const body = await proxy(`https://api.mangadex.org/manga/${uuid}`);
    const document = JSON.parse(body);

    const rawTitles = document.data.attributes.title;
    const title = rawTitles[Object.keys(rawTitles)[0]];
    const names = {
        default: "English",
        alternatives: {
            English: title,
        },
    };
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

    let coverArt = null;
    for (const relationship of document.data.relationships) {
        if (relationship.type === "cover_art") {
            coverArt = relationship.id;
            break;
        }
    }
    if (coverArt === null) {
        throw new Error("Could not find cover art!");
    }

    const cover = await proxy(`https://api.mangadex.org/cover/${coverArt}`);
    const coverDocument = JSON.parse(cover);

    return {
        cover: `https://uploads.mangadex.org/covers/${uuid}/${coverDocument.data.attributes.fileName}`,
        item: template,
    };
}
