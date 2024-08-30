import { invoke } from "@tauri-apps/api/core";
import { Page } from "../misc/enums";


export default class CachedStore {
    constructor() {}

    async getPage<T>(key: Page): Promise<{data: T, lastUpdated: Date} | null> {
        const data = await invoke("get_page_cache", { key }) as {data: T, lastUpdated: string} | null;
        if (!data) return null;
        return { data: data.data, lastUpdated: new Date(data.lastUpdated) };
    }
    async setPage<T>(key: Page, data: T) {
        await invoke("store_page_cache", { key, data });
    }

    async get<T>(key: string): Promise<{data: T, lastUpdated: Date} | null> {
        const data = await invoke("get_cache", { key }) as {data: T, lastUpdated: string} | null;
        if (!data) return null;
        return { data: data.data, lastUpdated: new Date(data.lastUpdated) };
    }
    async set<T>(key: string, data: T) {
        await invoke("store_cache", { key, data });
    }
}