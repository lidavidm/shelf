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
    import { createEventDispatcher } from "svelte";

    export let propertyName;
    export let valueName;
    export let alternatives;

    let defaultAlternative = 0;
    let values = [];
    let index = 0;
    for (const value of Object.entries(alternatives.alternatives)) {
        values.push({
            property: value[0],
            value: value[1],
        });
        if (value[0] === alternatives.default) {
            defaultAlternative = index;
        }
        index++;
    }

    const dispatch = createEventDispatcher();

    $: {
        let newValue = {
            default: values[defaultAlternative].property,
            alternatives: Object.fromEntries(
                values.map((value) => [value.property, value.value])
            ),
        };
        dispatch("change", newValue);
    }

    function deleteItem(index) {
        values = values.filter((_, i) => i != index);
        if (values.length === 0) {
            values = [...values, { property: "", value: "" }];
        }
        if (defaultAlternative >= values.length) {
            defaultAlternative = values.length - 1;
        }
    }
</script>

<style>
</style>

<div class="alternatives-editor">
    <table>
        <thead>
            <tr>
                <th>{propertyName}</th>
                <th>{valueName}</th>
                <th>Default?</th>
            </tr>
        </thead>
        <tbody>
            {#each values as value, index}
                <tr>
                    <td><input type="text" bind:value={value.property} /></td>
                    <td><input type="text" bind:value={value.value} /></td>
                    <td>
                        <input
                            type="radio"
                            name="default"
                            value={index}
                            bind:group={defaultAlternative} />
                        <button on:click={() => deleteItem(index)}>
                            <span class="material-icons" aria-hidden="true">
                                delete
                            </span>
                        </button>
                    </td>
                </tr>
            {/each}
            <tr>
                <button
                    on:click={() => (values = [...values, { property: '', value: '' }])}>
                    <span class="material-icons" aria-hidden="true">
                        note_add
                    </span>
                    Add
                    {valueName}
                </button>
            </tr>
        </tbody>
    </table>
</div>
