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

export default async function dynastyScans(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    const body = await proxy(rawUrl);
    const document = util.parse(body);

    // TODO: also implement a path to import a series

    const title = document.querySelector("#chapter-title b").textContent.trim();
    const author = document
        .querySelector("#chapter-title > a")
        .textContent.trim();

    const key = `manga-${util.titleToKey(author)}-${util.titleToKey(title)}`;

    const names = {
        default: "English",
        alternatives: {
            English: title,
        },
    };

    const entries = [];
    template.key = key;
    template.kind = "Manga";
    template.name = names;
    template.added = formatISO(new Date());
    template.started = template.added;
    template.completed = template.added;
    template.publication_status = "Complete";
    const url = new URL(rawUrl);
    url.hash = "";
    template.extra = {
        external_url: url.toString(),
    };
    template.status = "Completed";
    template.tags = ["Oneshot", "Yuri"];
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
            extra: null,
        },
    ];
    url.pathname = document.querySelector("#image img").getAttribute("src");
    return {
        cover: url.toString(),
        item: template,
    };
}
