<script lang="ts">
    import Fa from "svelte-fa";
    import { faPen, faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { project_data } from "../models";

    export let selectedClassUuid: string | null = null;

    let editingClassNameUuid: string | null;
    let newClassName = "";

    function toggleClassSelection(uuid: string) {
        if (selectedClassUuid == uuid) selectedClassUuid = null;
        else selectedClassUuid = uuid;
    }

    function handleNewClassKeydown(e: KeyboardEvent) {
        if (e.key == "Enter") createNewClass();
    }
    function handleEditNameKeydown(e: KeyboardEvent) {
        if (e.key == "Enter") editingClassNameUuid = null;
    }

    function createNewClass() {
        if (newClassName.length != 0) {
            project_data.update((data) => {
                data.newClass(newClassName);
                return data;
            });
            newClassName = "";
        }
    }

    function removeClass(uuid: string) {
        project_data.update((data) => {
            console.log(data.classes, uuid);
            let idx = data.classes.indexOf(
                data.classes.find((c) => c.uuid == uuid)!
            );
            data.classes.splice(idx, 1);
            return data;
        });

        if (selectedClassUuid == uuid) selectedClassUuid = null;
    }
</script>

<div class="h-full border-r border-neutral-700 w-min flex flex-col p-4">
    <h1 class="text-center text-2xl mb-4">Class list</h1>
    <div class="flex items-center gap-4">
        <input
            type="text"
            bind:value={newClassName}
            on:keydown={handleNewClassKeydown}
            placeholder="New class"
            class="input input-bordered input-sm w-60"
        />
        <button
            class="btn btn-primary btn-sm btn-square"
            on:click={createNewClass}
        >
            <Fa icon={faPlus} size="lg" />
        </button>
    </div>
    <div class="divider" />

    <div class="w-full h-full flex flex-col gap-2">
        {#each $project_data.classes as item}
            {#if item.uuid == editingClassNameUuid}
                <!-- svelte-ignore a11y-autofocus -->
                <input
                    type="text"
                    class="input input-sm"
                    bind:value={item.name}
                    on:keydown={handleEditNameKeydown}
                    on:focusout={() => (editingClassNameUuid = null)}
                    autofocus
                />
            {:else}
                <div class="flex w-full gap-2 group">
                    <button
                        on:click={() => toggleClassSelection(item.uuid)}
                        class="h-8 rounded-xl text-left pl-4 select-none flex-auto transition-all"
                        class:btn-primary={selectedClassUuid == item.uuid}
                        >{item.name}</button
                    >
                    <button
                        on:click={() => (editingClassNameUuid = item.uuid)}
                        class="btn btn-accent btn-sm hidden group-hover:flex btn-square"
                    >
                        <Fa icon={faPen} />
                    </button>
                    <button
                        on:click={() => removeClass(item.uuid)}
                        class="btn btn-error btn-sm hidden group-hover:flex btn-square"
                    >
                        <Fa icon={faTrash} />
                    </button>
                </div>
            {/if}
        {/each}
    </div>
</div>
