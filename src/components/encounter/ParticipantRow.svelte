<script lang="ts">
  import { conditionClasses } from "$lib/model/Condition";
  import type { GameViewModel } from "$lib/viewmodel/GameViewModel";
  import {
    LairViewModel,
    MonsterViewModel,
    ParticipantViewModel,
    PlayerViewModel,
  } from "$lib/viewmodel/ParticipantViewModel";
  import HealDialog from "./HealDialog.svelte";
  import Stat from "./Stat.svelte";

  export let game: GameViewModel;
  export let participant: ParticipantViewModel;
  export let isActive: boolean = false;
  export let isSelected: boolean = false;

  let monster: MonsterViewModel = participant as MonsterViewModel;
  let player: PlayerViewModel = participant as PlayerViewModel;
  let lair: LairViewModel = participant as LairViewModel;

  let healDialogRef: HealDialog;

  $: monster: participant as MonsterViewModel;
  $: player: participant as PlayerViewModel;
  $: lair: participant as LairViewModel;
</script>

<div class="participant{isActive ? ' active' : ''} ">
  {#if isActive}
    <div class="active-indicator {conditionClasses(participant.conditions)}">
      {participant.initiative}
    </div>
  {:else}
    <div></div>
  {/if}
  <img
    class="portrait {conditionClasses(participant.conditions)}"
    src={participant.smallPortrait}
    alt={`${participant.name} portrait`}
  />

  <div
    class="panel {isActive ? 'active ' : ''}{conditionClasses(
      participant.conditions,
    )}"
  >
    <div class="caption">
      <span class="title">
        <span class="initiative {conditionClasses(participant.conditions)}"
          >{participant.initiative}.</span
        >
        <span class="name {conditionClasses(participant.conditions)}"
          >{participant.name}</span
        >
      </span>
      {#if participant.conditions.length > 0}
        <span class="conditions">
          {#each participant.conditions as condition}
            <span class="{condition.name} badge">
              {condition.name}
              {#if condition.duration}
                <span class="count"
                  >{condition.startTime! +
                    condition.duration -
                    game.time}s</span
                >
              {/if}
            </span>
          {/each}
        </span>
      {/if}
      <span class="decorations">
        <button class="toolbar">+ Condition</button>
        <button class="toolbar">Edit…</button>
        <button class="toolbar danger">&#x2715;</button>
      </span>
    </div>
    {#if participant instanceof MonsterViewModel}
      <div class="content monster">
        <div>
          <input
            type="checkbox"
            id="{monster.name}-action"
            bind:checked={monster.action}
          />
          <label for="{monster.name}-action">Action</label>
        </div>
        <div>
          <input
            type="checkbox"
            id="{monster.name}-bonus"
            bind:checked={monster.bonusAction}
          />
          <label for="{monster.name}-bonus">Bonus</label>
        </div>
        <div>
          <input
            type="checkbox"
            id="{monster.name}-reaction"
            bind:checked={monster.reaction}
          />
          <label for="{monster.name}-reaction">Reaction</label>
        </div>

        {#each Array(monster.legendaryActions)
          .fill(0)
          .map((_, i) => i) as i}
          <div class="legendary">
            <input
              type="checkbox"
              id="{monster.name}-legendary-{i}"
              checked={monster.legendaryActions >= i + 1}
            />
            <label for="{monster.name}-legendary-{i}">Legendary</label>
          </div>
        {/each}

        <div class="ac"><strong>AC:</strong> {monster.ac}</div>
        <div class="hp"><strong>HP:</strong> {monster.hp}</div>
        {#if monster.tempHp > 0}
          <div class="temp-hp"><strong>temp:</strong> {monster.tempHp}</div>
        {/if}
        <button class="danger damage-button">Damage</button>
        <button class="ok heal-button" on:click={() => healDialogRef.open()}>
          Heal
        </button>

        <div class="str"><Stat label="str" value={monster.stats.str} /></div>
        <div class="dex"><Stat label="dex" value={monster.stats.dex} /></div>
        <div class="con"><Stat label="con" value={monster.stats.con} /></div>
        <div class="int"><Stat label="int" value={monster.stats.int} /></div>
        <div class="wis"><Stat label="wis" value={monster.stats.wis} /></div>
        <div class="cha"><Stat label="cha" value={monster.stats.cha} /></div>

        <div class="notes" contenteditable="true"></div>
      </div>
    {:else if participant instanceof PlayerViewModel}
      <div class="content player">
        <div>
          <input
            type="checkbox"
            id="{monster.name}-action"
            bind:checked={monster.action}
          />
          <label for="{monster.name}-action">Action</label>
        </div>
        <div>
          <input
            type="checkbox"
            id="{monster.name}-bonus"
            bind:checked={monster.bonusAction}
          />
          <label for="{monster.name}-bonus">Bonus</label>
        </div>
        <div>
          <input
            type="checkbox"
            id="{monster.name}-reaction"
            bind:checked={monster.reaction}
          />
          <label for="{monster.name}-reaction">Reaction</label>
        </div>

        <div class="str"><Stat label="str" value={player.stats.str} /></div>
        <div class="dex"><Stat label="dex" value={player.stats.dex} /></div>
        <div class="con"><Stat label="con" value={player.stats.con} /></div>
        <div class="int"><Stat label="int" value={player.stats.int} /></div>
        <div class="wis"><Stat label="wis" value={player.stats.wis} /></div>
        <div class="cha"><Stat label="cha" value={player.stats.cha} /></div>

        <div class="notes" contenteditable="true"></div>
      </div>
    {:else if participant instanceof LairViewModel}
      <div class="content lair">
        <div>
          <input
            type="checkbox"
            id="{lair.name}-action"
            bind:checked={lair.action}
          />
          <label for="{lair.name}-action">Action</label>
        </div>
        <div class="notes" contenteditable="true"></div>
      </div>
    {/if}
  </div>
</div>

<HealDialog bind:monster bind:this={healDialogRef} />

<style lang="scss">
  .participant {
    display: grid;
    grid-template-columns: 4rem auto 1fr;
    gap: var(--vertical-gap) var(--horizontal-gap);
    border-radius: var(--ui-border-radius);
  }

  .active-indicator {
    margin: 2.5rem auto auto auto;
    font-weight: bold;
    color: var(--page-background);
    background-color: var(--chrome);
    display: block;
    text-align: right;
    padding: var(--vertical-gap) var(--horizontal-gap);
    border-radius: 3rem;
    border: double 8px var(--page-background);
  }

  .portrait {
    width: 8rem;
    height: 8rem;
  }

  .caption {
    font-weight: normal;
    padding: 0 var(--horizontal-gap) 3px var(--horizontal-gap) !important;
    display: grid;
    grid-template-columns: auto auto 1fr auto;
    gap: var(--horizontal-gap);
    align-items: center;
  }

  .initiative {
    font-weight: normal;

    &.dead {
      text-decoration: line-through;
    }
  }

  .name {
    font-weight: bold;

    &.dead {
      text-decoration: line-through;
    }
  }

  .title {
    grid-column: 1;
  }

  .conditions {
    display: inline-flex;
    gap: var(--horizontal-gap);
    margin-left: var(--horizontal-gap);
    grid-column: 2;
  }

  .decorations {
    grid-column: 4;
  }

  input[type="checkbox"]:not(.button):checked + label {
    text-decoration: line-through;
    &::before {
      content: "\2718"; // Heavy Ballot X
    }
  }

  .toolbar {
    margin: 0;
  }

  .content {
    display: grid;
    grid-template-rows: repeat(3, 2rem);
    grid-auto-flow: column;
    column-gap: calc(3 * var(--horizontal-gap));

    * {
      margin: auto 0;
    }

    input,
    label {
      margin: 0;
      padding: 0;
    }
  }

  .panel.active {
    background-color: color-mix(
      in srgb-linear,
      var(--muted-color6-low-midground),
      transparent 50%
    );
  }

  .action {
    grid-column: 1;
    grid-row: 1;
  }

  .bonus {
    grid-column: 1;
    grid-row: 2;
  }

  .reaction {
    grid-column: 1;
    grid-row: 3;
  }

  .notes {
    grid-column: 7;
    grid-row: 1 / span 3;
    height: 100%;

    border-left: 1px solid var(--chrome);
    padding-left: var(--horizontal-gap);

    &:hover {
      background-color: var(--input-hover-background);
    }

    &:focus:enabled,
    &:active:enabled {
      background-color: var(--input-active-background);
    }
  }

  .monster,
  .player,
  .lair {
    grid-template-columns: 8rem 9rem 10rem 7rem 6rem 6rem 1fr;

    .legendary {
      grid-column: 2;
    }

    .ac {
      grid-column: 3;
      grid-row: 1;
    }

    .hp {
      grid-column: 3;
      grid-row: 2;
      font-size: calc(var(--base-font-size) * 2);
    }

    .temp-hp {
      grid-column: 3;
      grid-row: 3;
    }

    .str {
      grid-column: 5;
      grid-row: 1;
    }

    .dex {
      grid-column: 5;
      grid-row: 2;
    }

    .con {
      grid-column: 5;
      grid-row: 3;
    }

    .int {
      grid-column: 6;
      grid-row: 1;
    }

    .wis {
      grid-column: 6;
      grid-row: 2;
    }

    .cha {
      grid-column: 6;
      grid-row: 3;
    }

    .damage-button {
      grid-row: 1 / span 2;
      grid-column: 4;
      align-self: center;
      margin-top: var(--vertical-gap);
    }

    .heal-button {
      grid-row: 2 / span 2;
      grid-column: 4;
      align-self: center;
      margin-bottom: var(--vertical-gap);
    }
  }
</style>
