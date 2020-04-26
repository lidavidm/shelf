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
            const nameList = page.querySelectorAll(".card-body.p-0 > .row > div:nth-child(2) > div:nth-child(2) .list-inline-item");
            let ctr = 1;
            for (const el of nameList) {
                names.alternatives[`Alternate Name ${ctr}`] = el.textContent.trim();
                ctr++;
            }

            const entries = [];
            // This will have to wait -.-
            // const entryTable = page.querySelector(".table-striped > tbody:nth-child(2)");

            // for (const el of entryTable.querySelectorAll("tr")) {

            // }

            const template = window.fetch("/item/:template:")
                  .then(r => r.json());

            return template.then((template) => {
                template.key = key;
                template.kind = "Manga";
                template.name = names;
                template.added = moment().format();
                template.extra = {
                    mangadex_url: url,
                };
                return [template];
            });
        });
}
