<script lang="ts">
  import { onDestroy } from "svelte";
  import { type MessageBox, messageBoxStore } from "../lib/MessageBox";

  let message: MessageBox | undefined = undefined;
  const unsubscribe = messageBoxStore.subscribe((m) => (message = m));

  onDestroy(unsubscribe);
</script>

{#if message}
  <div class="overlay">
    <div class="card {message.severity}">
      <h1 class="caption">{message.title}</h1>
      <p>
        {message.content}
      </p>
      <div class="buttons-container">
        {#if message.affirmativeButton}
          <button
            class={message.severity + " primary"}
            onclick={() => {
              messageBoxStore.clear();
              message!.affirmativeButton.action?.();
            }}
          >
            {message.affirmativeButton.label}
          </button>
        {/if}

        {#if message.negativeButton}
          <button
            class={message.severity + " secondary"}
            onclick={() => {
              messageBoxStore.clear();
              message!.negativeButton!.action?.();
            }}
          >
            {message.negativeButton.label}
          </button>
        {/if}

        {#if message.cancelButton}
          <button
            class={message.severity + " tertiary"}
            onclick={() => {
              messageBoxStore.clear();
              message!.cancelButton!.action?.();
            }}
          >
            {message.cancelButton.label}
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  .overlay {
    z-index: 1000;
    background-color: var(--content-background-transparent);
    -webkit-backdrop-filter: blur(24px);
    backdrop-filter: blur(24px);
    transition: opacity 500ms ease-out;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .card {
    z-index: 1001;
    position: fixed;
    top: 30%;
    margin: 0 auto;

    p {
      white-space: pre-wrap;
    }
  }
</style>
