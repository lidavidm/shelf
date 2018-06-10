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
                    <th>External</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items" v-bind:id="item.key">
                    <td class="item-status" v-bind:class="item['status']"></td>
                    <td>{{ item["kind"] }}</td>
                    <td>
                        <a
                            v-on:click="edit(item.key)"
                            title="Click to edit"
                        >
                            {{ item["name"]["alternatives"][item["name"]["default"]] }}
                        </a>
                    </td>
                    <td>{{ item["entries"].filter(e => e.completed).length }} / {{ item["entries"].length }}</td>
                    <td>{{ item["rating"] === null ? "-" : item["rating"] }}</td>

                    <td>
                        <a
                            v-if="item.extra.mangadex_url"
                            v-bind:href="item.extra.mangadex_url"
                            target="_blank"
                        >
                            Mangadex
                        </a>
                         <a
                            v-if="item.extra.mal_id"
                            v-bind:href="malUrl(item.kind, item.extra.mal_id)"
                            target="_blank"
                        >
                            MyAnimeList
                        </a>
                    </td>
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
                return this.items.sort(firstBy(v => v.status === "InProgress" ? 0 : 1)
                    .thenBy(v => v.kind)
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

            malUrl(kind, id) {
                if (kind === "Manga") {
                    return `https://myanimelist.net/manga/${id}`;
                }
                return `https://myanimelist.net/anime/${id}`;
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

    th:nth-child(1) {
        padding-right: 0.5em;
        width: 0.3em;
    }

    th:nth-child(2) {
        width: 6em;
    }

    th:nth-child(4) {
        width: 7em;
    }

    th:nth-child(5) {
        width: 5em;
    }

    td {
        height: 2.5em;
        line-height: 2.5em;
    }

    td a {
        color: inherit;
        text-decoration: none;
    }

    td a:hover {
        text-decoration: underline;
        cursor: pointer;
    }

    .item-status {
        border-bottom: 1px solid var(--theme-base);
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
