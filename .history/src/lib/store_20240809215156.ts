import { PageIndex } from "$lib";
import { writable } from "svelte/store";

export const PageStore = writable<PageIndex>(PageIndex.HOME);