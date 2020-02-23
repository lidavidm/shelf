export function makeKey(kind, name) {
    return kind + "-" + (name
                         .replace(/[\W:]+/g, "-")
                         .replace(/^-+/, "")
                         .replace(/-+$/, "")
                         .toLowerCase());
}

/** Map an item kind to a user-friendly string. */
export function humanKind(kind) {
    switch (kind) {
    case "OVA":
        return "OVA (Original Video Animation)";
    case "ONA":
        return "ONA (Original Net Animation)";
    case "ShortStory":
        return "Short Story";
    case "VisualNovel":
        return "Visual Novel";
    case "Music":
        return "Music Video";
    default:
        return kind;
    }
}
