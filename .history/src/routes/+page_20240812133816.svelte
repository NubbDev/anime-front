<script lang="ts">
  import { DisplayStateCSS, PageIndex, PageStore } from '$lib'
  import NavBar from "../components/navigation/NavBar.svelte";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import FillerSpace from "../components/layout/FillerSpace.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";
  import MorePage from "../components/pages/main/MorePage.svelte";

  const PageState = {
    home: {
      display: DisplayStateCSS.SHOW,
      show: true
    },
    releases: {
      display: DisplayStateCSS.HIDE,
      show: false
    }
  }

  PageStore.subscribe(value => {
    switch (value) {
      case PageIndex.HOME:
        PageState.home.display = DisplayStateCSS.SHOW;
        
        break;
      default:
        PageState.home.display = DisplayStateCSS.HIDE;
        break;
    }
  })

</script>
<NavBar />
<SearchNoti/>

<div class="home {PageState.home.display == DisplayStateCSS.SHOW ? 'show' : 'hide'}">
  <HomePage />
</div>

<div class="">
  <MorePage page={PageIndex.RELEASES} />
</div>

<FillerSpace height="10vh" />

<style>
  div.show {
    opacity: 1;
  }
  div.hide {
    opacity: 0;
  }

  .home {
    transition: opacity 0.5s;
  }

</style>