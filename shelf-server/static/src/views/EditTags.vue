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

<template>
    <div class="tag-editor">
        <label for="series">Tags</label>
        <ul id="tags">
            <li v-for="tag in tags" class="tag">
                {{tag}}
                <button @click="removeTag(tag)">×</button>
            </li>
            <li class="tag-input">
                <input
                    type="text"
                    id="new-tag"
                    placeholder="New tag"
                    v-model:value="newTag"
                    @keyup.enter="addTag()"
                    @keyup.tab="selectSuggestionCycle"
                    @keydown.tab.stop.prevent=""
                    @keyup.escape="clearSuggestions()"
                />
                <!-- TODO: full tab completion -->
                <!-- @keyup.up="selectSuggestionUp()" -->
                <!-- @keyup.down="selectSuggestionDown()" -->
                <ul class="suggestions" v-if="suggestions.length > 0">
                    <li
                        v-for="(suggestion, index) in suggestions"
                        v-bind:class="{ selected: index === selectedIndex }"
                    >
                        {{suggestion}}
                    </li>
                </ul>
            </li>
        </ul>
    </div>
</template>

<script>
    import firstBy from "thenby";
    import PrefixTrie from "../prefixtrie";
    import * as util from "../util";

    export default {
        name: "edit-tags",
        props: {
            value: Array,
        },
        data() {
            return {
                tags: null,
                autocomplete: null,
                suggestions: [],
                newTag: null,
                selectedIndex: -1,
            };
        },
        mounted() {
            this.tags = this.value;
            window.fetch("/tag")
                  .then(r => r.json())
                  .then(tags => {
                      this.autocomplete = new PrefixTrie(tags);
                  });
        },
        methods: {
            sort() {
                this.tags.sort();
            },

            update() {
            },

            addTag() {
                if (this.selectedIndex >= 0) {
                    this.tags.push(this.suggestions[this.selectedIndex]);
                } else if (this.newTag != null && !this.newTag.match(/^\s*$/)) {
                    this.tags.push(this.newTag);
                } else {
                    alert("Tag must not be blank.");
                    return;
                }
                this.sort();
                this.newTag = "";
            },

            removeTag(tag) {
                this.tags = this.tags.filter(x => x !== tag);
            },

            selectSuggestionCycle() {
                if (this.suggestions.length > 0) {
                    this.selectedIndex = (this.selectedIndex + 1) % this.suggestions.length;
                }
            },

            clearSuggestions() {
                this.suggestions = [];
                this.selectedIndex = -1;
            },
        },
        watch: {
            newTag: function(prefix, oldPrefix) {
                if (prefix === "") {
                    this.clearSuggestions();
                }
                else if (prefix !== oldPrefix) {
                    this.suggestions = Array.from(this.autocomplete.beginsWith(prefix)).sort();
                }
            },
        },
    };
</script>

<style lang="css" scoped>
    ul {
        margin: 0;
        padding: 0;
        list-style-type: none;
        display: inline-flex;
        justify-content: space-between;
        min-height: 2em;
        border-bottom: 1px solid #000;
    }

    li.tag {
        display: flex;
        flex: 0 0 auto;
        margin-right: 0.5em;
        padding: 0 0.25em;
        background: var(--theme-4);
        border-radius: 0.2em;
        height: 1.75em;
        line-height: 2em;
    }

    input[type="text"] {
        border-bottom: 0;
    }

    .tag-input {
        position: relative;
    }

    .suggestions {
        position: absolute;
        left: 0;
        top: 2em;
        z-index: 9999;
        background: #FFF;
        border: 1px solid #000;
        box-shadow: 0 5px 5px rgba(0,0,0,0.2);
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
