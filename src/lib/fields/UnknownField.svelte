<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { type Property } from "../../models";
    import { padString, arrayToInt, arrayToFloat } from "../../lib/utils";

    export let property: Property;
    export let address: number;
    export let size: number;

    let data: Uint8Array = new Uint8Array(8);

    $: {
        invoke<Uint8Array>("read", {
            address: address + property.offset,
            count: size,
        })
            .then((d) => (data = new Uint8Array(d)))
            .catch((_) => (data = new Uint8Array(8)));
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

<div class="flex">
    <p class="text-yellow-300">{padString(property.offset.toString(16), 4)}</p>
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
