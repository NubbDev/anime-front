import type { AnimeGenres, MediaFormat, MediaStatus } from "./enums"

export interface AnimeCardInfo {
    id: number,
    title: {
        english: string,
        romaji: string
    },
    type: string,
    coverImage?: {
        large?: string,
        medium?: string,
        color?: string
    },
    genres: (AnimeGenres)[],
    bannerImage?: string,
    description?: string
    format?: MediaFormat,
}

export interface AnimeInfo {
    id: number,
    title: {
        english: string,
        romaji: string,
        native: string
    },
    bannerImage: string | null,
    coverImage: {
        extraLarge?: string,
        large?: string,
        medium?: string,
        color: string
    },
    genres: (AnimeGenres)[],
    countryOfOrigin: string,
    description: string,
    trailer: {
        id: string,
        site: string,
        thumbnail: string
    } | null,
    status: keyof typeof MediaStatus,
    format: keyof typeof MediaFormat,
    startDate: AnimeFuzzyDate,
    endDate: AnimeFuzzyDate,
    gogoanime?: {
        dub: GoGoAnimeInfo| null
        sub: GoGoAnimeInfo | null
    }
}

export interface GoGoAnimeInfo {
    id: string,
    totalEpisodes: number,
    episodes: {
        id: string,
        number: number,
    }[]
}

export interface AnimeFuzzyDate {
    year: number | null,
    month: number | null,
    day: number | null
}

export interface MessageKind<T, D> {
    type: T,
    data: D
}

export interface GogoEpisodeLink {
    headers: {
        Referer: string,
        watchsb: string, // or null, since only provided with server being equal to "streamsb".
        "User-Agent": string // or null
    },
    sources: [
        {
            url: string,
            quality: string,
            isM3U8: true
        }
    ]
}