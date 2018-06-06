<template>
    <section id="items">
        <table>
            <thead>
                <tr>
                    <th>Type</th>
                    <th>Name</th>
                    <th>Status</th>
                    <th>Entries</th>
                    <th>Rating</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items">
                    <td>{{ item["kind"] }}</td>
                    <td>
                        {{ item["name"]["alternatives"][item["name"]["default"]] }}
                        <button
                            class="edit"
                            v-on:click="edit(item['key'])"
                        >
                            Edit
                        </button>
                    </td>
                    <td>{{ item["status"] }}</td>
                    <td>{{ item["entries"].filter(e => e.completed).length }} / {{ item["entries"].length }}</td>
                    <td>{{ item["rating"] === null ? "-" : item["rating"] }}</td>
                </tr>
            </tbody>
        </table>

        <EditItem v-if="editing" v-bind:item="editing" v-on:done="doneEditing" />
    </section>
</template>

<script>
    import firstBy from "thenby";
    import EditItem from "./EditItem";

    export default {
        name: "items",
        data() {
            return {
                items: [],
                editing: null,
            };
        },
        mounted() {
            window.fetch("/item")
                  .then(r => r.json())
                  .then((items) => {
                      items.sort(firstBy(v => v.kind)
                          .thenBy(v => v.name.alternatives[v.name.default]));
                      this.items = items;
                  });
        },
        methods: {
            edit(key) {
                this.editing = key;
            },

            doneEditing() {
                this.editing = null;
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
    }

    th {
        border-bottom: 1px solid black;
        text-align: left;
    }

    .edit {
        float: right;
    }
</style>
