export default function mangadex(url) {
    return window.fetch(`/proxy?url=${url}`)
        .then((r) => {
            console.log(r);
            return [];
        });
}
