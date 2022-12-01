<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window';
    import MainWindow from './lib/MainWindow.svelte';
    import ProcessSelector from './lib/ProcessSelector.svelte';

    let currentWindow = appWindow.label
</script>

{#if currentWindow == 'main' }
    <MainWindow />
{:else if currentWindow == 'wnd_process_select' }
    <ProcessSelector />
{/if}

