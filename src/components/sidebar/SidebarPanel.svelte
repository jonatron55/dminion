<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import type { AppSidebarMode } from "$lib/AppMode";
  import DicePanel from "./DicePanel.svelte";
  import NamegenPanel from "./NamegenPanel.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";

  export let mode: Exclude<AppSidebarMode, undefined>;
</script>

<aside class="panel">
  <div class="content">
    {#if mode === "dice"}
      <DicePanel />
    {:else if mode === "namegen"}
      <NamegenPanel />
    {:else if mode === "settings"}
      <SettingsPanel />
    {/if}
  </div>
</aside>

<style lang="scss">
  aside {
    height: 100%;
    max-height: 100%;
    display: grid;
    grid-template-rows: 1fr;
    overflow: hidden;
  }

  .content {
    overflow: auto;
  }
</style>
