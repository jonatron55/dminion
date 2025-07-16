<script lang="ts">
  import { modiferStr, statStr } from "$lib/model/Stats";

  export let label: string;
  export let value: number;
</script>

<div class="stat">
  <span class="label">{label}</span>
  <span class="value">{statStr(value)}</span>
  <span class="modifier">{modiferStr(value)}</span>
</div>

<style lang="scss">
  .stat {
    display: inline-flex;
    flex-direction: row;
    align-items: center;
    align-items: stretch;
    border: 1px solid var(--chrome);
    border-radius: var(--ui-border-radius);
  }

  .label {
    text-transform: uppercase;
    margin: 2px var(--horizontal-gap);
  }

  .value {
    text-align: right;
    background-color: var(--chrome);
    padding: 2px 4px;
    font-weight: bold;
  }

  .modifier {
    text-align: right;
    margin: 2px 4px;
    font-style: italic;
  }
</style>
