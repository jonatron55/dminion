<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { onDestroy, onMount, tick } from "svelte";

  export let buttonClass: string = "button";
  export let panelClass: string = "card";
  export let disabled: boolean = false;
  export let align: "left" | "right" = "left";

  let isOpen = false;
  let triggerRef: HTMLElement | null = null;
  let panelRef: HTMLElement | null = null;

  async function positionPanel() {
    await tick();
    if (!triggerRef || !panelRef) {
      return;
    }

    const rect = triggerRef.getBoundingClientRect();
    const panelRect = panelRef.getBoundingClientRect();

    const vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    const vh = Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0);

    let top = rect.bottom;
    let left = align === "right" ? rect.right - panelRect.width : rect.left;

    // Clamp horizontally
    if (left + panelRect.width > vw) {
      left = Math.max(0, vw - panelRect.width);
    }
    left = Math.max(0, left);

    // If it would overflow vertically, open upward
    if (top + panelRect.height > vh) {
      top = Math.max(0, rect.top - panelRect.height);
    }

    panelRef.style.left = `${Math.round(left)}px`;
    panelRef.style.top = `${Math.round(top)}px`;
  }

  function onDocumentClick(e: MouseEvent) {
    // if (isOpen) {
    //   const target = e.target as Node | null;
    //   if (!target || !panelRef?.contains(target) || !triggerRef?.contains(target)) {
    //     close();
    //   }
    // }
  }

  function onPanelClick(e: MouseEvent) {
    // e.stopPropagation();
    // close();
  }

  function toggle() {
    if (isOpen) {
      close();
    } else {
      open();
    }
  }

  function open() {
    if (!disabled) {
      isOpen = true;
      positionPanel();
    }
  }

  function close() {
    isOpen = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && isOpen) {
      close();
    }
  }

  function onResize() {
    if (isOpen) {
      positionPanel();
    }
  }

  function onScroll() {
    if (isOpen) {
      positionPanel();
    }
  }

  onMount(() => {
    document.addEventListener("click", onDocumentClick);
    document.addEventListener("keydown", onKeydown);
    window.addEventListener("resize", onResize);
    window.addEventListener("scroll", onScroll, true);
  });

  onDestroy(() => {
    document.removeEventListener("click", onDocumentClick);
    document.removeEventListener("keydown", onKeydown);
    window.removeEventListener("resize", onResize);
    window.removeEventListener("scroll", onScroll, true);
  });
</script>

<button
  bind:this={triggerRef}
  class={buttonClass}
  aria-expanded={isOpen}
  on:click={toggle}
  on:input={positionPanel}
  on:keydown={onKeydown}
  {disabled}
>
  <slot name="trigger"></slot>
</button>

{#if isOpen}
  <div
    bind:this={panelRef}
    class="dropdown {panelClass}"
    role="menu"
    tabindex="-1"
    on:click={onPanelClick}
    on:keydown={onKeydown}
  >
    <div class="content" role="none">
      <slot name="content"></slot>
    </div>
  </div>
{/if}

<style lang="scss">
  .dropdown {
    position: fixed;
    z-index: 300;
    min-width: 8rem;
    max-width: 36rem;
  }
</style>
