<template>
    <section id="items">
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Status</th>
                    <th>Rating</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items">
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
                    <td>{{ item["rating"] === null ? "-" : item["rating"] }}</td>
                </tr>
            </tbody>
        </table>

        <EditItem v-if="editing" v-bind:item="editing" v-on:done="doneEditing" />
    </section>
</template>

<script>
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
                      console.log(items);
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
