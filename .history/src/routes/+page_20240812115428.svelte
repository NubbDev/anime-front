<script lang="ts">
  import NavBar from "../components/navigation/NavBar.svelte";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import FillerSpace from "../components/layout/FillerSpace.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";
  import { DisplayStateCSS } from '$lib'

  const PageState = {
    home: {
      display: DisplayStateCSS.SHOW,
      show: true
    }
  }

</script>
<NavBar />
<SearchNoti/>

<div class="{PageState.home.display == DisplayStateCSS.SHOW ? 'show' : 'hide'}">
  <HomePage />
</div>

<FillerSpace height="10vh" />

<style>
  div.show {
    opacity: 1;
  }
  div.hide {
    opacity: 0;
  }
</style>