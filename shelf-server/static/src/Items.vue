<template>
    <section id="items">
        <section id="controls">
            <form>
                <button v-on:click="addEntry">Add Entry</button>

                <label for="import-url">Import URL:</label>
                <input id="import-url" type="text" />
                <button v-on:click="onImportURL">Import</button>
            </form>

            <form>
                <label for="search">Search:</label>
                <input ref="search" id="search" type="text" placeholder="by title" v-model="searchQuery" />
            </form>

            Stats: {{ items.length }} items
        </section>

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

            <tbody v-for="category in itemsByCategory" v-bind:id="category.title">
                <tr class="items-category-title">
                    <td colspan="6">
                        {{category.title}}
                        ({{category.items.length}} items)
                    </td>
                </tr>
                <tr v-for="item in category.items" v-bind:id="item.key" v-if="searchMatches(item)">
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
    import * as util from "./util";

    export default {
        name: "items",
        data() {
            return {
                // The raw items
                items: [],
                // A list of { category: string, items: list } objects
                itemsByCategory: [],
                searchQuery: "",
                editing: null,
                editingItem: null,
                series: {},
                people: {},
            };
        },
        mounted() {
            this.getItems();
            this.$refs.search.focus();
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
                this.itemsByCategory = [];
                this.items.sort(firstBy(v => v.status === "InProgress" ? 0 : 1)
                    .thenBy(v => v.kind)
                    .thenBy(v => v.status)
                    .thenBy(v => v.name.alternatives[v.name.default].toLowerCase()));

                const firstNotInProgress = this.items.findIndex(item => item.status !== "InProgress");
                this.itemsByCategory.push({
                    "title": "In Progress",
                    "items": this.items.slice(
                        0,
                        firstNotInProgress === -1 ? this.items.length : firstNotInProgress
                    ),
                });
                let prevKind = null;
                for (let index = firstNotInProgress; index < this.items.length; index++) {
                    const item = this.items[index];
                    if (item.kind != prevKind) {
                        prevKind = item.kind;
                        this.itemsByCategory.push({
                            "title": util.humanKind(item.kind),
                            "items": [],
                        });
                    }
                    this.itemsByCategory[this.itemsByCategory.length - 1].items.push(item);
                }

                return this.items;
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
                window.fetch("/item/:template:")
                      .then(r => r.json())
                      .then((template) => {
                          template.key = null;
                          this.editingItem = template;
                      });
            },

            searchMatches(item) {
                if (!this.searchQuery) {
                    return true;
                }

                return Object.values(item.name.alternatives)
                             .some((v) => v.toLowerCase().indexOf(this.searchQuery.toLowerCase()) >= 0);
            },
        },
        components: {
            EditItem,
        },
    };
</script>

<style lang="css">
    #controls > * {
        margin: 1em 0;
    }

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
        padding-right: 0.25em;
        height: 2em;
        line-height: 1.5em;
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

    .items-category-title {
        font-style: italic;
        padding: 0.25em 0;
        border-bottom: 2px solid black;
        position: sticky;
        top: 0;
        background: #EFEFEF;
    }

    .items-category-title td {
        text-align: center;
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
