<template>
    <section id="item">
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
    };
</script>

<style lang="css">
</style>
