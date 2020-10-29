<script>
    import { onMount } from "svelte";
    import firstBy from "thenby";

    import * as util from "./util";

    export let router;

    let itemsByCategory = [];
    let people = {};
    let series = {};

    onMount(async function() {
        const [itemList, peopleList, seriesList] = await Promise.all([
            fetch("/item").then(r => r.json()),
            fetch("/person").then(r => r.json()),
            fetch("/series").then(r => r.json()),
        ]);
        // TODO: these can be stores
        people = buildMap(peopleList);
        series = buildMap(seriesList);
        itemsByCategory = sortItems(itemList);
    });

    function buildMap(items) {
        const result = {};
        for (const item of items) {
            result[item.key] = item.name.alternatives[item.name.default];
        }
        return result;
    }

    function getPersonName(person) {
        if (!people[person]) {
            return person;
        }
        return people[person];
    }

    function getSeriesName(key) {
        if (!series[key]) {
            return key;
        }
        return series[key];
    }

    function getMalUrl(kind, id) {
        if (kind === "Manga") {
            return `https://myanimelist.net/manga/${id}`;
        }
        return `https://myanimelist.net/anime/${id}`;
    }

    function sortItems(items) {
        items.sort(firstBy(v => v.status === "InProgress" ? 0 : 1)
            .thenBy(v => v.kind)
            .thenBy(v => v.status)
            .thenBy(v => v.name.alternatives[v.name.default].toLowerCase()));
        items = items.filter(v => !v.tags.includes("NSFW") && !v.tags.includes("Ecchi"));
        const itemsByCategory = [];
        const firstNotInProgress = items.findIndex(item => item.status !== "InProgress");
        itemsByCategory.push({
            "title": "In Progress",
            "items": items.slice(
                0,
                firstNotInProgress === -1 ? items.length : firstNotInProgress
            ),
        });
        let prevKind = null;
        for (let index = firstNotInProgress; index < items.length; index++) {
            const item = items[index];
            if (item.kind != prevKind) {
                prevKind = item.kind;
                itemsByCategory.push({
                    "title": util.humanKind(item.kind),
                    "items": [],
                });
            }
            itemsByCategory[itemsByCategory.length - 1].items.push(item);
        }
        return itemsByCategory;
    }
</script>

<main>
    {#each itemsByCategory as category (category.title)}
        <section>
            <header>
                <h2>{category.title}</h2>
            </header>
            <ul class="item-list">
                {#each category.items as item (item.key)}
                    <li>
                        <div class="cover">
                            {#if item.covers.length > 0}
                                <img
                                    src={'/blob/' + item.covers[0].key + '/contents'}
                                    alt={item.covers[0].description}
                                    title={item.covers[0].description}
                                />
                            {:else}
                                <img src="https://via.placeholder.com/320x240" alt="" />
                            {/if}
                        </div>

                        <div class="info">
                            <h3
                                title={item.name.alternatives[item.name.default]}
                                on:click="{e => router.show('/edit/' + item.key)}"
                            >
                                {item.name.alternatives[item.name.default]}
                            </h3>

                            {#if item.series}
                                <span>
                                    {getSeriesName(item.series[0])}
                                    {#if item.series[1]}
                                        ({item.series[1]})
                                    {/if}
                                </span>
                            {/if}

                            {#each item.people as person}
                                {#if person[0] === 'Author' || person[0] === 'Director'}
                                    <span>
                                        <em>by</em> {getPersonName(person[1])}
                                    </span>
                                {/if}
                            {/each}

                            <span>
                                Rating: {item.rating ? item.rating : "-"}/10
                            </span>

                            <div class="spacer"></div>

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
                                    <a href={item.extra.mangadex_url} target="_blank">
                                        <img src="mangadex.svg" alt="MangaDex" />
                                    </a>
                                {/if}
                                {#if item.extra && item.extra.mal_id}
                                    <a href={getMalUrl(item.kind, item.extra.mal_id)} target="_blank">
                                        <img src="assets/images/mal.ico" alt="MyAnimeList" />
                                    </a>
                                {/if}
                            </div>
                            <div class="item-bar">
                                <span>{item.kind}</span>
                                <span class={'item-status ' + item.status}>{item.status}</span>
                                <span>
                                    {item.entries.filter(e => e.completed).length}
                                    /
                                    {item.publication_status === "Complete" ? item.entries.length : "?"}
                                </span>
                            </div>
                        </div>
                    </li>
                {/each}
            </ul>
        </section>
    {/each}
</main>

<style>
    h3 {
        font-size: 1em;
        font-style: italic;
    }

    .item-list {
        display: flex;
        flex-wrap: wrap;
        list-style-type: none;
        margin: 0;
        padding: 0;
    }

    .item-list li {
        border-bottom: 1px solid #000;
        display: flex;
        width: 25em;
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
        border-bottom: 1px dotted #000;
        height: 2em;
        line-height: 2em;
        overflow: hidden;
        padding: 0 0.25em;
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
