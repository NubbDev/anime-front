// place files you want to import through the `$lib` alias in this folder.

export {
    PageStore,
    FrontPageStore,
    PageHistory,
    TrendingPageStore,
    BodyScroll
} from './store'

export {
    PageIndex,
    DisplayStateCSS,
    MediaTrendSort,
    MediaSeason,
    AnimeGenres
} from './misc/enums'

export type {
    AnimeCardInfo
} from './misc/types'

export const findScroller = (e: HTMLElement) => {
    e.onscroll = function() { console.log(e); }
    Array.from(e.children).forEach((el) => {
        const element = el as HTMLElement;
        findScroller(element);
    });
}