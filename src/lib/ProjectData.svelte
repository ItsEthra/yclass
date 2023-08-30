<script lang="ts">
    import Fa from "svelte-fa";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
</script>

<div class="h-full border-r border-neutral-700 w-min flex flex-col p-4">
    <div class="flex items-center gap-4">
        <input
            type="text"
            placeholder="New class"
            class="input input-sm w-60"
        />
        <button class="btn btn-primary btn-square btn-sm">
            <Fa icon={faPlus} />
        </button>
    </div>
</div>
