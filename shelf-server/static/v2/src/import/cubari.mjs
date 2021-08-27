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
import * as util from "./util.mjs";

function getMeta(document, property) {
    for (const el of document.querySelectorAll("meta")) {
        if (el.getAttribute("property") === property) {
            return el.getAttribute("content");
        }
    }
    throw new Error(`Could not find property ${property}`);
}

export default async function cubari(
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
    if (url.pathname.startsWith("/read/")) {
        template.tags = ["Oneshot"];
        const title = getMeta(document, "og:title").split(/\|/g)[0].trim();
        template.key = `manga-cubari-${util.titleToKey(title)}`;
        template.name = {
            default: "English",
            alternatives: {
                English: title,
            },
        };
        template.entries = [
            {
                number: 1,
                volume: null,
                name: {
                    default: "English",
                    alternatives: {
                        English: "Oneshot",
                    },
                },
                completed: template.added,
            },
        ];
        template.completed = template.added;
        template.publication_status = "Complete";
        template.status = "Completed";
        return {
            cover: getMeta(document, "og:image").trim(),
            item: template,
        };
    }
    throw new Error(`Cannot import ${rawUrl}`);
}
