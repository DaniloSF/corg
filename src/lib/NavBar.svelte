<script lang="ts">
  import { roaList } from "../data/store";
  import Greet from "./Greet.svelte";
  import FilePath from "./FilePath.svelte";
  import RoaComponent from "./RoaComponent.svelte";

  let name = "world";
  $: greeting = `Hello ${name}!`;
  $: if (roaList) {
    console.log(roaList);
  }
</script>

<div
  class="relative min-h-16 bottom-0 left-0 z-30 w-full h-16 bg-gradient-to-r from-sky-500 via-sky-600 to-purple-500"
>
  <div
    class="flex items-center justify-between w-full h-full px-3 mx-auto max-w-7x1"
  >
    <div class="relative flex items-center shadow-2xl">
      <div class="font-medium text-blue-100">CORG: Characters Organizer</div>
    </div>
    <div class="w-auto">
      <FilePath on:parsed/>
    </div>
  </div>
</div>

<style>
</style>
