// place files you want to import through the `$lib` alias in this folder.

import CachedStore from './classes/CachedStore'
import Websocket from './classes/Websocket'
import History from './classes/History'
import { AnimeGenres } from './misc/enums'

export {
    BodyScroll,
    AppStateStore,
    AppSearchStore,
    TimeElapsedStore,
    AnimeSelectedStore,
    SearchPageStore,
    AnimePlayerStore
} from './store'

export {
    PageIndex,
    MediaTrendSort,
    MediaSeason,
    AnimeGenres,
    MediaFormat,
    AppStates,
    ClientWSMessageType,
    ServerWSMessageType,
    Page,
    GOGOServers
} from './misc/enums'

export type {
    AnimeCardInfo,
    AnimeInfo,
    GoGoAnimeInfo,
    AnimeFuzzyDate,
} from './misc/types'


export {
    Websocket
}

export const clipTitle = (title: string, length: number) => {
    if (title.length > length) {
        let splitTitle = title.split(" ");
        title = "";
        while (title.length < length) {
            title += splitTitle.shift() + " ";
        }
        title += "...";
    }
    return title;
}

export const findScroller = (e: HTMLElement) => {
    e.onscroll = function() { console.log(e); }
    Array.from(e.children).forEach((el) => {
        const element = el as HTMLElement;
        findScroller(element);
    });
}

export const genreColors = (genre: AnimeGenres) => {
    switch (genre) {
        case AnimeGenres.Action: return "#e6194B"
        case AnimeGenres.Adventure: return "#3cb44b"
        case AnimeGenres.Comedy: return "#ffe119"
        case AnimeGenres.Drama: return "#4363d8"
        case AnimeGenres.Ecchi: return "#f58231"
        case AnimeGenres.Fantasy: return "#911eb4"
        case AnimeGenres.Horror: return "#46f0f0"
        case AnimeGenres.Mecha: return "#f032e6"
        case AnimeGenres.Music: return "#bcf60c"
        case AnimeGenres.Mystery: return "#fabebe"
        case AnimeGenres.Psychological: return "#008080"
        case AnimeGenres.Romance: return "#e6beff"
        case AnimeGenres.SciFi: return "#9a6324"
        case AnimeGenres.SliceOfLife: return "#fffac8"
        case AnimeGenres.Sports: return "#800000"
        case AnimeGenres.Supernatural: return "#aaffc3"
        case AnimeGenres.Thriller: return "#808000"
        default: return "#000000"
    }
}

export const formatFormat = (format: string) => {
    let formatted = format.replace(/_/g, " ");
}

export const CachedValues = new CachedStore();
export {
    History
}