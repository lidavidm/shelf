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
        margin: 1rem 0;
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
