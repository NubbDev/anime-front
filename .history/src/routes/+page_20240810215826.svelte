<script>
  import { onMount } from "svelte";
  import NavBar from "../components/navigation/NavBar.svelte";
  import { PageIndex, PageStore } from "$lib";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import { invoke } from "@tauri-apps/api/core";
  onMount(async () => {
    const data = await invoke("get_trending", { page: "" });
  });

</script>
<NavBar />
<SearchNoti/>