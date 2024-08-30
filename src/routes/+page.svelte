<script lang="ts">
  import { History, PageIndex, BodyScroll, AppStateStore, AppStates, Websocket, Page, ServerWSMessageType, TimeElapsedStore, AnimeSelectedStore, type AnimeInfo, AnimePlayerStore } from '$lib'
  import NavBar from "../components/navigation/NavBar.svelte";
  import SearchNoti from "../components/navigation/SearchNoti.svelte";
  import HomePage from "../components/pages/main/HomePage.svelte";
  import MorePage from "../components/pages/main/MorePage.svelte";
  import { get } from 'svelte/store';
  import { onMount } from 'svelte';

  import LoadingScreen from '../components/layout/LoadingScreen.svelte';
  import Search from '../components/layout/Search.svelte';
  import AnimeInfoPage from '../components/pages/anime_info/AnimeInfo.svelte';
  import type { GogoEpisodeLink } from '$lib/misc/types';
  import PlayerPage from '../components/pages/player/PlayerPage.svelte';

  let appWebSocket: Websocket;

  let div: Element

  const PageState = {
    home: {
      display: 'show',
      show: true
    },
    season: {
      display: 'hide',
      show: false
    },
    trending: {
      display: 'hide',
      show: false
    },
    popular: {
      display: 'hide',
      show: false
    },
    top: {
      display: 'hide',
      show: false
    },
    anime: {
      display: 'hide',
      show: false
    },
    releases: {
      display: 'hide',
      show: false
    },
    downloads: {
      display: 'hide',
      show: false
    },
    profile: {
      display: 'hide',
      show: false
    },
    setting: {
      display: 'hide',
      show: false
    },
    search: {
      display: 'hide',
      show: false
    },
    about: {
      display: 'hide',
      show: false
    },
    player: {
      display: 'hide',
      show: false
    },
    auth: {
      display: 'hide',
      show: false
    },
    notFound: {
      display: 'hide',
      show: false
    }
  }

  const showPage = (key: keyof typeof PageState) => {
    PageState[key].show = true;
    setTimeout(() => {
      PageState[key].display = 'show';
    }, 10)
    return 
  }

  const hidePage = (key: keyof typeof PageState) => {
    PageState[key].display = 'hide';
    setTimeout(() => {
      PageState[key].show = false;
    }, 650)
    return
  }

  History.onUpdate(async value => {
    // console.log("Current", PageIndex[value])
    // console.log("Last", PageIndex[History.lastPage])
    if (div != undefined) div.scrollTop = 0;
    handle(value, showPage);
    if (History.lastPage == value || History.lastPage == null) return;
    handle(History.lastPage, hidePage);
  })

  const handle = (page: PageIndex, callback: (key: keyof typeof PageState) => void) => {
    switch(page) {
      case PageIndex.HOME: callback("home");
        break
      case PageIndex.TRENDING: callback("trending");
        break
      case PageIndex.RELEASES: callback("releases");
        break
      case PageIndex.DOWNLOADS: callback("downloads");
        break
      case PageIndex.ANIME: callback("anime");
        break
      case PageIndex.PROFILE: callback("profile");
        break
      case PageIndex.SETTINGS: callback("setting");  
        break;
      case PageIndex.POPULAR: callback("popular");
        break
      case PageIndex.TOP: callback("top");
        break
      case PageIndex.SEASON: callback("season");
        break
      case PageIndex.ABOUT: callback("about");
        break
      case PageIndex.PLAYER: callback("player");
        break
      case PageIndex.SEARCH: callback("search");
        break
      case PageIndex.AUTHENCATION: callback("auth");
        break
      case PageIndex.NOT_FOUND: callback("notFound");
        break
    }
  }

  onMount(async () => {
    appWebSocket = await Websocket.connect();
    await History.init();

    appWebSocket.onConnect(() => {
      AppStateStore.set(AppStates.CONNECTED);
    })
    appWebSocket.onDisconnect(async () => {
      AppStateStore.set(AppStates.CONNECTING);
      await History.clear();
    })

    window.addEventListener('keydown', async (e) => {
      if (e.ctrlKey && e.key === 'r') {
        e.preventDefault();
        await Websocket.disconnect()
      }
    })

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

  let appState = get(AppStateStore)
  AppStateStore.subscribe(value => {
    appState = value;
  })

  Websocket.addListener<AnimeInfo>(ServerWSMessageType.AnimeData, message => {
    AnimeSelectedStore.set(message);
    History.push(PageIndex.ANIME);
  })

  Websocket.addListener<GogoEpisodeLink>(ServerWSMessageType.EpisodeLink, message => {
    AnimePlayerStore.set(message);
  })

</script>

<LoadingScreen />
{#if appState !== AppStates.CONNECTING}
  <Search/>
  <NavBar />
  <SearchNoti />
  {#if PageState.home.show}
    <div class="home {PageState.home.display}" >
      <HomePage/>
    </div>
  {/if}
  {#if PageState.trending.show}
    <div class="extra {PageState.trending.display}" >
      <MorePage page={PageIndex.TRENDING}/>
    </div>
  {/if}
  {#if PageState.popular.show}
    <div class="extra {PageState.popular.display}" >
      <MorePage page={PageIndex.POPULAR}/>
    </div>
  {/if}
  {#if PageState.top.show}
    <div class="extra {PageState.top.display}" >
      <MorePage page={PageIndex.TOP}/>
    </div>
  {/if}
  {#if PageState.season.show}
    <div class="extra {PageState.season.display}" >
      <MorePage page={PageIndex.SEASON}/>
    </div>
  {/if}
  {#if PageState.search.show}
    <div class="extra {PageState.search.display}">
      <MorePage page={PageIndex.SEARCH}/>
    </div>
  {/if}
  {#if PageState.anime.show}
    <div class="anime-info {PageState.anime.display}">
      <AnimeInfoPage/>
    </div>
  {/if}
  {#if PageState.player.show}
  <div class="anime-player show">
    <PlayerPage/>
  </div>
  {/if}

{/if}


<style>
  div {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 1;
    height: 100vh;
    width: 100vw;
  }

  .home {
    transition: opacity 0.5s ease-in-out;
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

  .anime-info {
    z-index:11;
    position: fixed;
    transition: transform 0.5s ease-in-out;
  } 
  .anime-info.hide {
    transform: translateY(100%);
  }
  .anime-info.show {
    transform: translateY(0);
  }

  .anime-player {
    z-index: 9;
    position: fixed;
    transition: opacity 0.5s ease-in-out;
  }
  .anime-player.hide {
    opacity: 0;
  }
  .anime-player.show {
    opacity: 1;
  }

</style>