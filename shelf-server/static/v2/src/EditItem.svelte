<script>
    import NullableMultiDate from "./component/NullableMultiDate.svelte";
    import TagList from "./component/TagList.svelte";
    import TitleBar from "./component/TitleBar.svelte";
    import toastStore from "./component/toast.js";
    import * as itemEdit from "./item-edit.mjs";
    import items from "./stores/items.js";
    import people from "./stores/people.js";
    import series from "./stores/series.js";
    import * as util from "./util";

    export let router;
    export let params;

    let item = null;
    let loading = fetch("/item/" + params.key)
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
</script>

<style>
    :global(.edit-item-name) {
        font-style: normal;
        font-weight: normal;
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

        <section>
            <div>
                <label for="key">Key:</label>
                <input id="key" readonly type="text" value={item.key} />
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
                <label for="rating">Rating</label>
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
                <NullableMultiDate
                    id="started"
                    label="Started"
                    placeholder="(not started)"
                    value={item.started}
                    on:input={(e) => (item.started = e.detail)} />
            </div>
            <div>
                <NullableMultiDate
                    id="completed"
                    label="Completed"
                    placeholder="(not completed)"
                    value={item.completed}
                    on:input={(e) => (item.completed = e.detail)} />
            </div>

            <!-- Name -->
            <!-- People -->
            <!-- Series -->
            <div>
                <TagList bind:tags={item.tags} />
            </div>
            <!-- Extra/URLs -->

            <div>
                <button on:click={() => addMultipleEntries()}>Add Multiple
                    Entries</button>
                <button on:click={() => completeMultipleEntries()}>Complete
                    Multiple Entries</button>
            </div>
        </section>
        <section class="buttons">
            <button>Cancel</button>
            <button on:click={save}>Save</button>
        </section>

        <section>
            <!-- Tabbed: -->
            <!-- Entries -->
            <div>
                <p>
                    {item.entries.filter((e) => e.completed).length}
                    complete/{item.entries.length}
                    entries
                </p>
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
            <!-- Covers -->
            <!-- Synopsis/Comments -->
        </section>

        <textarea
            width="100"
            height="200">{JSON.stringify(item, null, 2)}</textarea>
    {:catch error}
        {error}
    {/await}
</main>
