import moment from "moment";

import * as util from "../util";

export default function malurl(url) {
    return window.fetch(`/proxy?url=${url}`)
        .then(r => r.text())
        .then((body) => {
            const parser = new window.DOMParser();
            const doc = parser.parseFromString(body, "text/html");
            const page = doc.documentElement;

            const title = page.querySelector("h1 *[itemprop=name]")
                  .textContent
                  .trim();

            let kind = page.querySelectorAll(".js-scrollfix-bottom div > span + a")[0].textContent;
            switch (kind) {
            case "Movie":
                kind = "Film";
            }

            const key = util.makeKey(kind, title);
            const names = {
                default: "English",
                alternatives: {
                    "English": title,
                },
            };

            for (const node of page.querySelectorAll(".js-scrollfix-bottom .spaceit_pad")) {
                const text = node.textContent.trim();
                if (text.startsWith("Japanese:")) {
                    names.alternatives["Japanese"] = text.slice(9).trim();
                }
            }

            const imported = [{
                key,
                kind,
                name: names,
                people: [],
                season: null,
                entries: [],
                status: "Planned",
                rating: null,
                added: moment().format(),
                started: null,
                completed: null,
                extra: {
                    mal_id: page.querySelector("#myinfo_anime_id").value,
                },
            }];

            return imported;
        });
}
