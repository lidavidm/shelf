export default function malbackup(s) {
    const parser = new DOMParser();
    const doc = parser.parseFromString(s, "application/xml");

    const list = doc.documentElement;
    const imported = [];

    for (const child of list.children) {
        if (child.nodeName === "anime" || child.nodeName === "manga") {
            const titleField = child.nodeName === "manga" ?
                  "manga_title" : "series_title";
            const episodesField = child.nodeName === "manga" ?
                  "manga_chapters" : "series_episodes";
            const watchedField = child.nodeName === "manga" ?
                  "my_read_chapters" : "my_watched_episodes";

            const title = child.querySelector(titleField).textContent.trim();
            let kind = "Unknown";
            if (child.nodeName === "manga") {
                kind = "Manga";
            }
            else {
                switch (child.querySelector("series_type").textContent.trim()) {
                case "TV":
                case "Special":
                    kind = "TV";
                    break;
                case "Movie":
                    kind = "Film";
                    break;
                case "OVA":
                    kind = "OVA";
                    break;
                case "ONA":
                    kind = "ONA";
                    break;
                case "Music":
                    kind = "Music";
                    break;
                }
            }
            const score = parseInt(child.querySelector("my_score").textContent.trim(), 10);

            let status = "Planned";
            switch (child.querySelector("my_status").textContent.trim()) {
            case "Plan to Watch":
            case "Plan to Read":
                status = "Planned";
                break;
            case "Completed":
                status = "Completed";
                break;
            case "Dropped":
                status = "Dropped";
                break;
            case "On-Hold":
                status = "OnHold";
                break;
            case "Watching":
            case "Reading":
                status = "InProgress";
                break;
            }

            const key = kind.toLowerCase() + "-" + title
                  .replace(/[\W:]+/g, "-")
                  .replace(/^-+/, "")
                  .replace(/-+$/, "")
                  .toLowerCase();

            const mal_id =
                  kind === "Manga" ?
                  child.querySelector("manga_mangadb_id").textContent.trim() :
                  child.querySelector("series_animedb_id").textContent.trim();

            const reconsumed =
                  kind === "Manga" ?
                  parseInt(child.querySelector("my_times_read").textContent.trim(), 10) :
                  parseInt(child.querySelector("my_times_watched").textContent.trim(), 10);

            const result = {
                key,
                kind,
                name: {
                    default: "Japanese (Romaji)",
                    alternatives: {
                        "Japanese (Romaji)": title,
                    },
                },
                people: [],
                season: null,
                entries: [],
                status,
                rating: (score === 0 || !Number.isFinite(score)) ? null : score,
                added: new Date().toISOString(),
                started: child.querySelector("my_start_date").textContent.trim(),
                completed: child.querySelector("my_finish_date").textContent.trim(),
                extra: {
                    reconsumed,
                    mal_id,
                },
            };

            const episodes = parseInt(child.querySelector(episodesField).textContent.trim(), 10);
            const watched = parseInt(child.querySelector(watchedField).textContent.trim(), 10);

            for (let i = 0; i < Math.max(episodes, watched); i++) {
                result.entries.push({
                    name: null,
                    number: i + 1,
                    volume: null,
                    completed: i < watched,
                });
            }

            imported.push(result);
        }
    }

    return imported;
}
