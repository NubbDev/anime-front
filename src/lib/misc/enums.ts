export enum PageIndex {
    HOME,
    ABOUT,
    RELEASES,
    DOWNLOADS,
    PROFILE,
    SETTINGS,
    ANIME,
    PLAYER,
    SEARCH,
    AUTHENCATION,
    NOT_FOUND,
    TRENDING,
    POPULAR,
    TOP,
    SEASON
}

export enum MediaTrendSort {
    ID = "ID",
    ID_DESC = "ID_DESC",
    MEDIA_ID = "MEDIA_ID",
    MEDIA_ID_DESC = "MEDIA_ID_DESC",
    DATE = "DATE",
    DATE_DESC = "DATE_DESC",
    SCORE = "SCORE",
    SCORE_DESC = "SCORE_DESC",
    POPULARITY = "POPULARITY",
    POPULARITY_DESC = "POPULARITY_DESC",
    TRENDING = "TRENDING",
    TRENDING_DESC = "TRENDING_DESC",
    EPISODE = "EPISODE",
    EPISODE_DESC = "EPISODE_DESC",
}

export enum MediaSeason {
    WINTER = "WINTER",
    SPRING = "SPRING",
    SUMMER = "SUMMER",
    FALL = "FALL"
}
export enum AnimeGenres {
    Action = "Action",
    Adventure = "Adventure",
    Comedy = "Comedy",
    Drama = "Drama",
    Ecchi = "Ecchi",
    Fantasy = "Fantasy",
    Horror = "Horror",
    Mecha = "Mecha",
    Music = "Music",
    Mystery = "Mystery",
    Psychological = "Psychological",
    Romance = "Romance",
    SciFi = "Sci-Fi",
    SliceOfLife = "Slice of Life",
    Sports = "Sports",
    Supernatural = "Supernatural",
    Thriller = "Thriller"
}

export enum MediaFormat {
    TV = "TV",
    TV_SHORT = "TV_SHORT",
    MOVIE = "MOVIE",
    SPECIAL = "SPECIAL",
    OVA = "OVA",
    ONA = "ONA",
    MUSIC = "MUSIC",
    MANGA = "MANGA",
    NOVEL = "NOVEL",
    ONE_SHOT = "ONE_SHOT"
}

export enum MediaStatus {
    FINISHED = "FINISHED",
    RELEASING = "RELEASING",
    NOT_YET_RELEASED = "NOT_YET_RELEASED",
    CANCELLED = "CANCELLED",
    HIATUS = "HIATUS"
}

export enum AppStates {
    CONNECTING,
    READY,
    CONNECTED,
    
}

export enum ClientWSMessageType {
    Search,
    CommonPage,
    HomePage,
    SearchPage,
    SeasonAnimes,
    GetAnime,
    GetEpisode
}

export enum ServerWSMessageType {
    SearchResult,
    CommonPageData,
    HomePageRoute,
    SearchPageContent,
    SeasonAnimesList,
    AnimeData,
    EpisodeLink,
    GogoAnimeData,
}

export enum Page {
    Home,
    About,
    Releases,
    Downloads,
    Profile,
    Settings,
    Player,
    Search,
    Auth,
    NotFound,
    Trending,
    Popular,
    Top,
    Seasonal
}

export enum GOGOServers {
    Gogo = "gogocdn",
    SreamSB = "streamsb",
    Vidstreaming = "vidstreaming",
}