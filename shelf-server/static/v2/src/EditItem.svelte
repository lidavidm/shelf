<script>
    import { onMount } from "svelte";
    import TitleBar from "./component/TitleBar.svelte";
    import * as util from "./util";

    export let router;
    export let params;

    let item = null;
    let loading = fetch("/item/" + params.key)
        .then((r) => r.json())
        .then((value) => (item = value));
    let people = {};
    let series = {};

    onMount(async function () {
        const [peopleList, seriesList] = await Promise.all([
            fetch("/person").then((r) => r.json()),
            fetch("/series").then((r) => r.json()),
        ]);

        // TODO: these can be stores
        people = buildMap(peopleList);
        series = buildMap(seriesList);
    });

    function buildMap(items) {
        const result = {};
        for (const item of items) {
            result[item.key] = item.name.alternatives[item.name.default];
        }
        return result;
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
                    <option :value="null">-</option>
                    <option :value="0">0</option>
                    <option :value="1">1</option>
                    <option :value="2">2</option>
                    <option :value="3">3</option>
                    <option :value="4">4</option>
                    <option :value="5">5</option>
                    <option :value="6">6</option>
                    <option :value="7">7</option>
                    <option :value="8">8</option>
                    <option :value="9">9</option>
                    <option :value="10">10</option>
                </select>
            </div>

            <!-- Started -->
            <!-- Completed -->

            <!-- Name -->
            <!-- People -->
            <!-- Series -->
            <!-- Tags -->
            <!-- Extra/URLs -->

            <!-- Tabbed: -->
            <!-- Entries -->
            <!-- Covers -->
            <!-- Synopsis/Comments -->
        </section>
        <section class="buttons">
            <button>Cancel</button>
            <button>Save</button>
        </section>
    {:catch error}
        {error}
    {/await}
</main>
