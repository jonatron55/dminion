<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { conditionNames } from "$lib/model/Condition";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import type { ParticipantViewModel } from "$lib/viewmodel/ParticipantViewModel";
  import DialogBox from "../DialogBox.svelte";

  export let participant: ParticipantViewModel;
  export let game: GameViewModel;

  let isOpen: boolean = false;
  let conditionName: string = "";
  let customConditionName: string = "";
  let instigatorId: number | null = null;
  let isConcentrating: boolean = false;
  let durationType: "untilRemoved" | "elapsed" | "startTurn" | "endTurn" = "untilRemoved";
  let elapsedDuration: number = 1;
  let elapsedUnit: "seconds" | "minutes" | "hours" = "minutes";
  let turnParticipantId: number | null = null;

  export function open() {
    // Reset form state
    conditionName = "";
    customConditionName = "";
    isConcentrating = false;
    durationType = "untilRemoved";
    elapsedDuration = 1;
    elapsedUnit = "minutes";
    turnParticipantId = null;

    // Set default instigator
    if (participant.id === game.activeParticipantId) {
      instigatorId = null;
    } else {
      instigatorId = game.activeParticipantId ?? null;
    }

    isOpen = true;
  }

  async function apply() {
    // Use custom condition name if "other" is selected
    const finalConditionName = conditionName === "other" ? customConditionName : conditionName;

    // TODO: Call participant.addCondition or similar with the collected data
    console.log("Apply condition:", {
      conditionName: finalConditionName,
      instigatorId,
      isConcentrating,
      durationType,
      elapsedDuration,
      elapsedUnit,
      turnParticipantId,
    });
    isOpen = false;
  }

  function cancel() {
    isOpen = false;
  }

  $: participants = Object.values(game.participants);
  $: hasInstigator = instigatorId !== null;
  $: showElapsedInputs = durationType === "elapsed";
  $: showTurnParticipant = durationType === "startTurn" || durationType === "endTurn";
  $: isCustomCondition = conditionName === "other";
</script>

{#if isOpen}
  <DialogBox
    title="Add Condition to {participant.name}"
    affirmative="Apply"
    cancel="Cancel"
    severity=""
    onAffirmative={apply}
    onCancel={cancel}
  >
    <div class="condition-dialog">
      <!-- Left Column: Conditions -->
      <div class="left-column">
        <h2>Condition</h2>
        <div class="condition-options">
          {#each conditionNames as condition}
            <span class={condition}>
              <input
                type="radio"
                name="condition"
                id="condition-{condition}"
                value={condition}
                bind:group={conditionName}
              />
              <label for="condition-{condition}" class="condition-label">{condition}</label>
            </span>
          {/each}
          <span>
            <input type="radio" name="condition" id="condition-other" value="other" bind:group={conditionName} />
            <label for="condition-other" class="condition-label">Other:</label>
          </span>
          {#if isCustomCondition}
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
          <select id="instigator" bind:value={instigatorId}>
            <option value={null}>None</option>
            {#each participants as p}
              <option value={p.id}>{p.name}</option>
            {/each}
          </select>

          {#if hasInstigator}
            <span class="checkbox-row">
              <input type="checkbox" id="concentrating" bind:checked={isConcentrating} />
              <label for="concentrating">Instigator is concentrating</label>
            </span>
          {/if}
        </div>

        <!-- Duration -->
        <div class="section">
          <h2>Duration</h2>
          <div class="duration-options">
            <span>
              <input
                type="radio"
                name="duration"
                id="duration-until-removed"
                value="untilRemoved"
                bind:group={durationType}
              />
              <label for="duration-until-removed">Until removed</label>
            </span>

            <span>
              <input type="radio" name="duration" id="duration-elapsed" value="elapsed" bind:group={durationType} />
              <label for="duration-elapsed">Elapsed time</label>
            </span>

            {#if showElapsedInputs}
              <div class="elapsed-inputs">
                <input
                  type="number"
                  id="elapsed-duration"
                  bind:value={elapsedDuration}
                  min="1"
                  placeholder="Duration"
                />
                <select id="elapsed-unit" bind:value={elapsedUnit}>
                  <option value="seconds">seconds</option>
                  <option value="minutes">minutes</option>
                  <option value="hours">hours</option>
                </select>
              </div>
            {/if}

            <span>
              <input
                type="radio"
                name="duration"
                id="duration-start-turn"
                value="startTurn"
                bind:group={durationType}
              />
              <label for="duration-start-turn">Start of turn</label>
            </span>

            <span>
              <input type="radio" name="duration" id="duration-end-turn" value="endTurn" bind:group={durationType} />
              <label for="duration-end-turn">End of turn</label>
            </span>

            {#if showTurnParticipant}
              <div class="turn-participant">
                <select id="turn-participant" bind:value={turnParticipantId}>
                  <option value={null}>Select participant</option>
                  {#if hasInstigator}
                    <option value={instigatorId}>
                      {participants.find((p) => p.id === instigatorId)?.name} (instigator)
                    </option>
                  {/if}
                  {#each participants as p}
                    {#if p.id !== instigatorId}
                      <option value={p.id}>{p.name}</option>
                    {/if}
                  {/each}
                </select>
              </div>
            {/if}
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
        padding-left: calc(var(--horizontal-gap) * 2);

        input[type="number"] {
          flex: 1;
          min-width: 5rem;
        }

        select {
          flex: 1;
        }
      }

      .turn-participant {
        padding-left: calc(var(--horizontal-gap) * 2);

        select {
          width: 100%;
        }
      }
    }
  }
</style>
