<script lang="ts">
  import { ClientWSMessageType, GOGOServers, History, PageIndex, Websocket, type GoGoAnimeInfo } from "$lib";

    export let delay: number;
    export let sub: {
        id: string,
        number: number,
    };
    export let dub: {
        id: string,
        number: number,
    } | undefined;

    const handleEpisodeSend = async () => {
        const subId = sub.id
        const dubId = dub ? dub.id : null

        const typeSub = false
        await Websocket.send(
            ClientWSMessageType.GetEpisode,
            {
                id: typeSub ? subId : dubId ?? subId,
                server: GOGOServers.Gogo
            }
        )
        History.push(PageIndex.PLAYER)
    }
    
</script>

<button class="episode-wrapper" style="animation-delay: {delay}s;" on:click={handleEpisodeSend}>
    <h6>
        Episode {sub.number}
    </h6>
    <div>
        <span>
            Sub
        </span>
        <span>
            {dub ? ' | Dub' : ''}
        </span>
    </div>
</button>

<style>
    button {
        width: 100%;
        padding: 0.5rem 1rem;
        box-sizing: border-box;
        animation: slide-in-blurred-left 0.6s cubic-bezier(0.230, 1.000, 0.320, 1.000) both;
        border: none;
        border-radius: 0.25rem;
        background: rgba(119, 119, 119, 0.15);
    }

    h6 {
        margin: 0;
        color: var(--headings);
        font-size: 5vw;
    }

    @keyframes slide-in-blurred-left{0%{transform:translateX(-1000px) scaleX(2.5) scaleY(.2);transform-origin:100% 50%;filter:blur(40px);opacity:0}100%{transform:translateX(0) scaleY(1) scaleX(1);transform-origin:50% 50%;filter:blur(0);opacity:1}}
</style>