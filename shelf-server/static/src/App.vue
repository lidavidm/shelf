<template>
    <main id="app">
        <h1>shelf</h1>
        <router-view />
    </main>
</template>

<script>
    export default {
        name: "app",
        data() {
            return {
            };
        },
    };
</script>

<style lang="css">
    :root {
        --theme-base: #FFF;
        --theme-1: #e07a5f;
        --theme-2: #3d405b;
        --theme-3: #81b29a;
        --theme-4: #f2cc8f;
        --theme-text: #000;

        --theme-completed: var(--theme-3);
        --theme-dropped: var(--theme-1);
        --theme-onhold: var(--theme-4);
        --theme-inprogress: var(--theme-2);
        --theme-planned: var(--theme-text);

        --theme-positive: var(--theme-3);
        --theme-danger: var(--theme-1);
    }

    body {
        background: var(--theme-base);
        color: var(--theme-text);
        font-family: "Linux Libertine O";
        font-size: 20px;
        text-rendering: optimizeLegibility;
    }

    #app {
        width: 60em;
        margin: 0 auto;
    }

    #app h1 {
        font-size: 3em;
        font-weight: normal;
    }

    h1, h2, h3 {
        font-family: "Linux Biolinum O", sans-serif;
    }

    code {
        font-family: "Linux Libertien Mono O", monospace;
    }

    button {
        background: var(--theme-4);
        border: 0;

        padding: 0.25em 1em;

        font-family: "Linux Biolinum O", sans-serif;
        font-size: 1.25rem;

        cursor: pointer;
    }

    button.positive {
        background: var(--theme-positive);
    }

    button.danger {
        background: var(--theme-danger);
    }

    input[type=text] {
        background: none;
        border: 0;
        border-bottom: 1px solid var(--theme-2);

        padding: 0;
        margin: 0;

        font-family: "Linux Biolinum O", sans;
        font-size: 1.2rem;
    }
</style>
