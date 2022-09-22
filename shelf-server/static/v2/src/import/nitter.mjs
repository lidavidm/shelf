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

export default async function twitter(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    // Rewrite url to grab from a nitter instance instead
    // retry a few times if necessary
    const url = new URL(rawUrl);
    if (url.host != "farside.link") {
        rawUrl = `https://farside.link/nitter${url.pathname}`;
    }
    const realUrl = new URL(`https://twitter.com${url.pathname}`);

    const body = await proxy(rawUrl);
    const document = util.parse(body);

    template.kind = "Manga";
    template.added = formatISO(new Date());
    template.started = template.added;
    template.extra = {
        external_url: realUrl.toString(),
    };

    const siteprefix = "twitter";
    const title = getMeta(document, "og:description").trim();
    const author = realUrl.pathname.split("/")[1];
    const titleAndAuthor = "${rawTitle}（${author}）";
    // TODO: it'd be nice to associate accounts with authors
    template.key = `manga-${siteprefix}-${util.titleToKey(
        author
    )}-${util.titleToKey(title)}`;
    template.name = {
        default: "Japanese",
        alternatives: {
            Japanese: `${title} (@${author})`,
        },
    };
    template.entries = [
        {
            name: {
                default: "English",
                alternatives: {
                    English: "Oneshot",
                },
            },
            number: 1,
            volume: null,
            completed: template.added,
            extra: null,
        },
    ];
    template.completed = template.added;
    template.publication_status = "Complete";
    template.status = "Completed";
    template.tags = ["Oneshot"];
    return {
        cover: getMeta(document, "og:image").trim(),
        item: template,
    };
}
