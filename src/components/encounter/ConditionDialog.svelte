<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { conditionNames } from "$lib/model/Condition";
  import { ConditionDialogViewModel } from "$lib/viewmodel/ConditionDialogViewModel";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import type { ParticipantViewModel } from "$lib/viewmodel/ParticipantViewModel";
  import DialogBox from "../DialogBox.svelte";

  export let participant: ParticipantViewModel;
  export let game: GameViewModel;

  let isOpen: boolean = false;
  let selectedConditions: string[] = [];
  let customConditionName: string = "";
  let instigatorId: number | null = null;
  let isConcentrating: boolean = false;
  let durationType: "untilRemoved" | "elapsed" | "startTurn" | "endTurn" = "untilRemoved";
  let elapsedDuration: number = 1;
  let elapsedUnit: "seconds" | "minutes" | "hours" = "minutes";
  let turnParticipantId: number = participant.id;
  let turnParticipantName: string = participant.name;
  let viewModel: ConditionDialogViewModel;

  export function open() {
    // Reset form state
    selectedConditions = [];
    customConditionName = "";
    isConcentrating = false;
    durationType = "untilRemoved";
    elapsedDuration = 1;
    elapsedUnit = "minutes";
    instigatorId = viewModel.getDefaultInstigatorId();

    isOpen = true;
  }

  async function apply() {
    const conditions = viewModel.buildConditions({
      selectedConditions,
      customConditionName,
      durationType,
      elapsedDuration,
      elapsedUnit,
      instigatorId,
    });

    await participant.addConditions(conditions);
    isOpen = false;
  }

  function cancel() {
    isOpen = false;
  }

  $: viewModel = new ConditionDialogViewModel(game, participant);
  $: participants = viewModel.participants;
  $: hasInstigator = instigatorId !== null;
  $: turnParticipantId = viewModel.getTurnParticipantId(instigatorId);
  $: turnParticipantName = viewModel.getParticipantName(turnParticipantId);
  $: customCondition = selectedConditions.includes("other");
</script>

{#if isOpen}
  <DialogBox
    title="Add Conditions to {participant.name}"
    affirmative="Apply"
    cancel="Cancel"
    severity=""
    onAffirmative={apply}
    onCancel={cancel}
  >
    <div class="condition-dialog">
      <!-- Left Column: Conditions -->
      <div class="left-column">
        <h2>Conditions</h2>
        <div class="condition-options">
          {#each conditionNames as condition}
            <span class={condition}>
              <input type="checkbox" id="condition-{condition}" value={condition} bind:group={selectedConditions} />
              <label for="condition-{condition}" class="condition-label">{condition}</label>
            </span>
          {/each}
          <span>
            <input type="checkbox" id="condition-other" value="other" bind:group={selectedConditions} />
            <label for="condition-other" class="condition-label">Other:</label>
          </span>
          {#if customCondition}
            <input
              type="text"
              id="custom-condition-name"
              bind:value={customConditionName}
              placeholder="Enter condition name"
              class="custom-input"
            />
          {/if}
        </div>
      </div>

      <!-- Right Column: Options -->
      <div class="right-column">
        <!-- Instigator -->
        <div class="section">
          <h2><label for="instigator">Instigator</label></h2>
          <span class="checkbox-row">
            <select id="instigator" bind:value={instigatorId}>
              <option value={null}>None</option>
              {#each participants as p}
                <option value={p.id}>{p.name}</option>
              {/each}
            </select>

            <input type="checkbox" id="concentrating" bind:checked={isConcentrating} disabled={!hasInstigator} />
            <label for="concentrating">Concentrating</label>
          </span>
        </div>

        <!-- Duration -->
        <div class="section">
          <h2>Expires</h2>
          <div class="duration-options">
            <span>
              <input
                type="radio"
                name="duration"
                id="duration-until-removed"
                value="untilRemoved"
                bind:group={durationType}
              />
              <label for="duration-until-removed">When removed</label>
            </span>

            <span>
              <input
                type="radio"
                name="duration"
                id="duration-start-turn"
                value="startTurn"
                bind:group={durationType}
              />
              <label for="duration-start-turn">Start of {turnParticipantName}’s next turn</label>
            </span>
            <span>
              <input type="radio" name="duration" id="duration-end-turn" value="endTurn" bind:group={durationType} />
              <label for="duration-end-turn">End of {turnParticipantName}’s next turn</label>
            </span>

            <span>
              <input type="radio" name="duration" id="duration-elapsed" value="elapsed" bind:group={durationType} />
              <label for="duration-elapsed">After</label>
            </span>

            <div class="elapsed-inputs">
              <input
                type="number"
                id="elapsed-duration"
                bind:value={elapsedDuration}
                min="1"
                placeholder="Duration"
                disabled={durationType !== "elapsed"}
              />
              <select id="elapsed-unit" bind:value={elapsedUnit} disabled={durationType !== "elapsed"}>
                <option value="seconds">seconds</option>
                <option value="minutes">minutes</option>
                <option value="hours">hours</option>
              </select>
            </div>
          </div>
        </div>
      </div>
    </div>
  </DialogBox>
{/if}

<style lang="scss">
  .condition-dialog {
    display: grid;
    grid-template-columns: auto auto;
    gap: calc(var(--horizontal-gap) * 3);
    margin-bottom: calc(var(--vertical-gap) * 2);

    .left-column,
    .right-column {
      display: flex;
      flex-direction: column;
      gap: calc(var(--vertical-gap) * 2);
    }

    .condition-options {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: var(--vertical-gap);

      > span {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: var(--horizontal-gap);
      }

      .custom-input {
        margin-left: calc(var(--horizontal-gap) * 2);
        margin-top: var(--vertical-gap);
        grid-column: span 2;
      }
    }

    .condition-label {
      text-transform: capitalize;
    }

    .section {
      display: flex;
      flex-direction: column;
      gap: var(--vertical-gap);

      > label:first-child {
        font-weight: bold;
      }
    }

    .checkbox-row {
      display: flex;
      flex-direction: row;
      align-items: center;
      gap: var(--horizontal-gap);

      input[type="checkbox"] {
        margin: 0;
      }
    }

    .duration-options {
      display: flex;
      flex-direction: column;
      gap: var(--vertical-gap);

      > span {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: var(--horizontal-gap);
      }

      .elapsed-inputs {
        display: flex;
        flex-direction: row;
        gap: var(--horizontal-gap);
        padding-left: 3.5rem;
        align-items: center;

        input[type="number"] {
          flex: 1;
          min-width: 5rem;
          max-width: 6rem;
        }

        select {
          flex: 1;
          margin: auto 0;
        }
      }
    }
  }
</style>
