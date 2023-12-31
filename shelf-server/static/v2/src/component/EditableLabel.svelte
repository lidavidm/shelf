<!--
Copyright 2023 David Li <li.davidm96@gmail.com>

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
<script>
    export let value;
    let editing = false;
    let originalValue = value;

    function handleKey(e) {
        // https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
        // Needs to be keydown; keyup won't have isComposing set on Enter
        if (e.isComposing || e.keyCode === 229) {
            return;
        }
        if (e.key === "Enter") {
            editing = false;
            originalValue = value;
        } else if (e.key === "Escape") {
            editing = false;
            value = originalValue;
        }
    }
</script>

<style>
</style>

<span>
    {#if editing}
        <input type="text" bind:value={value} on:keydown={handleKey} autofocus />
    {:else}
        <span on:click={(e) => (editing = !editing)}>{value}</span>
    {/if}
</span>
