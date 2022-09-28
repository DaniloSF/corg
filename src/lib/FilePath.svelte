<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import {
        basename,
        dirname,
        join,
        localDataDir,
    } from "@tauri-apps/api/path";
    import { invoke } from "@tauri-apps/api/tauri";
    import { roaList } from "../data/store";
    import type RoaList from "../data/store";
    import OpenButton from "./OpenButton.svelte";
    import { createEventDispatcher, onMount } from "svelte";

    const dispatch = createEventDispatcher();

    let path = "";
    let name = "";

    //Get path to order.roa directory, defaults to %appdata%/RivalsofAether/workshop
    async function get_path() {
        const appDirPath = await localDataDir();
        const default_path = await join(
            appDirPath,
            "RivalsofAether",
            "workshop"
        );
        console.log(default_path);
        const path = (await open({
            title: "Select the order.roa directory",
            defaultPath: default_path,
            filters: [
                {
                    name: "All Files",
                    extensions: ["roa"],
                },
            ],
        })) as string;
        const response = await dirname(path);
        if (response == null) {
            name = default_path;
        } else {
            name = response;
        }
    }

    async function open_roa() {
        if (name == "") {
            return;
        }
        let o_path = await join(name, "order.roa");
        let cat_path = await join(name, "categories.roa");
        await invoke("read_roa_file", {
            orderPath: o_path,
            categoriesPath: cat_path,
        }).then(
            (value) => {
                alert(value);
            },
            (reason) => {}
        );
        dispatch("parsed");
        // await get_roa();
    }

    async function get_roa() {
        let rl: string = await invoke("get_roa_list");

        let obj = JSON.parse(rl);
        roaList.set(obj);
    }

    onMount(async () => {
        // await open_roa();
    });
</script>

<div class="flex items-center h-8 px-3 text-sm font-medium text-white gap-1">
    <div
        class="relative h-full flex border border-white rounded-md cursor-pointer px-2"
    >
        <button on:click={get_path}>Order.roa PATH</button>
    </div>
    <p>{path}</p>
    <div
        class="relative h-full flex border border-white rounded-md cursor-pointer px-2"
    >
        <button on:click={open_roa}>Open ROA</button>
    </div>
</div>
