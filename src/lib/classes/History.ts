import { PageIndex } from "$lib/misc/enums";
import { Channel, invoke } from "@tauri-apps/api/core";

export default class History {
    static readonly listeners:  Array<(data: PageIndex) => void> = [];
    static lastPage: PageIndex | null = null;
    constructor() {

    }

    static async init() {
        const onMessage = new Channel<PageIndex>()
        onMessage.onmessage = (msgData) => {
            for (const l of this.listeners) {
                l(msgData)
            }
        }

        return await invoke('init_history', {onMessage})
    }

    static async push(page: PageIndex) {
        this.lastPage = await invoke('push_history_state', {page})
    }

    static async pop() {
        this.lastPage = await invoke('pop_history_state');
    }

    static async get(): Promise<PageIndex> {
        return await invoke('get_history_state')
    }

    static async clear() {
        await invoke('clear_history')
    }

    static async getList(): Promise<PageIndex[]> {
        return await invoke('get_history_list')
    }

    static async previousPage(): Promise<PageIndex> {
        return await invoke('get_history_previous_page')
    }

    static onUpdate(listener: (data: PageIndex) => void) {
        this.listeners.push(listener)
    }



}