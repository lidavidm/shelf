import { formatISO } from "date-fns";

/**
 * Complete the next entry of an item, adding a new entry if appropriate.
 */
export function completeNextEntry(item) {
    const completed = formatISO(new Date());
    // Complete the first uncompleted item; assumes entries are sorted
    for (const entry of item.entries) {
        if (!entry.completed) {
            // Easy: just complete this entry
            entry.completed = completed;
            return [item, entry];
        }
    }

    let [_, [entry]] = addNextEntry(item, 1);
    entry.completed = completed;
    return [item, entry];
}

/** Add the next entry of an item. */
export function addNextEntry(
    item,
    count = 1,
    givenVolume = null,
    options = null
) {
    const entries = [];
    const restartNumbering = options && options.restartNumbering;
    for (let i = 0; i < count; i++) {
        let volume = givenVolume;
        let number = 1;
        let userNumber = 1;
        let namePrefix = entryName(item.kind);
        if (item.entries.length !== 0) {
            const lastCompleted = item.entries[item.entries.length - 1];
            if (volume == null) {
                volume = lastCompleted.volume;
            }
            number = lastCompleted.number + 1;
            userNumber = number;
            if (restartNumbering && volume != lastCompleted.volume) {
                userNumber = 1;
            } else if (lastCompleted.name) {
                // Infer actual number from chapter title
                const name =
                    lastCompleted.name.alternatives[lastCompleted.name.default];
                const match = name.match(
                    /^(Volume|Chapter|Episode)\s+([0-9]+(?:\.[0-9]+)?)/
                );
                if (match) {
                    namePrefix = match[1];
                    userNumber = Math.floor(Number.parseFloat(match[2])) + 1;
                }
                if (namePrefix === "Volume") {
                    volume = userNumber;
                }
            }
        }

        const entry = {
            name: {
                default: "English",
                alternatives: {
                    English: `${namePrefix} ${userNumber}`,
                },
            },
            number,
            volume,
            completed: false,
        };
        item.entries.push(entry);
        entries.push(entry);
        number++;
    }
    return [item, entries];
}

export function entryName(kind, plural = false) {
    switch (kind) {
        case "Music":
        case "ONA":
        case "OVA":
        case "TV":
            return plural ? "Episodes" : "Episode";
        case "Novel":
            return plural ? "Volumes" : "Volume";
        case "Collection":
        case "Manga":
        case "NonFiction":
        case "ShortStory":
            return plural ? "Chapters" : "Chapter";
        default:
            return plural ? "Entries" : "Entry";
    }
}
