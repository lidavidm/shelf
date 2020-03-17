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

import moment from "moment";

export default function mangadex(url) {
    return window.fetch(`/proxy?url=${url}`)
        .then(r => r.text())
        .then((body) => {
            const parser = new window.DOMParser();
            const doc = parser.parseFromString(body, "text/html");
            const page = doc.documentElement;

            const title = page.querySelector("h6.card-header")
                  .textContent
                  .trim();

            const key = "manga-" + (
                title
                    .replace(/[\W:]+/g, "-")
                    .replace(/^-+/, "")
                    .replace(/-+$/, "")
                    .toLowerCase());

            const names = {
                default: "English",
                alternatives: {
                    "English": title,
                },
            };
            const nameList = page.querySelector("div.m-0:nth-child(1) > div:nth-child(2) > ul:nth-child(1)");
            let ctr = 1;
            for (const el of nameList.querySelectorAll("li")) {
                names.alternatives[`Alternate Name ${ctr}`] = el.textContent.trim();
                ctr++;
            }

            const entries = [];
            // This will have to wait -.-
            // const entryTable = page.querySelector(".table-striped > tbody:nth-child(2)");

            // for (const el of entryTable.querySelectorAll("tr")) {

            // }

            const imported = [{
                key,
                kind: "Manga",
                name: names,
                people: [],
                season: null,
                entries,
                status: "Planned",
                rating: null,
                added: moment().format(),
                tags: [],
                started: null,
                completed: null,
                publication_status: "Publishing",
                extra: {
                    mangadex_url: url,
                },
            }];

            return imported;
        });
}
