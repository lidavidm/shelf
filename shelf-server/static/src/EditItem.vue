<template>
    <section id="item" v-if="data">
        <header>
            <h2>{{ data.name.alternatives[data.name.default] }} <span>({{ data.kind }})</span></h2>
        </header>
        <section id="item-meta">
            <div>
                <label>Added to shelf</label>
                <span>{{ data.added }}</span>
            </div>

            <div>
                <label>Started on</label>
                <span>{{ data.started }}</span>
            </div>

            <div>
                <label>Finished on</label>
                <span>{{ data.completed }}</span>
            </div>

            <div>
                <label for="kind">Item Type</label>
                    <select id="kind" v-model="data.kind">
                        <option>Unknown</option>
                        <option>Manga</option>
                        <option>TV</option>
                        <option>Film</option>
                        <option>Book</option>
                    </select>
            </div>

            <div>
                <label for="season">Season Name</label>
                    <input id="season" type="text" v-model="data.season" />
            </div>

            <div>
                <label for="status">Status</label>
                    <select id="status" v-model="data.status">
                        <option>Planned</option>
                        <option value="InProgress">In Progress</option>
                        <option>Completed</option>
                        <option value="OnHold">On Hold</option>
                        <option>Dropped</option>
                    </select>
            </div>

            <div>
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
        </section>

        <section>
            <edit-alternative
                key-name="Language"
                value-name="Name"
                v-model="data.name"
            />
        </section>

        <button class="positive" v-on:click="save">Save</button>
        <button class="danger" v-on:click="cancel">Cancel</button>

        <section id="entries">
            <header><h3>{{ entryCategorization(true) }}</h3></header>

            <button v-on:click="nextEntry">Add Next {{ entryCategorization() }}</button>
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
                            <button class="danger">Delete</button>
                            <button>Move Up</button>
                            <button>Move Down</button>
                        </td>
                    </tr>
                    <tr>
                        <td><button>Add Entry</button></td>
                    </tr>
                </tbody>
            </table>
        </section>

        <section id="item-json">
            <code>
{{ JSON.stringify(data, null, 4) }}
            </code>
        </section>
    </section>
    <section id="item" v-else>
        Loading…
    </section>
</template>

<script>
    export default {
        name: "item",
        props: {
            item: String,
        },
        data() {
            return {
                data: null,
            };
        },
        mounted() {
            window.fetch(`/item/${this.item}`)
                  .then(r => r.json())
                  .then((item) => {
                      console.log(item);
                      this.data = item;
                  });
        },
        methods: {
            save() {
                window.fetch(`/item/${this.item}`, {
                    method: "POST",
                    body: JSON.stringify(this.data),
                    headers: {
                        "Content-Type": "application/json",
                    },
                }).then((response) => {
                    if (response.status === 200) {
                        this.$emit("done");
                    }
                    else {
                        // TODO: have an actual error message location
                        alert(`Error ${response.status}: ${response.statusText}`);
                    }
                });
            },
            cancel() {
                this.$emit("done");
            },

            nextEntry() {
                if (!this.data) return;

                const number = this.data.entries.length + 1;
                this.data.entries.push({
                    name: {
                        default: "English",
                        alternatives: {
                            "English": `${this.entryCategorization()} ${number}`,
                        },
                    },
                    number,
                    volume: null,
                    completed: new Date().toISOString(),
                });
            },

            editEntryField(idx, field, ev) {
                const value = ev.target.value.trim();
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
        },
    };
</script>

<style lang="css">
    #item {
        position: fixed;
        box-sizing: border-box;

        top: 1em;
        left: 2em;
        right: 2em;
        height: calc(100vh - 2em);

        padding: 1em;
        overflow-y: auto;

        border: 1px solid var(--theme-2);
        background: var(--theme-base);
        box-shadow: 0px 2px 1px var(--theme-2);
    }

    #item header {
        margin: 0.5em 0 1em;
    }

    #item h2 {
        font-size: 2em;
        font-weight: normal;
        font-style: italic;
        margin: 0;
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
        max-height: 20em;
        overflow-y: scroll;
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
        width: 15em;
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
