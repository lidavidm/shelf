// Copyright 2022 David Li <li.davidm96@gmail.com>
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
import * as util from "./util.mjs";

function getMeta(document, property) {
    for (const el of document.querySelectorAll("meta")) {
        if (el.getAttribute("property") === property) {
            return el.getAttribute("content");
        }
    }
    throw new Error(`Could not find property ${property}`);
}

export default async function sundayWebry(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    const body = await proxy(rawUrl);
    const document = util.parse(body);

    const url = new URL(rawUrl);

    template.kind = "Manga";
    template.added = formatISO(new Date());
    template.started = template.added;
    template.extra = {
        external_url: url.toString(),
    };

    let siteprefix = "sundaywebry";
    const rawTitle = getMeta(document, "og:title").split(/ \| /g)[0].trim();
    let titleAndAuthor = "";
    if (url.hostname === "www.sunday-webry.com") {
        titleAndAuthor = rawTitle.split(" / ", 2)[1];
    } else {
        titleAndAuthor = rawTitle;
        siteprefix = url.hostname.toLowerCase().replace(/[^a-z0-9]+/g, "");
    }
    const splitPoint = titleAndAuthor.lastIndexOf(" - ");
    const title = titleAndAuthor.substr(0, splitPoint);
    const author = titleAndAuthor.substr(splitPoint + 3);
    template.key = `manga-${siteprefix}-${util.titleToKey(titleAndAuthor)}`;
    template.name = {
        default: "Japanese",
        alternatives: {
            Japanese: `${title}（${author}）`,
        },
    };
    template.entries = [];
    template.completed = null;
    template.publication_status = "Publishing";
    template.status = "InProgress";
    return {
        cover: getMeta(document, "og:image").trim(),
        item: template,
    };
}
