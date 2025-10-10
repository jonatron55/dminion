<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { onDestroy } from "svelte";
  import { type MessageBox, messageBoxStore } from "../lib/MessageBox";
  import DialogBox from "./DialogBox.svelte";

  let message: MessageBox | undefined = undefined;
  const unsubscribe = messageBoxStore.subscribe((m) => (message = m));

  onDestroy(unsubscribe);
</script>

{#if message}
  <DialogBox
    severity={message.severity}
    title={message.title}
    affirmative={message.affirmativeButton?.label}
    negative={message.negativeButton?.label}
    cancel={message.cancelButton?.label}
    onAffirmative={() => {
      messageBoxStore.clear();
      message!.affirmativeButton.action?.();
    }}
    onNegative={() => {
      messageBoxStore.clear();
      message!.negativeButton!.action?.();
    }}
    onCancel={() => {
      messageBoxStore.clear();
      message!.cancelButton!.action?.();
    }}
  >
    <p>{message.content}</p>
  </DialogBox>
{/if}

<style lang="scss">
  p {
    white-space: pre-wrap;
  }
</style>
