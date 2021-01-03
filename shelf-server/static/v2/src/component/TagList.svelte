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
    import allTags from "../stores/tags.js";

    allTags.update();

    export let tags = [];
    let selectedIndex = -1;
    let newTag = "";

    $: suggestions = updateSuggestions(newTag);

    function updateSuggestions(newTag) {
        if (newTag != "") {
            return Array.from($allTags.beginsWith(newTag)).sort();
        }
        return [];
    }

    function removeTag(tag) {
        tags = tags.filter((x) => x !== tag);
    }

    function handleKeyDown(e) {
        switch (e.code) {
            case "Tab": {
                if (newTag === "") {
                    return;
                }
                e.preventDefault();
                if (suggestions.length > 0) {
                    selectedIndex = (selectedIndex + 1) % suggestions.length;
                }
                return;
            }
        }
    }

    function handleKeyUp(e) {
        switch (e.code) {
            case "Enter": {
                if (selectedIndex >= 0) {
                    tags = [...tags, suggestions[selectedIndex]].sort();
                } else if (newTag !== null && !newTag.match(/^\s*$/)) {
                    tags = [...tags, newTag].sort();
                } else {
                    return;
                }
                newTag = "";
                selectedIndex = -1;
                return;
            }
        }
    }
</script>

<style>
    .tag-editor ul {
        margin: 0;
        padding: 0;
        list-style-type: none;
        display: inline-flex;
        justify-content: space-between;
        min-height: 2em;
        border-bottom: 1px solid #000;
    }

    li.tag {
        border: 1px solid #000;
        display: flex;
        flex: 0 0 auto;
        margin-right: 0.5em;
        padding: 0 0 0 0.5em;
        height: 1.75em;
        line-height: 2em;
    }

    li.tag button {
        background: none;
        border: 0;
        border-left: 1px solid #000;
        font-family: inherit;
        font-size: inherit;
        padding: 0 0.5em;
        margin: 0 0 0 0.25em;
    }

    li.tag button:hover {
        cursor: pointer;
    }

    input[type="text"] {
        border-bottom: 0;
        height: 100%;
    }

    .tag-input {
        position: relative;
    }

    .suggestions {
        position: absolute;
        left: 0;
        top: 2em;
        z-index: 9999;
        background: #fff;
        border: 1px solid #000;
        box-shadow: 0 5px 5px rgba(0, 0, 0, 0.2);
        flex-direction: column;
    }

    .suggestions li {
        padding: 0 0.25em;
    }

    .suggestions li.selected {
        font-weight: bold;
        text-decoration: underline;
    }
</style>

<div class="tag-editor">
    <label for="tag-input">Tags</label>
    <ul>
        {#each tags as tag}
            <li class="tag">
                {tag}
                <button on:click={removeTag(tag)}>×</button>
            </li>
        {/each}
        <li class="tag-input">
            <input
                type="text"
                id="tag-input"
                placeholder="New tag"
                bind:value={newTag}
                on:keydown={handleKeyDown}
                on:keyup={handleKeyUp} />
            {#if suggestions.length > 0}
                <ul class="suggestions">
                    {#each suggestions as suggestion, index}
                        <li class:selected={index === selectedIndex}>
                            {suggestion}
                        </li>
                    {/each}
                </ul>
            {/if}
        </li>
    </ul>
</div>
