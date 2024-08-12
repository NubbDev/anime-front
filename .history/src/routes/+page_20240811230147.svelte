<script lang="ts">
  import NavBar from "../components/navigation/NavBar.svelte";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import FillerSpace from "../components/layout/FillerSpace.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";

</script>
<NavBar />
<SearchNoti/>

<div class="show">
  <HomePage />
</div>

<FillerSpace height="10vh" />

<style>
  div.show {
    opacity: 1;
  }
</style>