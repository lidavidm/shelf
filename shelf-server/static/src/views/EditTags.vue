<template>
    <div>
        <label for="series">Tags</label>
        <ul id="tags">
            <li v-for="tag in tags">{{tag}}</li>
            <li>
                <input type="text" v-model:value="newTag" id="new-tag" placeholder="New tag" />
                <button @click="addTag">Add Tag</button>
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
        },
    };
</script>

<style lang="css">
</style>
