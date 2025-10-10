<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { onMount } from "svelte";

  export let affirmative: string = "OK";
  export let negative: string | undefined = undefined;
  export let cancel: string | undefined = undefined;
  export let title: string;
  export let severity: "" | "ok" | "caution" | "danger" = "";

  export let onAffirmative: () => void = () => {};
  export let onNegative: () => void = () => {};
  export let onCancel: () => void = () => {};

  let dialogRef: HTMLDialogElement | null = null;

  onMount(() => {
    if (dialogRef) {
      dialogRef.showModal();
    }
  });
</script>

<dialog bind:this={dialogRef}>
  <div class="overlay">
    <div class="card {severity}">
      <h1 class="caption">{title}</h1>
      <div class="content">
        <slot />
      </div>
      <div class="buttons-container">
        <button class={severity + " primary"} onclick={onAffirmative}>
          {affirmative}
        </button>

        {#if negative}
          <button class={severity + " secondary"} onclick={onNegative}>
            {negative}
          </button>
        {/if}

        {#if cancel}
          <button class={severity + " tertiary"} onclick={onCancel}>
            {cancel}
          </button>
        {/if}
      </div>
    </div>
  </div>
</dialog>

<style lang="scss">
  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .overlay {
    z-index: 1000;
    background-color: var(--panel-background);
    -webkit-backdrop-filter: blur(24px);
    backdrop-filter: blur(24px);
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fade-in var(--ui-hydrate-time) ease-out;
  }

  .card {
    z-index: 1001;
    position: fixed;
    top: 30%;
    margin: 0 auto;
  }
</style>
