export { default as malbackup } from "./malbackup";
import malurl from "./malurl";
import mangadex from "./mangadex";

export function byURL(rawURL) {
    const url = new URL(rawURL);

    switch (url.hostname.toLowerCase()) {
    case "mangadex.org":
        return mangadex(rawURL);
    case "myanimelist.net":
        return malurl(rawURL);
    default:
        return Promise.resolve([]);
    }
}
