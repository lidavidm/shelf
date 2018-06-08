export { default as malbackup } from "./malbackup";
import mangadex from "./mangadex";

export function byURL(rawURL) {
    const url = new URL(rawURL);
    console.log(url.hostname);

    switch (url.hostname.toLowerCase()) {
    case "mangadex.org":
        return mangadex(rawURL);
    default:
        return Promise.resolve([]);
    }
}
