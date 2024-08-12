<script lang="ts">
    import { PageIndex, type AnimeCardInfo } from "$lib";
  import { onMount } from "svelte";

    export let page: PageIndex;

    let animes: AnimeCardInfo[];
    let totalPages: number;

    onMount(async () => {
        switch(page) {

        }
    });
    
</script>

<main>

</main>

<style>
    main {
        height: 100vh;
        width: 100vw;
    }
</style>