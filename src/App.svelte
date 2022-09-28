<script lang="ts">
  import CategoryContainer from "./lib/CategoryContainer.svelte";
  import { SortableList } from "@jhubbardsf/svelte-sortablejs";
  import NavBar from "./lib/NavBar.svelte";

  import { Category, roaList, CharactersTree } from "./data/store";
  import { each, onMount } from "svelte/internal";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { SortableEvent } from "sortablejs";

  let name = "world";
  let promise = getCharactersTree();

  $: console.log($roaList);

  function handleClick() {
    promise = getCharactersTree();
  }

  async function getCharactersTree() {
    const response: CharactersTree = await invoke("get_characters_tree");
    console.log(response);
    return response;
  }
</script>

<main id="screen" class="flex flex-col h-screen overflow-x-hidden">
  <NavBar on:parsed={handleClick} />
  <div
    class="h-full bg-gradient-to-b from-slate-100 via-slate-200 to-slate-300"
  >
    {#await promise}
      <div>loading...</div>
    {:then charactersTree}
      <SortableList
        class="list-group m-4 grid grid-cols-4 gap-4"
        multiDragClass="selected"
        animation={75}
        onSort={async (e) => {

          let new_i = [];
          let old_i = [];
          if (e.newIndicies.length > 0) {
            new_i = e.newIndicies.map((i) => i.index);
            old_i = e.oldIndicies.map((i) => i.index);
          } else {
            new_i = [e.newIndex];
            old_i = [e.oldIndex];
          }

          const response = await invoke("move_categories", {
            fromC: old_i,
            toC: new_i,
          });
        }}
      >
        {#each charactersTree.categories as cat, i}
          <CategoryContainer name={cat.name}>test</CategoryContainer>
        {/each}
      </SortableList>
    {/await}
  </div>
  <div class="absolute inset-x-0 bottom-0 z-40 bg-slate-500 h-10" />
</main>

<style>
</style>
