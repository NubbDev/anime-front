<script>
  import { onMount } from "svelte";
  import NavBar from "../components/navigation/NavBar.svelte";
  import { PageIndex, PageStore } from "$lib";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  onMount(() => {
    
    PageStore.set(PageIndex.HOME);
  });

</script>
<NavBar />
<SearchNoti/>