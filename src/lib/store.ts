import { PageIndex } from "./misc/enums";
import { writable } from "svelte/store";
import type { AnimeCardInfo } from "./misc/types";

export const PageStore = writable<PageIndex>(PageIndex.HOME);
export const PageHistory = writable<PageIndex[]>([PageIndex.HOME]);

export const BodyScroll = writable(0)

export const FrontPageStore = writable<{
    trending: AnimeCardInfo[],
    popular: AnimeCardInfo[],
    season: AnimeCardInfo[],
    top: AnimeCardInfo[],
    lastUpdated: Date
} | null>(null)
export const TrendingPageStore = writable<AnimeCardInfo[] | null>(null)
export const PopularPageStore = writable<AnimeCardInfo[] | null>(null)
export const SeasonPageStore = writable<AnimeCardInfo[] | null>(null)
export const TopPageStore = writable<AnimeCardInfo[] | null>(null)
