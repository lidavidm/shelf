export default function mangadex(url) {
    return window.fetch(`/proxy?url=${url}`)
        .then(r => r.text())
        .then((body) => {
            const parser = new window.DOMParser();
            const doc = parser.parseFromString(body, "text/html");
            const page = doc.documentElement;

            const title = page.querySelector(".panel-title")
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
            const nameList = page.querySelector(".col-sm-9 > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2) > ul:nth-child(1)");
            let ctr = 1;
            for (const el of nameList.querySelectorAll("li")) {
                names.alternatives[`Alternate Name ${ctr}`] = el.textContent.trim();
                ctr++;
            }

            const imported = [{
                key,
                kind: "Manga",
                name: names,
                people: [],
                season: null,
                entries: [],
                status: "Planned",
                rating: null,
                added: new Date().toISOString(),
                started: null,
                completed: null,
                extra: {
                    mangadex_url: url,
                },
            }];

            return imported;
        });
}
