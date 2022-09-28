<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import RoaList, { roaList } from "../data/store";

    let aa: RoaList;

    const sub = roaList.subscribe((value) => {
        aa = value;
        console.log(aa);
    });

    async function getCharactersTree() {
        const response = await invoke("get_characters_tree");
        console.log(response);
    }
</script>

<!-- A rounded box with a shadow -->
<div class="box">
    <div class="">
        {#await getCharactersTree()}
            <div>loading...</div>
        {:then charactersTree}
            <div>{charactersTree}</div>
        {/await}

        <div
            class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4"
        >
            <p>test</p>
        </div>
    </div>
</div>

<style>
</style>
