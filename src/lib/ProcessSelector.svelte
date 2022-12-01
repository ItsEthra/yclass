<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let filter = ''
    let processList = []

    onMount(async () => {
        processList = await invoke('fetch_all_processes', { filter })
    })
</script>

<h1>Selecting a process</h1>
