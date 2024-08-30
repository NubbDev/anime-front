
<script lang="ts">
    import { ClientWSMessageType, PageIndex, ServerWSMessageType, Websocket, type AnimeCardInfo, History, AppSearchStore } from "$lib";
    import SearchWrapper from "./SearchWrapper.svelte";
    import type { KeyboardEventHandler } from "svelte/elements";

    let searchValue = "";
    let searchResults: { anime: AnimeCardInfo, display: 'show' | 'hide' | 'hide-back' }[] = [];
    let enterKeyPressed = false;

    const searchBackdrop = {
        show: false,
        display: false
    }

    Websocket.addListener<{list: AnimeCardInfo[]}>(ServerWSMessageType.SearchResult, ({list}) => {

        if (searchResults.length == 0) {
            searchResults = list.map(result => ({ anime: result, display: "hide" }));
            for (let i = 0; i < searchResults.length; i++) {
                setTimeout(() => {
                    searchResults[i].display = "show";
                }, (i + 1) * 50);
            }
            return;
        }

        let timer = 0;
        for (let i = 0; i < searchResults.length; i++) {
            timer += 60;
            setTimeout(() => {
                searchResults[i].display = "hide-back";
            }, (i + 1) * 50);
        }

        setTimeout(() => {
            searchResults = [];
            setTimeout(() => {
                searchResults = list.map(result => ({ anime: result, display: "hide" }));
                for (let i = 0; i < searchResults.length; i++) {
                    setTimeout(() => {
                        searchResults[i].display = "show";
                    }, (i + 1) * 50);
                }
            }, 0.0000000001)
        }, timer)
    })

    AppSearchStore.subscribe(value => {
        if (value) {
            searchBackdrop.show = true;
            setTimeout(() => {
                searchBackdrop.display = true;
            }, 1)
        } else {
            searchBackdrop.display = false;
            setTimeout(() => {
                searchBackdrop.show = false;
            }, 500)
        }

        searchValue = "";
        searchResults = [];
    })

    const typeSearch = async (e: Event) => {
        searchValue = (e.target as HTMLInputElement).value;
        await Websocket.send(ClientWSMessageType.Search, {
            search: searchValue,
            limit: 5,
            currentPage: 1
        })
    }
    const keyboardSubmit: KeyboardEventHandler<HTMLInputElement> = async (e) => {
        if (!enterKeyPressed && e.key.toLowerCase() == "enter") {
            enterKeyPressed = true;
            await submitSearch();
            History.push(PageIndex.SEARCH);
            setTimeout(() => {
                AppSearchStore.set(false);
            }, 1000)
        }
    }
    const submitSearch = async () => {
        await Websocket.send(ClientWSMessageType.SearchPage, {
            search: searchValue,
            limit: 50,
            currentPage: 1
        })
        enterKeyPressed = false;
    }

    const displaySearchBar = () => {}

</script>

{#if searchBackdrop.show}
    <section class="{searchBackdrop.display ? 'show' : 'hide'}">
        <div class="search_bar">
            <input type="text" placeholder="Search..." on:input={typeSearch} value={searchValue} on:keypress={keyboardSubmit}/>
            <button type="submit" on:click={submitSearch}>Search</button>
        </div>
        <div class="search_results">
            {#each searchResults as result, i}
                <SearchWrapper anime={result.anime} display={result.display}/>
            {/each}
        </div>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="backdrop" on:click={() => AppSearchStore.set(false)}/>
    </section>
{/if}

<style>
    section {
        position: fixed;
        top: 6.5vh;
        height: 2.5rem;
        transition: width 0.5s ease;
        right: 5vw;
        z-index: 100;
    }
    section.hide {
        width: 0;
    }
    section.show {
        width: 90vw;
    }

    input {
        background: rgba(255, 255, 255, 0.5);
        border-color: #fff;
        opacity: 1;
        color: #fff;
        width: 100%;
        height: 100%;
        padding: 0.5rem;
        border-radius: 1rem;
    }

    input:focus {
        border: #fff;
    }

    section.show input {
        opacity: 1;
    }
    section.hide input {
        opacity: 0;
    }

    button {
        position: absolute;
        right: 0.5rem;
        top: 50%;
        transform: translateY(-50%);
        height: 65%;
        width: 5rem;
        background: rgba(255, 255, 255, 0.65);
        border-radius: 2.5rem;
        border: none;
        transition: opacity 0.65s ease;
    }

    .backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.5);
        z-index: -1;
        backdrop-filter: blur(10px);
        transition: opacity 0.5s ease;
    }

    section.show .backdrop {
        opacity: 1;
    }
    section.hide .backdrop {
        opacity: 0;
    }

    section.show button {
        opacity: 1;
    }

    section.hide button {
        opacity: 0;
    }
    .search_bar {
        position: relative;
        display: flex;
        gap: 1em;
    }

    .search_results {
        position: absolute;
        top: 2.5rem;
        left: 0;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    
</style>