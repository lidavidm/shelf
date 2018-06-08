<template>
    <div>
        <table class="alternatives">
            <thead>
                <tr>
                    <th>{{ keyName }}</th>
                    <th>{{ valueName }}</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(field, idx) in sortedAlternatives" v-bind:key="field[2]">
                    <td>
                        <input
                            type="text"
                            v-bind:value="field[0]"
                            v-on:input="editKey(idx, $event)"
                        />
                    </td>
                    <td>
                        <input
                            type="text"
                            v-bind:value="field[1]"
                            v-on:input="editValue(idx, $event)"
                        />
                    </td>
                    <td><button v-on:click="deleteAlternative(idx)" class="danger">Delete</button></td>
                </tr>
                <tr>
                    <td>
                        <button v-on:click="addAlternative">Add {{ valueName }}</button>
                    </td>
                    <td>
                        <label for="name-default">Default</label>
                        <select id="name-default" v-model="defaultKey" v-on:change="update">
                            <option v-for="field in sortedAlternatives" :value="field[0]">{{ field[0] }}</option>
                        </select>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script>
    import firstBy from "thenby";

    export default {
        name: "edit-alternative",
        props: {
            keyName: String,
            valueName: String,
            value: Object,
        },
        data() {
            return {
                defaultKey: null,
                nextIdx: 0,
                sortedAlternatives: [],
            };
        },
        mounted() {
            this.defaultKey = this.value.default;
            this.sortedAlternatives = Object
                .entries(this.value.alternatives)
                .map(([ k, v ], idx) => [ k, v, idx ]);
            this.sortedAlternatives.sort(firstBy(v => v[0]));
            this.nextIdx = this.sortedAlternatives.length;
        },
        methods: {
            addAlternative() {
                this.sortedAlternatives.push([ "", "", this.nextIdx ]);
                this.nextIdx++;
                this.update();
            },

            deleteAlternative(idx) {
                const [ removed ] = this.sortedAlternatives.splice(idx, idx + 1);
                if (removed[0] === this.defaultKey) {
                    // TODO: what if last one removed?
                    this.defaultKey = this.sortedAlternatives[0][0];
                }
                this.update();
            },

            editKey(idx, ev) {
                const key = this.sortedAlternatives[idx][0];
                this.$set(this.sortedAlternatives[idx], 0, ev.target.value);

                if (key === this.defaultKey) {
                    this.defaultKey = ev.target.value;
                }
                this.update();
            },

            editValue(idx, ev) {
                this.$set(this.sortedAlternatives[idx], 1, ev.target.value);
                this.update();
            },

            update() {
                const alternatives = {};
                for (const [ key, value ] of this.sortedAlternatives) {
                    if (alternatives[key]) {
                        return;
                    }
                    alternatives[key] = value;
                }
                this.$emit("input", {
                    default: this.defaultKey,
                    alternatives,
                });
            },
        },
    };
</script>

<style lang="css">
</style>
