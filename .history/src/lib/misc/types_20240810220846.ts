export interface Anime {
    id: number,
    title: {
        english: string,
        native: string
    },
    type: string,
    coverImage: {
        large: string,
        color: string
    },
    genres: string[],
    bannerImage?: string,
    description?: string
}