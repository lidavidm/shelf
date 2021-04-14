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

    const url = new URL(rawUrl);
    url.hash = "";

    template.kind = "Manga";
    template.added = formatISO(new Date());
    template.started = template.added;
    template.extra = {
        external_url: url.toString(),
    };
    template.tags = ["Yuri"];
    if (url.pathname.startsWith("/series/")) {
        const title = document
            .querySelector("h2.tag-title b")
            .textContent.trim();
        const author = document
            .querySelector("h2.tag-title > a")
            .textContent.trim();
        const key = `manga-${util.titleToKey(author)}-${util.titleToKey(
            title
        )}`;
        const names = {
            default: "English",
            alternatives: {
                English: title,
            },
        };
        const entries = [];
        let volume = null;
        let number = 1;
        for (const entryEl of document.querySelectorAll(".chapter-list > *")) {
            if (entryEl.tagName === "DT") {
                volume = parseInt(
                    entryEl.textContent.trim().match(/Volume (\d)+/)[1],
                    10
                );
            } else if (entryEl.tagName === "DD") {
                entries.push({
                    number,
                    volume,
                    name: {
                        default: "English",
                        alternatives: {
                            English: entryEl
                                .querySelector("a")
                                .textContent.trim(),
                        },
                    },
                    completed: null,
                    extra: null,
                });
                number++;
            }
        }
        template.key = key;
        template.name = names;
        template.completed = null;
        template.publication_status = "Publishing";
        template.status = "InProgress";
        template.entries = entries;
        url.pathname = document.querySelector(".cover img").getAttribute("src");
        return {
            cover: url.toString().replace(/%3F/g, "?"),
            item: template,
        };
    } else if (url.pathname.startsWith("/chapters/")) {
        const title = document
            .querySelector("#chapter-title b")
            .textContent.trim();
        const author = document
            .querySelector("#chapter-title > a")
            .textContent.trim();
        const key = `manga-${util.titleToKey(author)}-${util.titleToKey(
            title
        )}`;
        const names = {
            default: "English",
            alternatives: {
                English: title,
            },
        };
        template.key = key;
        template.name = names;
        template.completed = template.added;
        template.publication_status = "Complete";
        template.status = "Completed";
        template.tags.push("Oneshot");
        template.tags.sort();
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
    throw new Error(`Cannot import ${rawUrl}`);
}
