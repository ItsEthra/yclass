<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import toast from "svelte-french-toast";
    import { type Class, attached } from "../models";
    import UnknownField from "./fields/UnknownField.svelte";

    export let currentClass: Class;

    function onAddressKeyPress(e: KeyboardEvent) {
        if (e.code == "Enter") {
            const input = e.target as HTMLInputElement;
            invoke<number>("eval_address", { expr: input.value })
                .then((output: number) => (currentClass.address = output))
                .catch((err) => toast.error(err))
                .finally(() => input.blur());
        }
    }
</script>

<div class="p-4 w-full h-full">
    {#if !$attached}
        <div class="flex justify-center items-center w-full h-full">
            <h1 class="text-3xl text-warning">Attach to the process first</h1>
        </div>
    {:else}
        <div class="flex items-center gap-4 w-full">
            <input
                on:keypress={onAddressKeyPress}
                class="input input-primary input-sm"
                placeholder="Address"
                value={currentClass.address.toString(16).toUpperCase()}
                type="text"
            />
        </div>
        <div class="flex flex-col pt-4 overflow-scroll">
            {#each currentClass.properties as prop}
                <UnknownField
                    property={prop}
                    address={currentClass.address}
                    size={8}
                />
            {/each}
        </div>
    {/if}
</div>
