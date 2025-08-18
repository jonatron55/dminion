<script lang="ts">
  import { MonsterViewModel } from "$lib/viewmodel/MonsterViewModel";
  import DialogBox from "../DialogBox.svelte";

  export let monster: MonsterViewModel;

  let isOpen: boolean = false;
  let amount: number = 0;
  let type: "heal" | "setHp" | "setTempHp" = "heal";

  export function open() {
    isOpen = true;
  }

  async function apply() {
    await monster.heal({ type, amount });
    isOpen = false;
  }

  function cancel() {
    isOpen = false;
  }
</script>

{#if isOpen}
  <DialogBox
    title="Heal {monster.name}"
    affirmative="Apply"
    cancel="Cancel"
    severity="ok"
    onAffirmative={apply}
    onCancel={cancel}
  >
    <div class="heal-dialog">
      <div class="amount">
        <label id="amount-label" for="amount">Amount</label>
        <input id="amount" type="number" bind:value={amount} min="0" placeholder="amount" />
      </div>
      <div class="mode">
        <span>
          <input type="radio" name="mode" id="mode-heal" value="heal" bind:group={type} />
          <label for="mode-heal">Heal</label>
        </span>
        <span>
          <input type="radio" name="mode" id="mode-set-hp" value="setHp" bind:group={type} />
          <label for="mode-set-hp">Set HP</label>
        </span>
        <span>
          <input type="radio" name="mode" id="mode-temp-hp" value="setTempHp" bind:group={type} />
          <label for="mode-temp-hp">Set Temp. HP</label>
        </span>
      </div>
    </div>
  </DialogBox>
{/if}

<style lang="scss">
  .heal-dialog {
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
