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
export function addNextEntry(item, count = 1) {
    let volume = null;
    let number = 1;
    if (item.entries.length !== 0) {
        const lastCompleted = item.entries[item.entries.length - 1];
        volume = lastCompleted.volume;
        number = lastCompleted.number + 1;
    }

    const entries = [];
    for (let i = 0; i < count; i++) {
        const entry = {
            name: {
                default: "English",
                alternatives: {
                    English: `${entryName(item.kind)} ${number}`,
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
