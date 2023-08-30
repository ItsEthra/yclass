<script lang="ts">
    import ProcessAttachModal from "./lib/ProcessAttachModal.svelte";
    import ProjectData from "./lib/ProjectData.svelte";
    import Toolbar from "./lib/Toolbar.svelte";
    import { Toaster } from "svelte-french-toast";
    import ClassInspector from "./lib/ClassInspector.svelte";

    let selectedClassUuid: string | null = null;
</script>

<Toaster
    toastOptions={{
        style: "background-color: hsl(var(--b3)); color: hsl(var(--bc))",
    }}
/>
<ProcessAttachModal />

<div class="flex flex-col h-screen w-screen">
    <Toolbar />
    <div class="flex w-full h-full">
        <ProjectData bind:selectedClassUuid />
        {#if selectedClassUuid}
            <ClassInspector />
        {/if}
    </div>
</div>

<style>
</style>
