<template>
    <section id="items">

        <label for="import">Import MAL:</label>
        <input id="import" type="file" v-on:change="onFile" />

        <form>
            <label for="import-url">Import URL:</label>
            <input id="import-url" type="text" />
            <button v-on:click="onImportURL">Import</button>
        </form>

        Stats: {{ items.length }} items

        <table>
            <thead>
                <tr>
                    <th></th>
                    <th>Type</th>
                    <th>Name</th>
                    <th>Progress</th>
                    <th>Rating</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items">
                    <td class="item-status" v-bind:class="item['status']"></td>
                    <td>{{ item["kind"] }}</td>
                    <td>
                        {{ item["name"]["alternatives"][item["name"]["default"]] }}
                        <button
                            class="edit"
                            v-on:click="edit(item['key'])"
                        >
                            Edit
                        </button>
                    </td>
                    <td>{{ item["entries"].filter(e => e.completed).length }} / {{ item["entries"].length }}</td>
                    <td>{{ item["rating"] === null ? "-" : item["rating"] }}</td>
                </tr>
            </tbody>
        </table>

        <EditItem
            v-if="editing || editingItem"
            v-bind:item="editing"
            v-bind:initdata="editingItem"
            v-on:done="doneEditing"
        />
    </section>
</template>

<script>
    import firstBy from "thenby";
    import EditItem from "./EditItem";

    import * as imports from "./imports/index";

    export default {
        name: "items",
        data() {
            return {
                items: [],
                editing: null,
                editingItem: null,
            };
        },
        mounted() {
            window.fetch("/item")
                  .then(r => r.json())
                  .then((items) => {
                      this.items = items;
                      this.sortItems();
                  });
        },
        methods: {
            sortItems() {
                return this.items.sort(firstBy(v => v.kind)
                    .thenBy(v => v.status)
                    .thenBy(v => v.name.alternatives[v.name.default].toLowerCase()));
            },

            edit(key) {
                this.editing = key;
            },

            doneEditing() {
                this.editing = null;
                this.editingItem = null;
            },

            onFile(e) {
                const files = e.target.files;
                if (files.length === 0) return;

                const reader = new FileReader();
                reader.onload = () => {
                    const imported = imports.malbackup(reader.result);
                    for (const result of imported) {
                        window.fetch("/item", {
                            method: "PUT",
                            body: JSON.stringify(result),
                            headers: {
                                "Content-Type": "application/json",
                            },
                        });

                        this.items.push(result);
                    }
                    this.sortItems();
                };
                reader.readAsText(files[0]);
            },

            onImportURL(e) {
                e.preventDefault();
                const url = document.querySelector("#import-url").value;

                // TODO: need some way to approve the entry
                imports.byURL(url).then((imported) => {
                    for (const result of imported) {
                        console.log(result);
                        this.editingItem = result;
                        /* window.fetch("/item", {
                         *     method: "PUT",
                         *     body: JSON.stringify(result),
                         *     headers: {
                         *         "Content-Type": "application/json",
                         *     },
                         * });

                         * this.items.push(result); */
                    }
                    this.sortItems();
                });
            },
        },
        components: {
            EditItem,
        },
    };
</script>

<style lang="css">
    table {
        border-collapse: collapse;
        min-width: 60em;
        width: 100%;
    }

    th {
        border-bottom: 1px solid black;
        text-align: left;
    }

    td {
        height: 2.5em;
        line-height: 2.5em;
    }

    .item-status {
        width: 0.25em;
    }

    .item-status.Completed {
        background: var(--theme-completed);
    }

    .item-status.Dropped {
        background: var(--theme-dropped);
    }

    .item-status.OnHold {
        background: var(--theme-onhold);
    }

    .item-status.Planned {
        background: var(--theme-planned);
    }

    .item-status.InProgress {
        background: var(--theme-inprogress);
    }

    .edit {
        float: right;
    }
</style>
