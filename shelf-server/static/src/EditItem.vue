<template>
    <section id="item" v-if="data">
        <header>
            <h2>{{ data.name.alternatives[data.name.default] }} <span>({{ data.kind }})</span></h2>

            Added {{ data.added }}
        </header>
        <section>
            <label for="kind">Item Type</label>
            <select id="kind" v-model="data.kind">
                <option>Unknown</option>
                <option>Manga</option>
                <option>TV</option>
                <option>Film</option>
                <option>Book</option>
            </select>

            <label for="season">Season</label>
            <input id="season" v-model="data.season" />

            <label for="status">Status</label>
            <select id="status" v-model="data.status">
                <option>Planned</option>
                <option value="InProgress">In Progress</option>
                <option>Completed</option>
                <option value="OnHold">On Hold</option>
                <option>Dropped</option>
            </select>

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
        </section>

        <section>
            <table>
                <thead>
                    <tr>
                        <th>Language</th>
                        <th>Name</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(name, key) in data.name.alternatives">
                        <td><input type="text" v-model="key" /></td>
                        <td><input type="text" v-model="data.name.alternatives[key]" /></td>
                        <td><button>Delete</button></td>
                    </tr>
                    <tr>
                        <td><button>Add Name</button></td>
                    </tr>
                </tbody>
            </table>
            <label for="name-default">Default</label>
            <select id="name-default" v-model="data.name.default">
                <option v-for="(value, key) in data.name.alternatives" :value="key">{{ key }}</option>
            </select>
        </section>

        <button v-on:click="save">Save</button>
        <button v-on:click="cancel">Cancel</button>
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
                this.$emit("done");
            },
            cancel() {
                this.$emit("done");
            },
        },
    };
</script>

<style lang="css">
    #item {
        position: fixed;
        border: 1px solid #000;
        top: 1em;
        background: #FFF;
        left: 2em;
        right: 2em;
        box-shadow: 0px 2px 1px rgba(0,0,0,0.5);
        padding: 1em;
    }
</style>
