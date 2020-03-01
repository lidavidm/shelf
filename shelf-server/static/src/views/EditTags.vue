<template>
    <div class="tag-editor">
        <label for="series">Tags</label>
        <ul id="tags">
            <li v-for="tag in tags" class="tag">
                {{tag}}
                <button @click="removeTag(tag)">×</button>
            </li>
            <li>
                <input
                    type="text"
                    id="new-tag"
                    placeholder="New tag"
                    v-model:value="newTag"
                    @keyup.enter="addTag()"
                />
            </li>
        </ul>
    </div>
</template>

<script>
    import firstBy from "thenby";
    import * as util from "../util";

    export default {
        name: "edit-tags",
        props: {
            value: Array,
        },
        data() {
            return {
                tags: null,
                newTag: null,
            };
        },
        mounted() {
            this.tags = this.value;
        },
        methods: {
            sort() {
                this.tags.sort();
            },

            update() {
            },

            addTag() {
                if (this.newTag != null && !this.newTag.match(/^\s*$/)) {
                    this.tags.push(this.newTag);
                    this.sort();
                    this.newTag = "";
                } else {
                    alert("Tag must not be blank.");
                }
            },

            removeTag(tag) {
                this.tags = this.tags.filter(x => x !== tag);
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
</style>
