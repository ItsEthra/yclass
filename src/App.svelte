<script lang="ts">
    import ProcessAttachModal from "./lib/ProcessAttachModal.svelte";
    import ProjectData from "./lib/ProjectData.svelte";
    import Toolbar from "./lib/Toolbar.svelte";
    import { Toaster } from "svelte-french-toast";
</script>

<Toaster />
<ProcessAttachModal />

<div class="flex flex-col h-screen w-screen">
    <Toolbar />
    <ProjectData />
</div>

<style>
</style>
