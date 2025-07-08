<script lang="ts">
  import {
    participantName,
    smallPortraitUrl,
    type Participant,
  } from "$lib/game/Participant";

  export let participant: Participant;
  export let isActive: boolean = false;
  export let isSelected: boolean = false;
</script>

<div>
  <img
    class="portrait"
    src={smallPortraitUrl(participant)}
    alt={`${participantName(participant)} portrait`}
  />
</div>
