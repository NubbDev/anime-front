<script lang="ts">
    import { onMount } from "svelte";
    import { PageIndex, PageStore } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
    
    let TrendingAnimes = [];
    
    onMount(async () => {
        const data = await invoke("get_trending", { page: "" });
        TrendingAnimes = data;
    });
</script>