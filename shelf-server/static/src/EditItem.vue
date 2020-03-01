<template>
    <section id="item" v-if="data">
        <header>
            <h2>{{ data.name.alternatives[data.name.default] }} <span>({{ data.kind }})</span></h2>
        </header>
        <section id="item-body">
            <section id="item-meta">
                <div v-if="canEditKey">
                    <label>Key</label>
                    <input id="key" type="text" v-model="data.key" />
                </div>

                <section id="simple-fields">
                    <div class="field">
                        <label>Added to shelf</label>
                        <span>{{ data.added }}</span>
                    </div>

                   <div class="field">
                        <label for="kind">Item Type</label>
                        <select id="kind" v-model="data.kind">
                            <option>Unknown</option>
                            <option>Manga</option>
                            <option>TV</option>
                            <option>Film</option>
                            <option>Novel</option>
                            <option>OVA</option>
                            <option>ONA</option>
                            <option>Music</option>
                            <option>Play</option>
                            <option>Collection</option>
                            <option :value="'ShortStory'">Short Story</option>
                            <option>Musical</option>
                            <option :value="'VisualNovel'">Visual Novel</option>
                        </select>
                    </div>

                    <div class="field">
                        <label>Started on</label>
                        <input id="started" type="text" v-model="data.started" />
                        <button v-on:click="data.started = now()">Now</button>
                    </div>

                    <div class="field">
                        <label>Finished on</label>
                        <input id="finished" type="text" v-model="data.completed" />
                        <button v-on:click="data.completed = now()">Now</button>
                    </div>

                    <div class="field">
                        <label for="status">Status</label>
                        <select id="status" v-model="data.status">
                            <option>Planned</option>
                            <option value="InProgress">In Progress</option>
                            <option>Completed</option>
                            <option value="OnHold">On Hold</option>
                            <option>Dropped</option>
                        </select>
                    </div>

                    <div class="field">
                        <label for="publication-status">Publication Status</label>
                        <select id="publication-status" v-model="data.publication_status">
                            <option>Publishing</option>
                            <option>Complete</option>
                        </select>
                    </div>

                    <div class="field">
                        <label for="rating">Rating</label>
                        <select id="rating" v-model="data.rating">
                            <option :value="null">-</option>
                            <option :value="0">0</option>
                            <option :value="1">1</option>
                            <option :value="2">2</option>
                            <option :value="3">3</option>
                            <option :value="4">4</option>
                            <option :value="5">5</option>
                            <option :value="6">6</option>
                            <option :value="7">7</option>
                            <option :value="8">8</option>
                            <option :value="9">9</option>
                            <option :value="10">10</option>
                        </select>
                    </div>

                    <section id="external" class="field">
                        <label for="External URL">External URL</label>
                        <input type="text" :value="externalUrl" @input="updateExternalUrl" />
                    </section>
                </section>

                <edit-tags v-model="data.tags" />

                <!-- <div>
                     <label for="season">Season Name</label>
                     <input id="season" type="text" v-model="data.season" />
                     </div>
                -->

                <edit-series v-model="data.series" />
            </section>

            <section>
                <edit-alternative
                    key-name="Language"
                    value-name="Name"
                    v-model="data.name"
                />
            </section>

            <section id="item-people">
                <edit-people v-model="data.people" />
            </section>

            <section id="entries">
                <header>
                    <h3>
                        {{ entryCategorization(true) }}

                        <span>({{ data.entries.filter(e=>e.completed).length }} complete/{{ data.entries.length }})</span>
                    </h3>
                </header>

                <button v-on:click="nextEntry(now())">Complete Next {{ entryCategorization() }}</button>
                <button v-on:click="nextEntry(false)">Add Next {{ entryCategorization() }}</button>
                <button v-on:click="multipleEntry()">Add Multiple {{ entryCategorization() }}</button>
                <table class="entries">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Volume</th>
                            <th>Number</th>
                            <th>Completed</th>
                            <th></th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr v-for="(entry, index) in data.entries" v-bind:key="index">
                            <td>
                                <input
                                    type="text"
                                    v-bind:value="entry.name ? entry.name.alternatives[entry.name.default] : '-'"
                                    v-on:input="editEntryField(index, 'name', $event)"
                                />
                            </td>
                            <td>
                                <input
                                    type="text"
                                    v-bind:value="entry.volume === null ? '-' : entry.volume"
                                    v-on:input="editEntryField(index, 'volume', $event)"
                                />
                            </td>
                            <td>
                                <input
                                    type="text"
                                    v-bind:value="entry.number === null ? '-' : entry.number"
                                    v-on:input="editEntryField(index, 'number', $event)"
                                />
                            </td>
                            <td><input type="checkbox" v-model="entry.completed" /></td>
                            <td>
                                <button class="danger" @click="deleteEntry(index)">Delete</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </section>
        </section>
        <nav id="item-nav">
            <button class="danger" v-on:click="cancel">Cancel</button>
            <button class="positive" v-on:click="save">Save</button>
        </nav>
    </section>
    <section id="item" v-else>
        Loading…
    </section>
</template>

<script>
    import moment from "moment";
    import firstBy from "thenby";

    export default {
        name: "item",
        props: {
            item: String,
            initdata: Object,
        },
        data() {
            return {
                data: null,
                canEditKey: false,
                series: [],
            };
        },
        mounted() {
            if (this.initdata) {
                this.data = this.initdata;
                this.canEditKey = this.data.key === null;
            }
            else {
                window.fetch(`/item/${this.item}`)
                      .then(r => r.json())
                      .then((item) => {
                          console.log(item);
                          this.data = item;
                      });
            }
            window.fetch("/series")
                  .then(r => r.json())
                  .then((series) => {
                      series.sort(firstBy(x => x.name.alternatives[x.name.default]));
                      for (const s of series) {
                          this.series.push(s);
                      }
                  });
        },
        computed: {
            externalUrl() {
                if (this.data && this.data.extra) {
                    if (this.data.extra.mangadex_url) {
                        return this.data.extra.mangadex_url;
                    }
                    else if (this.data.extra.external_url) {
                        return this.data.extra.external_url;
                    }
                }
                return null;
            },
        },
        methods: {
            updateExternalUrl(e) {
                const value = e.target.value;
                try {
                    const url = new URL(value);
                    if (!this.data.extra) {
                        this.data.extra = {};
                    }
                    if (url.host === "mangadex.org" || url.host === "mangadex.cc") {
                        this.data.extra.mangadex_url = value;
                    }
                    else {
                        this.data.extra.external_url = value;
                    }
                }
                catch (e) {

                }
            },
            save() {
                let itemKey = this.item || this.data.key;
                window.fetch(`/item/${itemKey}`, {
                    method: "POST",
                    body: JSON.stringify(this.data),
                    headers: {
                        "Content-Type": "application/json",
                    },
                }).then((response) => {
                    if (response.status >= 200 && response.status < 300) {
                        this.$emit("done");
                    }
                    else {
                        // TODO: have an actual error message location
                        alert(`Error ${response.status}: ${response.statusText}`);
                    }
                });
            },
            cancel() {
                this.$emit("done-nochange");
            },

            nextEntry(completed) {
                if (!this.data) return;

                let number = 0;
                let maxCompleted = null;
                for (const entry of this.data.entries) {
                    number = Math.max(number, entry.number);

                    if (entry.completed) {
                        if (maxCompleted === null) {
                            maxCompleted = entry.number;
                        }
                        maxCompleted = Math.max(maxCompleted, entry.number);
                    }
                }
                number += 1

                if (completed && maxCompleted !== null && maxCompleted < number) {
                    for (const entry of this.data.entries) {
                        if (entry.number === maxCompleted + 1) {
                            entry.completed = completed;
                        }
                    }
                    return;
                }
                else if (completed && maxCompleted === null && this.data.entries.length > 0) {
                    for (const entry of this.data.entries) {
                        if (entry.number === 1) {
                            entry.completed = completed;
                            return;
                        }
                    }
                }

                this.data.entries.push({
                    name: {
                        default: "English",
                        alternatives: {
                            "English": `${this.entryCategorization()} ${number}`,
                        },
                    },
                    number,
                    volume: null,
                    completed,
                });
            },

            multipleEntry() {
                const num = parseInt(window.prompt("Number of entries to add?"), 10);

                let number = 0;
                for (const entry of this.data.entries) {
                    number = Math.max(number, entry.number);
                }

                for (let i = 0; i < num; i++) {
                    this.data.entries.push({
                        name: {
                            default: "English",
                            alternatives: {
                                "English": `${this.entryCategorization()} ${number + i + 1}`,
                            },
                        },
                        number: number + i + 1,
                        volume: null,
                        completed: false,
                    });
                }
            },

            deleteEntry(index) {
                this.data.entries.splice(index, 1);
            },

            editEntryField(idx, field, ev) {
                const value = ev.target.value.trim();

                if (field === "name") {
                    if (value === "" || value === "-") {
                        this.data.entries[idx].name = null;
                        ev.target.value = "-";
                    }
                    else if (this.data.entries[idx].name === null) {
                        this.data.entries[idx].name = {
                            default: "English",
                            alternatives: {
                                "English": value,
                            },
                        };
                    }
                    else {
                        const names = this.data.entries[idx].name;
                        names.alternatives[names.default] = value;
                    }
                    return;
                }

                if (value === "-" || value === "") {
                    this.data.entries[idx][field] = null;
                }
                else {
                    const vol = parseInt(value, 10);
                    if (Number.isInteger(vol) && vol >= 0) {
                        this.data.entries[idx][field] = vol;
                    }
                    else {
                        ev.target.value = "-";
                    }
                }
            },

            entryCategorization(plural=false) {
                if (!this.data) {
                    return plural ? "Entries" : "Entry";
                }
                switch (this.data.kind) {
                    case "TV":
                        return plural ? "Episodes" : "Episode";
                    case "Book":
                    case "Manga":
                        return plural ? "Chapters" : "Chapter";
                    default:
                        return plural ? "Entries" : "Entry";
                }
            },

            sortedNameAlternatives() {
                const entries = Object.entries(this.data.name.alternatives);
                entries.sort((alt1, alt2) => {
                    const k1 = alt1[0];
                    const k2 = alt2[0];
                    if (k1 === k2) {
                        return 0;
                    }
                    else if (k1 > k2) {
                        return 1;
                    }
                    return -1;
                });
                return entries;
            },

            now() {
                return moment().format();
            },
        },
    };
</script>

<style lang="css">
    #item {
        position: fixed;
        box-sizing: border-box;

        top: 1em;
        left: 4em;
        right: 4em;
        height: calc(100vh - 2em);

        border: 1px solid var(--theme-2);
        background: var(--theme-base);
        box-shadow: 0px 2px 1px var(--theme-2);
    }

    #item > header {
        height: 8%;
        padding: 1em 1em 0 1em;
        box-sizing: border-box;
    }

    #item-body {
        height: 88%;
        overflow-y: auto;
        padding: 0em 1em;
        box-sizing: border-box;
    }

    #simple-fields {
        display: flex;
        flex-wrap: wrap;
    }

    .field {
        flex: 1 0 50%;
        padding: 0.25em 0.5em 0.25em 0;
        box-sizing: border-box;
        line-height: 2em;
        height: 2em;
        display: flex;
    }

    .field > * {
        flex: 0 0 auto;
    }

    .field > label {
        padding-right: 0.5em;
    }

    .field > select, .field > input[type="text"] {
        flex: 1 0 auto;
    }

    #item-nav {
        display: flex;
        height: 4%;
    }

    #item-nav button {
        flex: 1 0;
    }

    #item-nav button:nth-child(1) {

    }

    #item h2 {
        font-size: 2em;
        font-weight: normal;
        margin: 0 0 0.5em;
    }

    #item h2 span {
        font-style: normal;
        font-weight: normal;
        font-size: 0.7em;
    }

    #item-meta {

    }

    #item-meta > div {
        margin: 0.5em 0;
    }

    #item-meta label {
        font-weight: bold;
    }

    #item-json {
    }

    #entries > header h3 span {
        font-weight: normal;
    }

    .entries, .names {
        table-layout: fixed;
    }

    table input[type=text] {
        width: 100%;
    }

    .entries th:nth-child(1) {
    }

    .entries th:nth-child(2) {
    }

    .entries th:nth-child(3 ) {
    }

    .entries th:nth-child(4) {
        width: 5em;
    }

    .entries th:nth-child(5) {
        width: 5em;
    }

    .entries td:nth-child(5) {
        display: flex;
    }

    code {
        display: block;
        white-space: pre-wrap;
        font-size: 0.6em;
    }
</style>
