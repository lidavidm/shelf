<script>
    import { formatISO } from "date-fns";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let id;
    export let label;
    export let placeholder;
    export let value;
    let error = null;

    function emit(value) {
        error = "";
        if (value === null) {
            dispatch("input", value);
        } else if (value.match(/^\d{4}-\d{2}-00$/)) {
            // YYYY-MM-00
            dispatch("input", value);
        } else {
            const date = new Date(value);
            if (isNaN(date)) {
                error = "Invalid date";
            } else {
                dispatch("input", value);
            }
        }
    }
</script>

<div>
    <label for={id}>{label}:</label>
    <input
        {id}
        type="text"
        {placeholder}
        value={value ? value : null}
        on:input={(e) => emit(e.target.value)} />
    {#if error}<span class="error">{error}</span>{/if}
    <button on:click={(e) => emit(null)}>Clear</button>
    <button on:click={(e) => emit(formatISO(new Date()))}>Now</button>
</div>
