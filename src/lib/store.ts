import { AppStates, PageIndex } from "./misc/enums";
import { writable } from "svelte/store";
import { type GogoEpisodeLink, type AnimeCardInfo, type AnimeInfo } from "./misc/types";

export const AppStateStore = writable<AppStates>(AppStates.CONNECTING)
export const AppSearchStore = writable<boolean>(false)

export const BodyScroll = writable(0)

export const SearchPageStore = writable<{value: string, list: AnimeCardInfo[]} | null>(null)
export const TimeElapsedStore = writable<number>(0)
export const AnimeSelectedStore = writable<AnimeInfo | null>(null)
export const AnimePlayerStore = writable<GogoEpisodeLink | null>(null)