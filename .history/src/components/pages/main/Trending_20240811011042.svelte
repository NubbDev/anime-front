<script lang="ts">
    import { onMount } from "svelte";
    import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
    import { invoke } from "@tauri-apps/api/core";
    import AnimeSections from "./AnimeSections.svelte";
    
</script>
