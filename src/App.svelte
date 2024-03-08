<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let msg = "";

  onMount(() => {
    listen<string>("menuitem", (event) => {
      msg = event.payload;
    });
  });

  async function onMenu1() {
    await invoke("menu1");
  }

  async function onMenu2() {
    await invoke("menu2");
  }
</script>

<main class="container">
  <div class="row">
    <p>
      <button on:click={onMenu1}>Context menu 1</button>
      <button on:click={onMenu2}>Context menu 2</button>
    </p>
  </div>

  <div class="row">
    <p>Selected menu item: {msg}</p>
  </div>
</main>
