import { PageIndex } from "./misc/enums";
import { writable } from "svelte/store";
import type { AnimeCardInfo } from "./misc/types";

export const PageStore = writable<PageIndex>(PageIndex.HOME);

export const FrontPageStore = writable<{
    trending: AnimeCardInfo[],
    popular: AnimeCardInfo[],
    season: AnimeCardInfo[],
    top: AnimeCardInfo[],
    lastUpdated: Date
} | null>(null)