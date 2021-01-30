<script>
    import EditAlternatives from "./component/EditAlternatives.svelte";
    import NullableMultiDate from "./component/NullableMultiDate.svelte";
    import TagList from "./component/TagList.svelte";
    import TitleBar from "./component/TitleBar.svelte";
    import toastStore from "./component/toast.js";
    import * as importUtil from "./import/util.mjs";
    import * as itemEdit from "./item-edit.mjs";
    import items from "./stores/items.js";
    import people from "./stores/people.js";
    import series from "./stores/series.js";
    import * as util from "./util";

    export let router;
    export let params;

    let urlToImport;
    let urlToImportDescription;

    let item = null;
    let isNewItem = !params.key || params.key === ":template:";
    let loading = fetch(params.key ? "/item/" + params.key : "/item/:template:")
        .then((r) => r.json())
        .then((value) => (item = value));

    async function save() {
        try {
            await items.patch(item);
        } catch (error) {
            toastStore.push({
                title: "Error.",
                body: error,
            });
            return;
        }
        toastStore.push({
            title: "Updated Entry.",
            body: item.name.alternatives[item.name.default],
        });
    }

    async function addMultipleEntries() {
        const count = parseInt(window.prompt("Number of entries to add?"), 10);
        const [newItem] = itemEdit.addNextEntry(item, count);
        item = newItem;
    }

    async function completeMultipleEntries() {
        const count = parseInt(
            window.prompt("Number of entries to complete?"),
            10
        );
        let newItem = item;
        for (let i = 0; i < count; i++) {
            [newItem] = itemEdit.completeNextEntry(item);
        }
        item = newItem;
    }

    async function importUrl() {
        if (!urlToImport) {
            toastStore.push({
                title: "Error.",
                body: "Must provide URL",
            });
            return;
        }
        if (!urlToImportDescription) {
            toastStore.push({
                title: "Error.",
                body: "Must provide description",
            });
            return;
        }
        const description = urlToImportDescription.trim();
        const coverRequest = await importUtil.proxy(urlToImport);
        const coverBlob = await coverRequest.blob();
        const blobKey = `blob-${item.key}-${importUtil.titleToKey(
            description
        )}`;

        const formData = new FormData();
        formData.append(blobKey, coverBlob);
        const blobUpload = await window.fetch("/blob", {
            method: "PUT",
            body: formData,
        });
        const blobResult = await blobUpload.json();
        console.log(blobResult);
        item.covers = [
            ...item.covers,
            { key: blobKey, description: description },
        ];
        await save();
    }

    function setPrimaryCover(index) {
        const cover = item.covers.splice(index, 1);
        item.covers = cover.concat(item.covers);
    }

    function generateKey() {
        item.key = `${item.kind.toLowerCase()}-${importUtil.titleToKey(
            item.name.alternatives[item.name.default]
        )}`;
    }
</script>

<style>
    :global(.edit-item-name) {
        font-style: normal;
        font-weight: normal;
    }

    table {
    }

    table th {
        text-align: left;
    }

    table th,
    table td {
        padding: 0.25em 0.5em 0.25em 0;
    }

    .columns {
        display: flex;
        justify-content: space-around;
        width: 100%;
    }

    .columns > .column {
        border-right: 1px dotted #000;
        flex: 1 1 calc(33% - 1em);
        padding: 0 0.5em;
    }

    .columns > .column:first-child {
        flex: 1 0 calc(33.3% - 0.5em);
        padding: 0 0.5em 0 0;
    }

    .columns > .column:last-child {
        border: 0;
    }

    #covers img {
        height: 20em;
    }
</style>

<main>
    {#await loading}
        Loading…
    {:then}
        <TitleBar>
            Editing
            {util.humanKind(item.kind)}:
            <span
                class="edit-item-name">{item.name.alternatives[item.name.default]}</span>
        </TitleBar>
        <p>Added {item.added}</p>

        <div class="columns">
            <div class="column">
                <section>
                    <div>
                        <label for="key">Key:</label>
                        <input
                            id="key"
                            readonly={!isNewItem}
                            type="text"
                            value={item.key} />
                        {#if isNewItem}
                            <button on:click={generateKey}>Generate From Title</button>
                        {/if}
                    </div>
                    <div>
                        <label for="kind">Kind:</label>
                        <select id="kind" bind:value={item.kind}>
                            <option>Unknown</option>
                            <option>Manga</option>
                            <option>TV</option>
                            <option>Film</option>
                            <option value="NonFiction">Non-Fiction</option>
                            <option>Novel</option>
                            <option>OVA</option>
                            <option>ONA</option>
                            <option>Music</option>
                            <option>Play</option>
                            <option>Collection</option>
                            <option value="ShortStory">Short Story</option>
                            <option>Musical</option>
                            <option value="VisualNovel">Visual Novel</option>
                        </select>
                    </div>

                    <div>
                        <label for="status">Status:</label>
                        <select id="status" bind:value={item.status}>
                            <option>Planned</option>
                            <option value="InProgress">In Progress</option>
                            <option>Completed</option>
                            <option value="OnHold">On Hold</option>
                            <option>Dropped</option>
                        </select>
                    </div>
                    <div>
                        <label for="publication-status">Publication Status:</label>
                        <select
                            id="publication-status"
                            bind:value={item.publication_status}>
                            <option>Publishing</option>
                            <option>Complete</option>
                        </select>
                    </div>

                    <div>
                        <label for="rating">Rating:</label>
                        <select id="rating" bind:value={item.rating}>
                            <option value={null}>-</option>
                            <option value={0}>0</option>
                            <option value={1}>1</option>
                            <option value={2}>2</option>
                            <option value={3}>3</option>
                            <option value={4}>4</option>
                            <option value={5}>5</option>
                            <option value={6}>6</option>
                            <option value={7}>7</option>
                            <option value={8}>8</option>
                            <option value={9}>9</option>
                            <option value={10}>10</option>
                        </select>
                    </div>

                    <div>
                        <label for="started">Started:</label>
                        <NullableMultiDate
                            id="started"
                            placeholder="(not started)"
                            value={item.started}
                            on:input={(e) => (item.started = e.detail)} />
                    </div>
                    <div>
                        <label for="completed">Completed:</label>
                        <NullableMultiDate
                            id="completed"
                            placeholder="(not completed)"
                            value={item.completed}
                            on:input={(e) => (item.completed = e.detail)} />
                    </div>

                    <!-- Name -->
                    <div>
                        <EditAlternatives
                            propertyName="Language"
                            valueName="Name"
                            alternatives={item.name}
                            on:change={(e) => (item.name = e.detail)} />
                    </div>
                    <!-- People -->
                    <!-- Series -->
                    <div>
                        <TagList bind:tags={item.tags} />
                    </div>
                    <!-- Extra/URLs -->
                </section>
                <section class="buttons">
                    <button>Cancel</button>
                    <button on:click={save}>Save</button>
                </section>
            </div>

            <div class="column">
                <section>
                    <h2>Entries</h2>
                    <div>
                        <p>
                            {item.entries.filter((e) => e.completed).length}
                            complete/{item.entries.length}
                            entries
                        </p>
                        <div>
                            <button on:click={() => addMultipleEntries()}>Add
                                Multiple Entries</button>
                            <button
                                on:click={() => completeMultipleEntries()}>Complete
                                Multiple Entries</button>
                        </div>
                        <table>
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Volume</th>
                                    <th>Number</th>
                                    <th>Completed?</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each [...item.entries].reverse() as entry}
                                    <tr>
                                        <td>
                                            {entry.name ? entry.name.alternatives[entry.name.default] : '-'}
                                        </td>
                                        <td>
                                            {entry.volume === null ? '-' : entry.volume}
                                        </td>
                                        <td>{entry.number}</td>
                                        <td>{entry.completed}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                    </div>
                </section>
            </div>

            <div class="column" id="covers">
                <h2>Covers</h2>
                <div>
                    <label for="import">Import URL:</label>
                    <input id="import" type="text" bind:value={urlToImport} />
                    <label for="import-description">Description:</label>
                    <input
                        id="import-description"
                        type="text"
                        bind:value={urlToImportDescription} />
                    <button on:click={importUrl}>Import</button>
                </div>
                {#if item.covers.length > 0}
                    {#each item.covers as cover, index}
                        <div>
                            <img
                                src={'/blob/' + cover.key + '/contents'}
                                alt={cover.description} />
                            <div>
                                <input
                                    type="text"
                                    bind:value={cover.description} />
                                <button
                                    on:click={() => setPrimaryCover(index)}>Make
                                    Primary Cover</button>
                            </div>
                        </div>
                    {/each}
                {:else}
                    <p>No covers.</p>
                {/if}
            </div>

            <!-- Synopsis/Comments -->
        </div>

        <textarea rows="40" cols="80">{JSON.stringify(item, null, 2)}</textarea>
    {:catch error}
        {error}
    {/await}
</main>
