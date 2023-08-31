<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import toast from "svelte-french-toast";
    import { type Class, attached } from "../models";
    import UnknownField from "./fields/UnknownField.svelte";

    export let currentClass: Class;

    function tryParseAddress(input: HTMLInputElement) {
        if (input.value.length == 0) {
            input.value = currentClass.address.toString(16).toUpperCase();
            return;
        }

        invoke<number>("eval_address", { expr: input.value })
            .then((output: number) => {
                currentClass.address = output;
                input.value = output.toString(16).toUpperCase();
            })
            .catch((err) => {
                input.value = currentClass.address.toString(16).toUpperCase();
                toast.error(err);
            })
            .finally(() => input.blur());
    }

    function onAddressKeyPress(e: KeyboardEvent) {
        if (e.code == "Enter") tryParseAddress(e.target as HTMLInputElement);
    }

    function onAddressUnfocus(e: FocusEvent) {
        let input = e.target as HTMLInputElement;
        input.value = currentClass.address.toString(16).toUpperCase();
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
                on:focusout={onAddressUnfocus}
                class="input input-primary input-sm"
                placeholder="Address"
                type="text"
            />
            <p class="text-xl">- {currentClass.name}</p>
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
