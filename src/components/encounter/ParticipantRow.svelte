<script lang="ts">
  import { conditionClasses } from "$lib/model/Condition";
  import {
    LairViewModel,
    MonsterViewModel,
    ParticipantViewModel,
    PlayerViewModel,
  } from "$lib/viewmodel/ParticipantViewModel";
  import Stat from "./Stat.svelte";

  export let participant: ParticipantViewModel;
  export let isActive: boolean = false;
  export let isSelected: boolean = false;

  let monster: MonsterViewModel = participant as MonsterViewModel;
  let player: PlayerViewModel = participant as PlayerViewModel;
  let lair: LairViewModel = participant as LairViewModel;

  $: monster: participant as MonsterViewModel;
  $: player: participant as PlayerViewModel;
  $: lair: participant as LairViewModel;
</script>

<div class="participant">
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

  <div class="panel {conditionClasses(participant.conditions)}">
    <div class="caption">
      <span class="initiative {conditionClasses(participant.conditions)}"
        >{participant.initiative}.</span
      >
      <span class="name {conditionClasses(participant.conditions)}"
        >{participant.name}</span
      >
      {#if participant.conditions.length > 0}
        <span class="conditions">
          {#each participant.conditions as condition}
            <span class="{condition.name} badge">
              {condition.name}
              {#if condition.duration}
                <span class="count">{condition.duration}s</span>
              {/if}
            </span>
          {/each}
        </span>
      {/if}
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

        {#each Array(3)
          .fill(0)
          .map((_, i) => i) as i}
          <div class="legendary">
            <input
              type="checkbox"
              id="{monster.name}-legendary-{i}"
              checked={monster.legendaryActions >= i + 1}
              disabled={monster.totalLegendaryActions <= i}
            />
            <label for="{monster.name}-legendary-{i}">Legendary</label>
          </div>
        {/each}

        <div>
          <input
            type="checkbox"
            id="{monster.name}-reaction"
            bind:checked={monster.reaction}
          />
          <label for="{monster.name}-reaction">Reaction</label>
        </div>
        <div class="ac"><strong>AC:</strong> {monster.ac}</div>
        <div class="hp"><strong>HP:</strong> {monster.hp}</div>
        <div class="temp-hp"><strong>temp:</strong> {monster.tempHp}</div>
        <button class="danger damage-button">Damage</button>
        <button class="ok heal-button">Heal</button>
        <!-- <div class="str">
          STR:
          <strong>{statStr(monster.stats.str)}</strong>
          [<em>{modiferStr(monster.stats.str)}</em>]
        </div> -->
        <div class="str"><Stat label="str" value={monster.stats.str} /></div>
        <div class="dex"><Stat label="dex" value={monster.stats.dex} /></div>
        <div class="con"><Stat label="con" value={monster.stats.con} /></div>
        <div class="int"><Stat label="int" value={monster.stats.int} /></div>
        <div class="wis"><Stat label="wis" value={monster.stats.wis} /></div>
        <div class="cha"><Stat label="cha" value={monster.stats.cha} /></div>
      </div>
    {:else if participant instanceof PlayerViewModel}
      <div class="content player">Onward!</div>
    {:else if participant instanceof LairViewModel}
      <div class="content lair">Loom, loom</div>
    {/if}
  </div>
</div>

<style lang="scss">
  .participant {
    display: grid;
    grid-template-columns: 8rem auto 1fr;
    gap: var(--vertical-gap) var(--horizontal-gap);
  }

  .active-indicator {
    margin: auto;
    font-size: calc(var(--base-font-size) * 2);
    color: var(--page-background);
    background-color: var(--chrome);
    display: block;
    text-align: right;
    padding: calc(var(--vertical-gap) * 2) calc(var(--horizontal-gap) * 2);
    border-radius: 3rem;
    border: double 8px var(--page-background);
  }

  .portrait {
    width: 8rem;
    height: 8rem;
  }

  .caption {
    font-weight: normal;
  }

  .initiative {
    font-weight: bold;

    &.dead {
      text-decoration: line-through;
    }
  }

  .name {
    font-weight: normal;

    &.dead {
      text-decoration: line-through;
    }
  }

  .conditions {
    display: inline-flex;
    gap: var(--horizontal-gap);
    margin-left: var(--horizontal-gap);
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

  .monster {
    grid-template-columns: repeat(5, auto) 1fr;

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
      grid-row: 2;
      grid-column: 4;
    }

    .heal-button {
      grid-row: 3;
      grid-column: 4;
    }
  }
</style>
