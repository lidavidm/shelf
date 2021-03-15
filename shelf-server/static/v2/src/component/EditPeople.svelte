<!--
Copyright 2020 David Li <li.davidm96@gmail.com>

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
    import people from "../stores/people.js";

    export let values;
    let creatingPerson = false;
    let createPersonName = "";
    let createPersonError = "";

    onMount(function () {
        people.update();
    });

    function setPerson(index, role, person) {
        values[index] = [role, person];
        values = values;
    }

    function deleteItem(index) {
        values = values.filter((_, i) => i != index);
    }

    function createPerson() {
        if (!createPersonName) {
            createPersonError = "Name must not be blank.";
            return;
        }
        createPersonError = "";
        const key = `person-${titleToKey(createPersonName)}`;
        if ($people[key]) {
            createPersonError = `Person already exists: ${$people[key]}`;
            return;
        }
        window
            .fetch("/person", {
                method: "PUT",
                body: JSON.stringify({
                    key,
                    name: {
                        default: "English",
                        alternatives: {
                            English: createPersonName,
                        },
                    },
                }),
                headers: {
                    "Content-Type": "application/json",
                },
            })
            .then((req) => req.text())
            .then(() => people.update())
            .then(() => {
                values = [...values, ["Author", key]];
                creatingPerson = false;
                createPersonName = "";
            });
    }
</script>

<style>
</style>

<div class="people-editor">
    <table>
        <thead>
            <tr>
                <th>Person</th>
                <th>Role</th>
            </tr>
        </thead>
        <tbody>
            {#each values as person, index}
                <tr>
                    <td>
                        <select bind:value={person[1]}>
                            {#each Object.keys($people).sort() as personKey}
                                <option value={personKey}>
                                    {$people[personKey]}
                                </option>
                            {/each}
                        </select>
                    </td>
                    <td>
                        <select bind:value={person[0]}>
                            <option value={'Author'}>Author</option>
                            <option value={'Artist'}>Artist</option>
                            <option value={'Director'}>Director</option>
                            <option value={'Translator'}>Translator</option>
                        </select>
                    </td>
                    <td>
                        <button on:click={() => deleteItem(index)}>
                            <span class="material-icons" aria-hidden="true">
                                delete
                            </span>
                        </button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
    <div>
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
    </div>
    {#if creatingPerson}
        <div>
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
        </div>
    {/if}
</div>
