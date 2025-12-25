<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { currentFontSize, currentFontStyle, fontSizes, fontStyles } from "$lib/theme";
  import AboutDialog from "../AboutDialog.svelte";
  import ThemeSelector from "./ThemeSelector.svelte";

  let aboutDialogRef: AboutDialog;
</script>

<div>
  <ThemeSelector />
  <div class="options">
    <h2>Font size</h2>
    {#each fontSizes as size}
      <input
        type="radio"
        id={"font-size-" + size.toLowerCase()}
        name="font-size"
        value={size}
        bind:group={$currentFontSize}
      />
      <label for={"font-size-" + size.toLowerCase()}>{size}</label>
    {/each}
  </div>
  <div class="options">
    <h2>Font style</h2>
    {#each fontStyles as style}
      <input
        type="radio"
        id={"font-style-" + style.toLowerCase()}
        name="font-style"
        value={style}
        bind:group={$currentFontStyle}
      />
      <label for={"font-style-" + style.toLowerCase()}>{style}</label>
    {/each}
  </div>
  <div class="options">
    <h2>Rules</h2>
    <input type="radio" id="rules-510" name="rules" value="srd510" checked />
    <label for="rules-510">SRD 5.1</label>
    <input type="radio" id="rules-521" name="rules" value="srd521" />
    <label for="rules-521">SRD 5.2.1</label>
  </div>
  <div class="options">
    <h2>Monster Hit Points</h2>
    <input type="radio" id="hp-fixed" name="hp" value="fixed" checked />
    <label for="hp-fixed">Fixed</label>
    <input type="radio" id="hp-rolled" name="hp" value="rolled" />
    <label for="hp-rolled">Rolled</label>
  </div>
  <hr />
  <button on:click={() => aboutDialogRef.open()}>About…</button>
</div>

<AboutDialog bind:this={aboutDialogRef} />

<style lang="scss">
  div {
    display: grid;
    gap: 1rem var(--horizontal-gap);
    overflow-x: hidden scroll;
    padding: 0 var(--horizontal-gap);
  }

  .options {
    display: grid;
    gap: 0;
  }
</style>
