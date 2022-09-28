<script lang="ts">
  import NavBar from "./NavBar.svelte";

  import { Category, roaList } from "../data/store";
  import { each, onMount } from "svelte/internal";
  import Sortable, { MultiDrag } from "sortablejs";
  import { slide } from "svelte/transition";

  export let name;
  let expand = false;
</script>

<div
  class="list-group-item p-3 w-32 flex-col bg-sky-100 border border-purple-500 rounded-xl shadow-lg items-center"
>
  <div
    class="font-medium mx-full w-full text-center"
    on:click={() => (expand = !expand)}
  >
    {name}
  </div>

  {#if expand}
    <div
      class="border-solid border-rose-400 border-2 bg-slate-400 w-full"
      transition:slide
    >
      <slot />
    </div>
  {/if}
</div>

<style>
  :global(.selected){
    background-color: #f0f0f0;
  }
</style>
