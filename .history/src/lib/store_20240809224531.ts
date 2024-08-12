import { PageIndex } from "./misc/enums";
import { writable } from "svelte/store";

export const PageStore = writable<PageIndex>(PageIndex.HOME);