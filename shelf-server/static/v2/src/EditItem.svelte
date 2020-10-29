<script>
    import { onMount } from "svelte";
    import * as util from "./util";

    export let router;
    export let params;

    let item = null;
    let loading = fetch("/item/" + params.key).then(r => r.json()).then((value) => item = value);
    let people = {};
    let series = {};

    onMount(async function() {
        const [peopleList, seriesList] = await Promise.all([
            fetch("/person").then(r => r.json()),
            fetch("/series").then(r => r.json()),
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

<main>
    {#await loading}
        Loading…
    {:then}
        <header>
            <h2>Editing {util.humanKind(item.kind)}: <span>{item.name.alternatives[item.name.default]}</span></h2>
            <p>Added {item.added}</p>
        </header>

        <section>
            <div>
                <label for="key">Key:</label>
                <input id="key" readonly type="text" value={item.key} />
            </div>

            <!-- Name -->

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
                <select id="publication-status" bind:value={item.publication_status}>
                    <option>Publishing</option>
                    <option>Complete</option>
                </select>
            </div>

            <!-- Rating -->

            <!-- Started -->
            <!-- Completed -->

            <!-- People -->
            <!-- Series -->
            <!-- Tags -->
            <!-- Extra/URLs -->

            <!-- Synopsis -->
            <!-- Comments -->

            <!-- Covers -->
            <!-- Entries -->
        </section>
        <section class="buttons">
            <button>Cancel</button>
            <button>Save</button>
        </section>
    {:catch error}
        {error}
    {/await}
</main>

<style>
    header h2 span {
        font-style: italic;
        font-weight: normal;
    }
</style>
