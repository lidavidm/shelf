export function makeKey(kind, name) {
    return kind + "-" + (name
                         .replace(/[\W:]+/g, "-")
                         .replace(/^-+/, "")
                         .replace(/-+$/, "")
                         .toLowerCase());
}
