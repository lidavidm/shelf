<template>
    <section id="items">

        <label for="import">Import MAL:</label>
        <input id="import" type="file" v-on:change="onFile" />

        <form>
            <label for="import-url">Import URL:</label>
            <input id="import-url" type="text" />
            <button v-on:click="onImportURL">Import</button>
        </form>

        <button v-on:click="addEntry">Add Entry</button>

        Stats: {{ items.length }} items

        <table id="items-list">
            <thead>
                <tr>
                    <th></th>
                    <th class="type">Type</th>
                    <th class="name">Name</th>
                    <th class="progress">Progress</th>
                    <th class="rating">Rating</th>
                    <th class="external">External</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items" v-bind:id="item.key">
                    <td class="item-status" v-bind:class="item['status']"></td>
                    <td class="type">{{ item["kind"] }}</td>
                    <td class="name">
                        <a
                            v-on:click="edit(item.key)"
                            title="Click to edit"
                        >
                            {{ item["name"]["alternatives"][item["name"]["default"]] }}
                        </a>
                        <span
                            class="item-series"
                        >
                            <span v-if="item.series && series[item.series[0]]">
                                {{series[item.series[0]]}}<template v-if="item.series[1]">, {{item.series[1]}}</template>
                            </span>
                            <span v-if="item.people.some(x => x[0] === 'Author')">
                                <em>by</em> {{getAuthor(item.people)}}
                            </span>
                        </span>
                    </td>
                    <td class="progress">
                        {{ item["entries"].filter(e => e.completed).length }} /
                        {{ item.publication_status === "Complete" ? item["entries"].length : "?" }}
                    </td>
                    <td class="rating">{{ item["rating"] === null ? "-" : item["rating"] }}</td>

                    <td class="external">
                        <a
                            v-if="item.extra.external_url"
                            v-bind:href="item.extra.external_url"
                            target="_blank"
                        >
                            {{(new URL(item.extra.external_url)).host}}
                        </a>
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
    import moment from "moment";
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
                series: {},
                people: {},
            };
        },
        mounted() {
            this.getItems();
        },
        methods: {
            getItems() {
                window.fetch("/series")
                      .then(r => r.json())
                      .then((items) => {
                          for (const series of items) {
                              this.$set(this.series, series.key, series.name.alternatives[series.name.default]);
                          }
                      });

                this.people = {};
                window.fetch("/person")
                      .then(r => r.json())
                      .then((people) => {
                          for (const person of people) {
                              this.people[person.key] = person;
                          }
                      });

                window.fetch("/item")
                      .then(r => r.json())
                      .then((items) => {
                          this.items = items;
                          this.sortItems();
                      });
            },

            getAuthor(people) {
                const author = people.filter(x => x[0] === 'Author')[0][1];
                return this.people[author].name.alternatives[this.people[author].name.default];
            },

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
                this.getItems();
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

            addEntry() {
                this.editingItem = {
                    key: null,
                    kind: "Manga",
                    name: {
                        default: "English",
                        alternatives: {
                            "English": "",
                        },
                    },
                    people: [],
                    season: null,
                    entries: [],
                    status: "Planned",
                    rating: null,
                    added: moment().format(),
                    started: null,
                    completed: null,
                    extra: {},
                };
            },
        },
        components: {
            EditItem,
        },
    };
</script>

<style lang="css">
    #items-list {
        border-collapse: collapse;
        min-width: 60em;
        width: 100%;
        border-top: 4px solid #000;
        margin: 1em 0;
    }

    #items-list th {
        padding: 0.25em 0;
        border-bottom: 2px solid black;
        text-align: left;
    }

    #items-list th.status {
        width: 0.25em;
    }

    #items-list th.type {
        width: 5em;
    }

    #items-list th.progress {
        width: 6em;
    }

    #items-list th.rating {
        width: 6em;
    }

    #items-list th.type, #items-list td.type,
    #items-list th.progress, #items-list td.progress,
    #items-list th.rating, #items-list td.rating {
        text-align: left;
    }

    #items-list td.name a {
        font-style: italic;
    }

    #items-list td {
        height: 2em;
        line-height: 2em;
        vertical-align: top;
    }

    #items-list td a {
        color: inherit;
        text-decoration: none;
    }

    #items-list td a:hover {
        text-decoration: underline;
        cursor: pointer;
    }

    .item-status {
        border-bottom: 1px solid var(--theme-base);
        border-left: 0.25em solid transparent;
    }

    .item-status.Completed {
        border-left-color: var(--theme-completed);
    }

    .item-status.Dropped {
        border-left-color: var(--theme-dropped);
    }

    .item-status.OnHold {
        border-left-color: var(--theme-onhold);
    }

    .item-status.Planned {
        border-left-color: var(--theme-planned);
    }

    .item-status.InProgress {
        border-left-color: var(--theme-inprogress);
    }

    .item-series {
        display: block;
        font-size: 1em;
        line-height: 1em;
    }

    .edit {
        float: right;
    }
</style>
