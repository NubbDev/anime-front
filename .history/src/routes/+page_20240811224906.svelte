<script lang="ts">
  import { onMount } from "svelte";
  import NavBar from "../components/navigation/NavBar.svelte";
  import { PageIndex, PageStore, type AnimeCardInfo } from "$lib";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AnimeSections from "../components/pages/main/AnimeSections.svelte";
  import BannerAnime from "../components/pages/main/BannerAnime.svelte";
  import FillerSpace from "../components/layout/FillerSpace.svelte";
  import BannerCarousel from "../components/pages/main/BannerCarousel.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";

</script>
<NavBar />
<SearchNoti/>

<HomePage />

<FillerSpace height="10vh" />