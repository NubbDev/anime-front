<script lang="ts">
    import type { AnimeCardInfo } from "$lib";
    export let anime: AnimeCardInfo;
</script>

<section>

</section>

<style>
    section {
        width: 100vw;
        height: 25vh;
        background-image: url({anime.bannerImage});
        background-size: cover;
        background-position: center;
        position: absolute;
    }
</style>