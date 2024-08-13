<script lang="ts">
  import { PageIndex, PageStore, PageHistory, BodyScroll } from '$lib'
  import NavBar from "../components/navigation/NavBar.svelte";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import FillerSpace from "../components/layout/FillerSpace.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";
  import MorePage from "../components/pages/main/MorePage.svelte";
  import { get } from 'svelte/store';
  import { onMount } from 'svelte';

  let history: PageIndex[] = [];
  let div: Element
  const PageState = {
    home: {
      display: true,
      show: true
    },
    season: {
      display: false,
      show: false
    },
    trending: {
      display: false,
      show: false
    },
    popular: {
      display: false,
      show: false
    },
    top: {
      display: false,
      show: false
    },
    releases: {
      display: false,
      show: false
    },
    downloads: {
      display: false,
      show: false
    },
    profile: {
      display: false,
      show: false
    },
    setting: {
      display: false,
      show: false
    }
  }

  PageStore.subscribe(value => {
    if (div != undefined) {
      div.scrollTop = 0;
    }
    let historySaved = get(PageHistory);
    if (value === historySaved[historySaved.length - 1]) {
      history = historySaved;
      return;
    };
    let fadeInTimeout = 1;
    let fadeOutTimeout = 500;

    switch (value) {
      case PageIndex.HOME: {
        PageState.home.show = true;
        setTimeout(() => {
          PageState.home.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.RELEASES: {
        PageState.releases.show = true;
        setTimeout(() => {
          PageState.releases.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.DOWNLOADS: {
        PageState.downloads.show = true;
        setTimeout(() => {
          PageState.downloads.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.PROFILE: {
        PageState.profile.show = true;
        setTimeout(() => {
          PageState.profile.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.SEASON: {
        PageState.season.show = true;
        setTimeout(() => {
          PageState.season.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.SETTINGS: {
        PageState.setting.show = true;
        setTimeout(() => {
          PageState.setting.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.TRENDING: {
        PageState.trending.show = true;
        setTimeout(() => {
          PageState.trending.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.POPULAR: {
        PageState.popular.show = true;
        setTimeout(() => {
          PageState.popular.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }
      case PageIndex.TOP: {
        PageState.top.show = true;
        setTimeout(() => {
          PageState.top.display = true;
        }, fadeInTimeout)
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, fadeOutTimeout);
        break;
      }

      default:
        hidePage(value);
        setTimeout(() => {
          disablePage(value);
        }, 500);
        break;
    }

    history = [...history, value];
    PageHistory.update(prev => {
      return [...prev, value];
    })
  })

  const hidePage = (nextPage: PageIndex) => {
    
    let lastPage = history[history.length - 1];
    switch (lastPage) {
      case PageIndex.HOME: {
        PageState.home.display = false;
        break;
      }
      case PageIndex.TRENDING: {
        PageState.trending.display = false;
        break;
      }
      case PageIndex.ABOUT: {
        // PageState..display = false;
        break;
      }
      case PageIndex.RELEASES: {
        PageState.releases.display = false;
        break;
      }
      case PageIndex.DOWNLOADS: {
        PageState.downloads.display = false;
        break;
      }
      case PageIndex.PROFILE: {
        PageState.profile.display = false;
        break;
      }
      case PageIndex.SETTINGS: {
        PageState.setting.display = false;
        break;
      }
      case PageIndex.PLAYER: {
        // PageState.trending.display = false;
        break;
      }
      case PageIndex.SEARCH: {
        // PageState.trending.display = false;
        break;
      }
      case PageIndex.AUTHENCATION: {
        // PageState.trending.display = false;
        break;
      }
      case PageIndex.NOT_FOUND: {
        // PageState.trending.display = false;
        break;
      }
      case PageIndex.POPULAR: {
        PageState.popular.display = false;
        break;
      }
      case PageIndex.TOP: {
        PageState.top.display = false;
        break;
      }
      case PageIndex.SEASON: {
        // PageState.trending.display = false;
        break;
      }
    }
  }

  const disablePage = (nextPage: PageIndex) => {
    let lastPage = history[history.length - 2];
    switch (lastPage) {
      case PageIndex.HOME: {
        PageState.home.show = false;
        break;
      }
      case PageIndex.TRENDING: {
        PageState.trending.show = false;
        break;
      }
      case PageIndex.ABOUT: {
        // PageState..show = false;
        break;
      }
      case PageIndex.RELEASES: {
        PageState.releases.show = false;
        break;
      }
      case PageIndex.DOWNLOADS: {
        PageState.downloads.show = false;
        break;
      }
      case PageIndex.PROFILE: {
        PageState.profile.show = false;
        break;
      }
      case PageIndex.SETTINGS: {
        PageState.setting.show = false;
        break;
      }
      case PageIndex.PLAYER: {
        // PageState.trending.show = false;
        break;
      }
      case PageIndex.SEARCH: {
        // PageState.trending.show = false;
        break;
      }
      case PageIndex.AUTHENCATION: {
        // PageState.trending.show = false;
        break;
      }
      case PageIndex.NOT_FOUND: {
        // PageState.trending.show = false;
        break;
      }
      case PageIndex.POPULAR: {
        PageState.popular.show = false;
        break;
      }
      case PageIndex.TOP: {
        PageState.top.show = false;
        break;
      }
      case PageIndex.SEASON: {
        // PageState.trending.show = false;
        break;
      }
    }
  }

  onMount(() => {
    div = document.body.getElementsByClassName('main_body')[0]
    let lastScroll = 0;
    BodyScroll.set(div.scrollTop)
    
    const handleBodyScroll = () => {
      if (lastScroll !== div.scrollTop) {
        lastScroll = div.scrollTop;
        BodyScroll.set(lastScroll)
      }
      requestAnimationFrame(handleBodyScroll)
    }

    handleBodyScroll()
  })

</script>
<NavBar />
<SearchNoti/>

<svelte:body />

{#if PageState.home.show}
  <div class="home {PageState.home.display ? 'show' : 'hide'}" >
    <HomePage />
  </div>
{/if}
{#if PageState.trending.show}
  <div class="extra {PageState.trending.display ? 'show' : 'hide'}" >
    <MorePage page={PageIndex.TRENDING} />
  </div>
{/if}
{#if PageState.popular.show}
  <div class="extra {PageState.popular.display ? 'show' : 'hide'}" >
    <MorePage page={PageIndex.POPULAR} />
  </div>
{/if}
{#if PageState.top.show}
  <div class="extra {PageState.top.display ? 'show' : 'hide'}" >
    <MorePage page={PageIndex.TOP} />
  </div>
{/if}
<!-- {#if PageState.trending.show}
  <div class="extra {PageState.trending.display ? 'show' : 'hide'}" >
    <MorePage page={PageIndex.TRENDING} />
  </div>
{/if} -->




<style>
  div {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 1;
  }
  /* div.show {
    opacity: 1;
  }
  div.hide {
    opacity: 0;
  } */

  .home {
    transition: opacity 0.5s;
  }

  .home.hide {
    opacity: 0;
  }
  .home.show {
    opacity: 1;
  }
  .extra {
    transition: transform 0.5s;
  }
  .extra.hide {
    transform: translateX(100%);
  }
  .extra.show {
    transform: translateX(0);
  }

</style>