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
                started: null,
                completed: null,
                extra: {
                    mangadex_url: url,
                },
            }];

            return imported;
        });
}
