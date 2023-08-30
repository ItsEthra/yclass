<script lang="ts">
    import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
    import { invoke } from "@tauri-apps/api";
    import Fa from "svelte-fa";
    import toast from "svelte-french-toast";

    function onAddressKeyPress(e: KeyboardEvent) {
        if (e.code == "Enter") {
            const input = e.target as HTMLInputElement;
            invoke<number>("eval_address", { expr: input.value })
                .then(
                    (output: number) =>
                        (input.value = `${output.toString(16).toUpperCase()}`)
                )
                .catch((err) => toast.error(err))
                .finally(() => input.blur());
        }
    }
</script>

<div class="p-4 w-full">
    <div class="flex items-center gap-4 w-full">
        <Fa size="lg" icon={faChevronRight} />
        <input
            on:keypress={onAddressKeyPress}
            class="input input-primary"
            placeholder="Address"
            type="text"
        />
    </div>
</div>

