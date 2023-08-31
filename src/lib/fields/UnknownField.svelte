<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { type Property } from "../../models";
    import { padString, arrayToInt, arrayToFloat } from "../../lib/utils";
    import { faExclamation } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    export let property: Property;
    export let address: number;
    export let size: number;

    let data: Uint8Array = new Uint8Array(8);
    let error = false;

    $: {
        invoke<Uint8Array>("read", {
            address: address + property.offset,
            count: size,
        })
            .then((d) => {
                error = false;
                data = new Uint8Array(d);
            })
            .catch((_) => {
                error = true;
                data = new Uint8Array(8);
            });
    }

    const byteColor = (byte: number) => {
        if (byte == 0) return `color: rgb(63, 63, 70)`;
        let [p1, p2, p3] = [
            byte & 0b111,
            (byte >> 3) & 0b111,
            (byte >> 6) & 0b111,
        ];
        let r = Math.max(p1 * 150, 100);
        let g = Math.max(p2 * 150, 100);
        let b = Math.max(p3 * 150, 100);
        return `color: rgb(${r}, ${g}, ${b})`;
    };
</script>

<div class="flex items-center">
    {#if error}
        <p class="mr-4 text-error">
            <Fa size="lg" icon={faExclamation} />
        </p>
    {/if}

    <p
        class="text-yellow-300 decoration-error underline-offset-2 decoration-2"
        class:underline={(address + property.offset) % 8 != 0}
    >
        {padString(property.offset.toString(16).toUpperCase(), 4)}
    </p>
    <p class="mx-3 text-green-400">
        {padString((address + property.offset).toString(16).toUpperCase(), 12)}
    </p>

    {#each data as byte}
        <p class="ml-2 select-none" style={byteColor(byte)}>
            {padString(byte.toString(16).toUpperCase(), 2)}
        </p>
    {/each}

    <p class="ml-3 text-cyan-200">{arrayToInt(data.buffer, 8, false)}</p>
    <p class="ml-3 text-red-400">{arrayToFloat(data.buffer, 8)}</p>
</div>

<style>
    p {
        font-size: 1.125rem;
    }
</style>
