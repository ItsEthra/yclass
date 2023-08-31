<script lang="ts">
    import { faArrowRotateRight } from "@fortawesome/free-solid-svg-icons";
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";
    import Fa from "svelte-fa";
    import { type ProcessEntry, attached } from "../models";
    import toast from "svelte-french-toast";

    let dialog: HTMLDialogElement | null = null;
    let entries: ProcessEntry[] = [];

    let filterName = "";

    appWindow.onMenuClicked(async (event) => {
        if (event.payload == "process_attach") {
            filterName = "";

            entries = await invoke("list_processes");
            dialog?.showModal();
        }
    });

    let loading = false;
    const refresh = async () => {
        invoke("list_processes").then((d: any) => (entries = d));
        loading = true;
        setTimeout(() => (loading = false), 1000);
    };

    const shortenName = (name: string) => {
        if (name.length >= 30) return `${name.slice(0, 25)}...`;
        else return name;
    };

    const attach = (entry: ProcessEntry) => {
        invoke("attach", { pid: entry.id })
            .then(() => {
                toast.success(`Attached to the ${entry.name} - ${entry.id}`);
                attached.set(true);
            })
            .catch((err) => toast.error(err));
    };
</script>

<dialog bind:this={dialog} class="modal">
    <form method="dialog" class="modal-box">
        <div class="h-80 overflow-scroll flex flex-col">
            {#each entries as entry}
                {#if entry.name
                    .toLowerCase()
                    .includes(filterName.toLowerCase()) || !filterName}
                    <button
                        on:click={() => attach(entry)}
                        class="text-lg btn align-left hover:underline hover:cursor-pointer m-2 transition-all hover:btn-primary"
                    >
                        {shortenName(entry.name)} - {entry.id}
                    </button>
                {/if}
            {/each}
        </div>

        <div class="modal-action flex items-center gap-2">
            <!-- svelte-ignore a11y-autofocus -->
            <input
                bind:value={filterName}
                type="text"
                class="input input-bordered input-primary w-full"
                placeholder="Name"
                autofocus
            />
            <button class="btn btn-secondary" on:click={refresh} type="button">
                <span class:animate-spin={loading}>
                    <Fa icon={faArrowRotateRight} />
                </span>
            </button>
        </div>
    </form>
</dialog>
