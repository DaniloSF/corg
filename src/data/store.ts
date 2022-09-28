import { writable } from "svelte/store";

class Item {
    icon: string;
    name: string;
    item_type: number;
    url: number;
    author: string;
    description: string;
    version: [number, number];
    finished: boolean;
    bg_color: string;
    plural: boolean;
    root: string;
}

class GenericTree {
    item_type: number = 1;
    items: Array<Item> = [];
}
export class CharactersTree {
    item_type: number;
    categories: Category[] = [];
}

export class Category {
    name: string;
    items: Item[] = [];
}

class RoaList {
    char_tree: CharactersTree = { item_type: 0, categories: [] };
    buddy_tree: GenericTree = { item_type: 1, items: [] };
    skin_tree: GenericTree = { item_type: 0, items: [] };
    stage_tree: GenericTree = { item_type: 2, items: [] };
}

export const roaList = writable(new RoaList);
export default RoaList