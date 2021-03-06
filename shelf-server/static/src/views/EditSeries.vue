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
    <div>
        <label for="series">Series</label>
        <select
            id="series"
            v-bind:value="series ? series[0] : null"
            v-on:input="selectSeries"
        >
            <option :value="null">-</option>
            <option v-for="s in allSeries" v-bind:value="s.key">
                {{ s.name.alternatives[s.name.default] }}
            </option>
        </select>
        <label v-if="series">Series Index:</label>
        <input
            type="text"
            v-if="series"
            v-bind:value="series[1]"
            v-on:input="updateIndex"
        />
        <template v-if="addingSeries">
            <label for="seriesName">New Series Name:</label>
            <input type="text" v-model:value="newSeriesName" id="seriesName" />
            <button @click="createSeries">Save Series</button>
        </template>
        <button v-else @click="addingSeries = true">Add Series</button>
    </div>
</template>

<script>
    import firstBy from "thenby";
    import * as util from "../util";

    export default {
        name: "edit-series",
        props: {
            value: Array,
        },
        data() {
            return {
                allSeries: [],
                series: null,
                addingSeries: false,
                newSeriesName: null,
            };
        },
        mounted() {
            this.series = this.value;
            this.getSeries();
        },
        methods: {
            getSeries() {
                window.fetch("/series")
                      .then(r => r.json())
                      .then((series) => {
                          this.allSeries = series;
                          this.sort();
                      });
            },

            sort() {
                this.allSeries.sort(firstBy(x => x.name.alternatives[x.name.default]));
            },

            selectSeries(e) {
                if (this.series) {
                    this.series[0] = e.target.value;
                }
                else {
                    this.series = [ e.target.value, null ];
                }
                this.update();
            },

            updateIndex(e) {
                const val = e.target.value.trim();
                this.series[1] = val ? val : null;
                this.update();
            },

            update() {
                const series = this.series;
                if (series && !series[1]) {
                    this.$emit("input", [ series[0], null ]);
                }
                else {
                    this.$emit("input", series);
                }
            },

            createSeries() {
                const name = this.newSeriesName.trim();
                const key = util.makeKey("series", name);
                window.fetch(`/series`, {
                    method: "PUT",
                    body: JSON.stringify({
                        key,
                        name: {
                            default: "English",
                            alternatives: {
                                "English": name,
                            },
                        },
                        people: [],
                    }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                }).then(req => req.text()).then(() => this.getSeries());
                this.addingSeries = false;
                this.newSeriesName = null;
            },
        },
    };
</script>

<style lang="css">
</style>
