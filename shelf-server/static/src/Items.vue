<template>
    <section id="items">
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Status</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items">
                    <td>{{ item["name"]["alternatives"][item["name"]["default"]] }}</td>
                    <td>PTW</td>
                </tr>
            </tbody>
        </table>
    </section>
</template>

<script>
    export default {
        name: "items",
        data() {
            return {
                items: [],
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
    };
</script>

<style lang="css">
</style>
