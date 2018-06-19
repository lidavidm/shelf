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
            };
        },
        mounted() {
            this.series = this.value;
            window.fetch("/series")
                  .then(r => r.json())
                  .then((series) => {
                      this.allSeries = series;
                      this.sort();
                  });
        },
        methods: {
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
                    console.log(JSON.stringify([ series[0], null ]));
                    this.$emit("input", [ series[0], null ]);
                }
                else {
                    console.log(JSON.stringify(series));
                    this.$emit("input", series);
                }
            },
        },
    };
</script>

<style lang="css">
</style>
