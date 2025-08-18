<script lang="ts">
  import { MonsterViewModel } from "$lib/viewmodel/MonsterViewModel";
  import DialogBox from "../DialogBox.svelte";

  export let monster: MonsterViewModel;

  let isOpen: boolean = false;
  let amount: number = 0;
  let mode: "damage" | "half damage" | "double damage" | "kill" = "damage";

  export function open() {
    isOpen = true;
  }
</script>

{#if isOpen}
  <DialogBox
    title="Damage {monster.name}"
    affirmative="Apply"
    cancel="Cancel"
    severity="danger"
    onAffirmative={() => {
      // Apply damage logic here
      isOpen = false;
    }}
    onCancel={() => {
      isOpen = false;
    }}
  >
    <div class="damage-dialog">
      <div class="amount">
        <label id="amount-label" for="amount">Amount</label>
        <input id="amount" type="number" bind:value={amount} min="0" placeholder="amount" />
      </div>
      <div class="mode">
        <span>
          <input type="radio" name="mode" id="mode-damage" value="damage" bind:group={mode} />
          <label for="mode-damage">Damage</label>
        </span>
        <span>
          <input type="radio" name="mode" id="mode-set-hp" value="Half damage" bind:group={mode} />
          <label for="mode-set-hp">Half damage</label>
        </span>
        <span>
          <input type="radio" name="mode" id="mode-temp-hp" value="Double damage" bind:group={mode} />
          <label for="mode-temp-hp">Double damage</label>
        </span>
        <span>
          <input type="radio" name="mode" id="mode-kill" value="kill" bind:group={mode} />
          <label for="mode-kill">Kill instantly</label>
        </span>
      </div>
    </div>
  </DialogBox>
{/if}

<style lang="scss">
  .damage-dialog {
    display: flex;
    flex-direction: row;
    gap: var(--horizontal-gap);
    align-items: center;
    margin-bottom: calc(var(--vertical-gap) * 2);

    .amount {
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: var(--vertical-gap);
    }

    .mode {
      display: flex;
      flex-direction: column;
      gap: var(--vertical-gap);
    }
  }
</style>
