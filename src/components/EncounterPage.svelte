<script lang="ts">
  import type { Game } from "$lib/game/Game";
  import { participantName, smallPortraitUrl } from "$lib/game/Participant";

  export let game: Game;
</script>

{#each game.participants as participant}
  <div class="participant">
    <h2>{participantName(participant)}</h2>
    <img
      src={smallPortraitUrl(participant)}
      alt={participantName(participant)}
    />
  </div>
{/each}
