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
let parse;
if (typeof global !== "undefined") {
    console.log("Using JSDOM");
    parse = (source) =>
        new jsdom.JSDOM(source, { contentType: "text/html" }).window.document;
} else {
    console.log("Using window.DOMParser");
    parse = (source) => {
        const parser = new window.DOMParser();
        const doc = parser.parseFromString(source, "text/html");
        return doc.documentElement;
    };
}

import * as util from "./util.mjs";

export default async function kobo(
    rawUrl,
    { template, proxy = util.defaultProxy }
) {
    const body = await proxy(rawUrl);
    const document = parse(body);

    const title = document
        .querySelector(".title.product-field")
        .textContent.trim();

    const key = "novel-" + util.titleToKey(title);

    const names = {
        default: "English",
        alternatives: {
            English: title,
        },
    };

    const entries = [
        {
            number: 1,
            volume: null,
            name: {
                default: "English",
                alternatives: {
                    English: title,
                },
            },
            completed: null,
            extra: null,
        },
    ];

    const metadata = document
        .querySelector(".bookitem-secondary-metadata")
        .textContent.trim();
    const isbnMatch = metadata.match(/ISBN: (\d+)/);
    const isbn = isbnMatch !== null ? isbnMatch[1] : null;

    template.key = key;
    template.kind = "Novel";
    template.name = names;
    template.added = formatISO(new Date());
    template.extra = {
        external_url: rawUrl,
        isbn,
    };
    template.entries = entries;
    template.status = "InProgress";
    let src = document.querySelector("img.cover-image").getAttribute("src");
    if (!src.startsWith("https") && src.startsWith("//")) {
        src = "https:" + src;
    }
    return {
        cover: src,
        item: template,
    };
}
