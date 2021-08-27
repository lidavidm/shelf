<!--
Copyright 2021 David Li <li.davidm96@gmail.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->
<script>
    import { onMount } from "svelte";
    import { titleToKey } from "../import/util.mjs";
    import seriesList from "../stores/series.js";

    export let series;
    let creatingSeries = false;
    $: selectedSeries = series ? series[0] : null;

    onMount(function () {
        seriesList.update();
    });

    function updateSelectedSeries(seriesKey) {
        if (seriesKey === "null") {
            series = null;
        } else {
            series = [seriesKey, series ? series[1] : null];
        }
    }

    function createPerson() {
        /* if (!createPersonName) {
         *     createPersonError = "Name must not be blank.";
         *     return;
         * }
         * createPersonError = "";
         * const key = `person-${titleToKey(createPersonName)}`;
         * if ($people[key]) {
         *     createPersonError = `Person already exists: ${$people[key]}`;
         *     return;
         * }
         * window
         *     .fetch("/person", {
         *         method: "PUT",
         *         body: JSON.stringify({
         *             key,
         *             name: {
         *                 default: "English",
         *                 alternatives: {
         *                     English: createPersonName,
         *                 },
         *             },
         *         }),
         *         headers: {
         *             "Content-Type": "application/json",
         *         },
         *     })
         *     .then((req) => req.text())
         *     .then(() => people.update())
         *     .then(() => {
         *         values = [...values, ["Author", key]];
         *         creatingPerson = false;
         *         createPersonName = "";
         *     }); */
    }
</script>

<style>
</style>

<div class="series-editor">
    <label for="series">Series</label>
    <select
        id="series"
        value={selectedSeries}
        on:change={(e) => updateSelectedSeries(e.target.value)}>
        <option value={null}>(no series)</option>
        {#each Object.keys($seriesList).sort() as seriesKey}
            <option value={seriesKey}>{$seriesList[seriesKey]}</option>
        {/each}
    </select>
    <button on:click={() => (creatingSeries = true)} disabled={creatingSeries}>
        <span class="material-icons" aria-hidden="true"> note_add </span>
        Create Series
    </button>
    {#if selectedSeries}
        <br />
        <label for="series-entry">Series Entry:</label>
        <input type="text" id="series-entry" value={series[1]} />
    {/if}
    <!-- <div>
         <button
         on:click={() => (values = [...values, ['Author', Object.keys($people).sort()[0]]])}>
         <span class="material-icons" aria-hidden="true"> note_add </span>
         Add Person
         </button>
         <button
         on:click={() => (creatingPerson = true)}
         disabled={creatingPerson}>
         <span class="material-icons" aria-hidden="true"> person_add </span>
         Create Person
         </button>
         </div> -->
    {#if creatingSeries}
        <!-- <div>
             <label for="new-person-name">Name:</label>
             <input
             id="new-person-name"
             type="text"
             placeholder="Person Name"
             bind:value={createPersonName} />
             <button on:click={createPerson}>
             <span class="material-icons" aria-hidden="true">
             person_add
             </span>
             Create Person
             </button>
             <button on:click={() => (creatingPerson = false)}>
             <span class="material-icons" aria-hidden="true"> cancel </span>
             Cancel
             </button>
             {#if createPersonError}
             <p>{createPersonError}</p>
             {/if}
             </div> -->
    {/if}
</div>
