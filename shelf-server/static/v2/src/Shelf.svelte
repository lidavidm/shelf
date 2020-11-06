<script>
    import { onMount } from "svelte";
    import firstBy from "thenby";

    import * as itemEdit from "./item-edit.mjs";
    import importKitsu from "./import/kitsu.mjs";
    import importMangadex from "./import/mangadex.mjs";
    import items from "./stores/items.js";
    import people from "./stores/people.js";
    import series from "./stores/series.js";
    import TitleBar from "./component/TitleBar.svelte";
    import toastStore from "./component/toast.js";
    import * as util from "./util";

    export let router;

    let displayed = { "In Progress": true };
    let tagFilters = { sfw: true };
    $: itemsByCategory = sortItems(Object.values($items), tagFilters);
    let urlToImport = "";

    function reload() {
        people.update();
        series.update();
        items.update();
    }
    onMount(reload);

    function getPersonName(person) {
        if (!$people[person]) {
            return person;
        }
        return $people[person];
    }

    function getSeriesName(key) {
        if (!$series[key]) {
            return key;
        }
        return $series[key];
    }

    function getMalUrl(kind, id) {
        if (kind === "Manga") {
            return `https://myanimelist.net/manga/${id}`;
        }
        return `https://myanimelist.net/anime/${id}`;
    }

    function sortItems(items, tagFilters) {
        items.sort(
            firstBy((v) => (v.status === "InProgress" ? 0 : 1))
                .thenBy((v) => v.kind)
                .thenBy((v) => v.status)
                .thenBy((v) =>
                    v.name.alternatives[v.name.default].toLowerCase()
                )
        );
        if (tagFilters["sfw"]) {
            items = items.filter(
                (v) => !v.tags.includes("NSFW") && !v.tags.includes("Ecchi")
            );
        }
        const itemsByCategory = [];
        const firstNotInProgress = items.findIndex(
            (item) => item.status !== "InProgress"
        );
        itemsByCategory.push({
            title: "In Progress",
            items: items.slice(
                0,
                firstNotInProgress === -1 ? items.length : firstNotInProgress
            ),
        });
        if (firstNotInProgress >= 0) {
            let prevKind = null;
            for (
                let index = firstNotInProgress;
                index < items.length;
                index++
            ) {
                const item = items[index];
                if (item.kind != prevKind) {
                    prevKind = item.kind;
                    const title = util.humanKind(item.kind);
                    itemsByCategory.push({
                        title,
                        items: [],
                    });
                    if (typeof displayed[title] === "undefined") {
                        displayed = { ...displayed, [title]: false };
                    }
                }
                itemsByCategory[itemsByCategory.length - 1].items.push(item);
            }
        }
        return itemsByCategory;
    }

    async function importUrl() {
        const url = new URL(urlToImport);
        let importer;
        switch (url.hostname) {
            case "kitsu.io":
                importer = importKitsu;
                break;
            case "mangadex.org":
                importer = importMangadex;
                break;
            default:
                alert(
                    `Unknown source: ${urlToImport} (hostname ${url.hostname})`
                );
                return;
        }

        const { cover, item } = await window
            .fetch("/item/:template:")
            .then((r) => r.json())
            .then((template) => importer(urlToImport, { template }));
        const coverRequest = await window.fetch(
            "/proxy?url=" + encodeURIComponent(cover)
        );
        const coverBlob = await coverRequest.blob();
        const formData = new FormData();
        const blobKey = `blob-${item.key}-cover`;
        formData.append(blobKey, coverBlob);

        const blobUpload = await window.fetch("/blob", {
            method: "PUT",
            body: formData,
        });
        const blobResult = await blobUpload.json();
        console.log(blobResult);

        item.covers = [{ key: blobKey, description: "Cover" }];
        const itemBody = JSON.stringify(item, null, 2);
        console.log(itemBody);
        const itemUpload = await window.fetch(
            "/item/" + encodeURIComponent(item.key),
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: itemBody,
            }
        );
        console.log(await itemUpload.json());
        toastStore.push({
            title: "Created Item.",
            body: item.name.alternatives[item.name.default],
        });
        reload();
    }

    /** Complete the next entry of an item */
    async function completeNext(key) {
        const item = $items[key];
        const [newItem, newEntry] = itemEdit.completeNextEntry(item);
        try {
            await items.patch(newItem);
        } catch (error) {
            toastStore.push({
                title: "Error.",
                body: error,
            });
            return;
        }
        toastStore.push({
            title: "Updated Entry.",
            body:
                "Completed " +
                newEntry.name.alternatives[newEntry.name.default],
        });
    }
</script>

<style>
    h3 {
        font-size: 1em;
        font-style: italic;
    }

    #filters > div {
        display: flex;
        flex-wrap: wrap;
    }

    .filter-group {
        border: 1px solid #ccc;
        border-radius: 5px;
        box-sizing: border-box;
        flex: 0 0;
        min-width: 40em;
        padding: 1em;
        position: relative;
        width: 33%;
    }

    .filter-group h3 {
        position: absolute;
        top: -0.75em;
        background: #fff;
    }

    .filter-group > .filter-group-inner {
        display: flex;
        flex-wrap: wrap;
    }

    .filter-group > .filter-group-inner > div {
        flex: 0 0 8em;
    }

    .item-list {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        list-style-type: none;
        margin: 0;
        padding: 0;
    }

    .item-list li {
        border-bottom: 1px solid #000;
        display: flex;
        width: 30em;
    }

    .item-list li > .cover {
        display: flex;
        flex: 0 0 40%;
        flex-direction: column;
        font-size: 0;
        justify-content: center;
        width: 40%;
    }

    .item-list li > .cover img {
        flex: 0 0;
        max-width: 100%;
    }

    .item-list li > .info {
        display: flex;
        flex: 1 0;
        flex-direction: column;
        min-width: 0; /* https://css-tricks.com/flexbox-truncated-text/ */
    }

    .item-list li .info > h3 {
        cursor: pointer;
        height: 2em;
        line-height: 2em;
        overflow: hidden;
        padding: 0 0.25em;
        text-align: center;
        text-overflow: ellipsis;
        white-space: nowrap;
        width: calc(100% - 0.5em);
    }

    .item-list li .info > *:not(h3) {
        font-size: 0.85em;
        padding: 0.25em;
    }

    .item-list li .info > .item-bar {
        border-top: 1px dotted #000;
    }

    .item-bar button {
        background: none;
        border: 0;
        cursor: pointer;
        margin: 0;
        padding: 0;
    }

    .item-bar button.complete-next {
        color: #000;
        transition: color 50ms ease-in-out;
    }

    .item-bar button.complete-next:hover {
        color: var(--theme-completed);
    }

    .item-bar button.complete-next span {
        font-size: 1.1em;
        vertical-align: middle;
    }

    .item-status.Completed {
        color: var(--theme-completed);
    }

    .item-status.Dropped {
        color: var(--theme-dropped);
    }

    .item-status.OnHold {
        color: var(--theme-onhold);
    }

    .item-status.Planned {
        color: var(--theme-planned);
    }

    .item-status.InProgress {
        color: var(--theme-inprogress);
    }

    .spacer {
        flex: 1 1;
    }

    .external-links img {
        height: 1em;
    }

    .tags span:not(:last-child):not(:first-child):after {
        content: ", ";
    }
</style>

<main>
    <TitleBar>Library</TitleBar>
    <section id="import">
        <div>
            <label for="import">Import URL:</label>
            <input id="import" type="text" bind:value={urlToImport} />
            <button on:click={importUrl}>Import</button>
        </div>
    </section>
    <section id="filters">
        <h2>Filters</h2>
        <div>
            <section class="filter-group">
                <h3>Categories</h3>
                <div class="filter-group-inner">
                    {#each Object.keys(displayed) as title (title)}
                        <div>
                            <input
                                id={'filter-' + title}
                                type="checkbox"
                                bind:checked={displayed[title]} />
                            <label for={'filter-' + title}>{title}</label>
                        </div>
                    {/each}
                </div>
            </section>
            <section class="filter-group">
                <h3>Tags</h3>
                <div class="filter-group-inner">
                    <div>
                        <input
                            id="filter-tag-sfw"
                            type="checkbox"
                            bind:checked={tagFilters['sfw']} />
                        <label for="filter-tag-sfw">SFW Only</label>
                    </div>
                </div>
            </section>
        </div>
    </section>
    {#each itemsByCategory.filter((cat) => displayed[cat.title]) as category (category.title)}
        <section>
            <header>
                <h2>{category.title} ({category.items.length} items)</h2>
            </header>
            <ul class="item-list">
                {#each category.items as item (item.key)}
                    <li>
                        <div class="cover">
                            {#if item.covers.length > 0}
                                <img
                                    src={'/blob/' + item.covers[0].key + '/contents'}
                                    alt={item.covers[0].description}
                                    title={item.covers[0].description} />
                            {:else}
                                <img
                                    src="https://via.placeholder.com/320x240"
                                    alt="" />
                            {/if}
                        </div>

                        <div class="info">
                            <h3
                                title={item.name.alternatives[item.name.default]}
                                on:click={(e) => router.show('/edit/' + item.key)}>
                                {item.name.alternatives[item.name.default]}
                            </h3>

                            {#if item.series}
                                <span>
                                    {getSeriesName(item.series[0])}
                                    {#if item.series[1]}({item.series[1]}){/if}
                                </span>
                            {/if}

                            {#each item.people as person}
                                {#if person[0] === 'Author' || person[0] === 'Director'}
                                    <span>
                                        <em>by</em>
                                        {getPersonName(person[1])}
                                    </span>
                                {/if}
                            {/each}

                            <span>
                                Rating:
                                {item.rating ? item.rating : '-'}/10
                            </span>

                            {#if item.completed}
                                <span>
                                    Completed:
                                    {#if item.completed.indexOf('T') > 0}
                                        {new Date(item.completed).toLocaleDateString('en-GB')}
                                    {:else}{item.completed}{/if}
                                </span>
                            {/if}

                            <div class="spacer" />

                            {#if item.tags && item.tags.length > 0}
                                <div class="item-bar tags">
                                    <span>Tags:</span>
                                    {#each item.tags as tag}
                                        <span>{tag}</span>
                                    {/each}
                                </div>
                            {/if}
                            <div class="item-bar external-links">
                                <span>External Links:</span>
                                {#if item.extra && item.extra.mangadex_url}
                                    <a
                                        href={item.extra.mangadex_url}
                                        target="_blank">
                                        <img
                                            src="mangadex.svg"
                                            alt="MangaDex" />
                                    </a>
                                {/if}
                                {#if item.extra && item.extra.mal_id}
                                    <a
                                        href={getMalUrl(item.kind, item.extra.mal_id)}
                                        target="_blank">
                                        <img
                                            src="assets/images/mal.ico"
                                            alt="MyAnimeList" />
                                    </a>
                                {/if}
                            </div>
                            <div class="item-bar">
                                <span><strong>{item.kind}</strong></span>
                                <span
                                    class={'item-status ' + item.status}>{item.status}</span>
                                <span>
                                    {item.entries.filter((e) => e.completed).length}
                                    /
                                    {item.publication_status === 'Complete' ? item.entries.length : '?'}
                                </span>
                                {#if item.status !== 'Completed'}
                                    <button
                                        aria-label="Complete Next Entry"
                                        class="complete-next"
                                        title="Complete Next Entry"
                                        on:click={completeNext(item.key)}>
                                        <span
                                            class="material-icons"
                                            aria-hidden="true">
                                            add_circle_outline
                                        </span>
                                    </button>
                                {/if}
                            </div>
                        </div>
                    </li>
                {/each}
            </ul>
        </section>
    {/each}
</main>
