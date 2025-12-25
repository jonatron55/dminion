<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License

-->
<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import AppIcon from "./AppIcon.svelte";
  import DialogBox from "./DialogBox.svelte";

  let appVersion: string = "";

  let isOpen: boolean = false;

  export function open() {
    isOpen = true;
  }

  function close() {
    isOpen = false;
  }

  onMount(async () => {
    appVersion = await getVersion();
  });
</script>

{#if isOpen}
  <DialogBox title="About Dungeon Minion" onAffirmative={close}>
    <div class="about">
      <div class="about-icon">
        <AppIcon />
      </div>
      <div class="version">
        Dungeon Minion {appVersion}
      </div>
      <hr />
      <p>Copyright © 2025 Jonathon B. Cobb</p>
      <p>
        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
        documentation files (the "Software"), to deal in the Software without restriction, including without limitation
        the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and
        to permit persons to whom the Software is furnished to do so, subject to the following conditions:
      </p>

      <p>
        The above copyright notice and this permission notice shall be included in all copies or substantial portions of
        the Software.
      </p>

      <p>
        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
        THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
        CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
        DEALINGS IN THE SOFTWARE.
      </p>
      <hr />
      <p>
        This work includes material from the System Reference Document 5.2.1 (“SRD 5.2.1”) and the System Reference
        Document 5.1 (“SRD 5.1”) by Wizards of the Coast LLC, both available at <a
          target="_blank"
          href="https://www.dndbeyond.com/srd">https://www.dndbeyond.com/srd</a
        >. The SRD 5.2.1 and SRD 5.1 are licensed under the Creative Commons Attribution 4.0 International License,
        available at
        <a target="_blank" href="https://creativecommons.org/licenses/by/4.0/"
          >https://creativecommons.org/licenses/by/4.0/</a
        >.
      </p>
    </div>
  </DialogBox>
{/if}

<style lang="scss">
  .about-icon {
    text-align: center;
    width: 12rem;
    height: 12rem;
    margin: auto;
  }

  .about {
    overflow-y: scroll;
    max-height: 40vh;
  }
  .version {
    text-align: center;
  }
</style>
