<template>
    <section id="items">
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
                            <span
                                v-for="person in item.people"
                                v-if="person[0] === 'Author' || person[0] === 'Director'">
                                <em>by</em> {{getPersonName(person[1])}}
                            </span>
                            <div class="tag-list" v-if="item.tags && item.tags.length > 0">
                                <em>Tags:</em>
                                <span class="tag" v-for="tag in item.tags">{{tag}}</span>
                            </div>
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
                            {{hostname(item.extra.external_url)}}
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
            v-on:done="doneEditing(true)"
            v-on:done-nochange="doneEditing(false)"
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
            hostname(url) {
                return (new URL(url)).host;
            },
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

            getPersonName(person) {
                return this.people[person].name.alternatives[this.people[person].name.default];
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

            doneEditing(reload) {
                this.editing = null;
                this.editingItem = null;
                if (reload) {
                    this.getItems();
                }
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

                imports.byURL(url).then((imported) => {
                    for (const result of imported) {
                        console.log(result);
                        this.editingItem = result;
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
                    publication_status: "Publishing",
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
        font-size: 0.9em;
        line-height: 1em;
    }

    .tag-list {
        display: inline-block;
    }

    .tag-list .tag {
        display: inline-block;
    }

    .tag-list .tag::after {
        display: inline-block;
        content: ", ";
    }

    .tag-list .tag:last-child::after {
        content: "";
    }

    .edit {
        float: right;
    }
</style>
