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
    const body = await proxy(rawUrl);
    const document = util.parse(body);

    const title = document.querySelector("h6.card-header").textContent.trim();

    const key = "manga-" + util.titleToKey(title);

    const names = {
        default: "English",
        alternatives: {
            English: title,
        },
    };
    const nameList = document.querySelectorAll(
        ".card-body.p-0 > .row > div:nth-child(2) > div:nth-child(2) .list-inline-item"
    );
    let ctr = 1;
    for (const el of nameList) {
        names.alternatives[`Alternate Name ${ctr}`] = el.textContent.trim();
        ctr++;
    }

    const entries = [];
    template.key = key;
    template.kind = "Manga";
    template.name = names;
    template.added = formatISO(new Date());
    template.extra = {
        mangadex_url: rawUrl,
    };
    template.status = "InProgress";
    return {
        cover: document.querySelector("img.rounded").getAttribute("src"),
        item: template,
    };
}
