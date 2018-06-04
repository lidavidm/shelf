<template>
    <div id="app">
        <h1>{{ msg }}</h1>
        <Items />
    </div>
</template>

<script>
    import Items from "./Items.vue";

    export default {
        name: "app",
        data() {
            return {
                msg: "Welcome to Your Vue.js App!",
            };
        },
        components: {
            Items,
        },
    };
</script>

<style lang="css">
    #app {
        color: #56b983;
    }
</style>
