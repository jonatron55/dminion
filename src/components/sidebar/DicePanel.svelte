<!--
  Copyright (c) 2025 Jonathon B. Cobb
  Licensed under the MIT License
-->

<script lang="ts">
  import { DiceViewModel, type HistoryItem } from "$lib/viewmodel/DiceViewModel";

  const diceViewModel = new DiceViewModel();
  let expression = "";
  let history: HistoryItem[] = [];

  const tokenBackspaceOrder = ["d100", "d20", "d12", "d10", "d8", "d6", "d4", "adv", "dis", "kh", "kl"] as const;

  function append(token: string): void {
    expression = `${expression}${token}`;
  }

  function clearExpression(): void {
    expression = "";
  }

  function backspace(): void {
    expression = expression.trimEnd();

    for (const token of tokenBackspaceOrder) {
      if (expression.endsWith(token)) {
        expression = expression.slice(0, -token.length).trimEnd();
        return;
      }
    }

    expression = expression.slice(0, -1).trimEnd();
  }

  async function submit(): Promise<void> {
    let expr = expression.trim();
    if (expr.length === 0) {
      if (history.length === 0) {
        return;
      }

      expr = history[history.length - 1].expression;
    }

    expression = "";

    try {
      await diceViewModel.roll(expr);
      history = diceViewModel.history;
    } catch (e) {
      // Error already captured in history by ViewModel
      history = diceViewModel.history;
    }

    requestAnimationFrame(() => {
      const lastEntry = document.querySelector<HTMLElement>(".dice-panel .history .entry:last-child");
      lastEntry?.scrollIntoView({ block: "end" });
    });
  }

  function onExpressionKeyDown(e: KeyboardEvent): void {
    if (e.key === "Enter") {
      e.preventDefault();
      void submit();
    }
  }
</script>

<div class="dice-panel">
  <div class="history" aria-label="Dice roll history">
    {#each history as item}
      <div class="entry">
        <div class="expression">{item.expression}</div>
        {#if item.roll}
          <div class="rolls">
            {#each item.roll.dice as die}
              <div class="roll">
                <span class="dim">d{die.sides}</span>
                <span class={`badge ${die.keep ? "keep" : "drop"}`}>{die.result}</span>
              </div>
            {/each}
          </div>
          <div class="result">{item.roll.value}</div>
        {:else if item.error}
          <span class="danger">{item.error}</span>
        {/if}
      </div>
    {/each}
  </div>

  <div class="input">
    <div class="display">
      <input
        type="text"
        aria-label="Dice expression"
        bind:value={expression}
        on:keydown={onExpressionKeyDown}
        placeholder="Dice expression"
      />
    </div>

    <div class="keypad" aria-label="Dice keypad">
      <button type="button" class="clear danger" on:click={clearExpression}>C</button>
      <button type="button" class="backspace danger" on:click={backspace}>← BS</button>

      <button type="button" class="ok" on:click={() => append("adv")}>adv</button>
      <button type="button" class="ok" on:click={() => append("dis")}>dis</button>
      <button type="button" class="ok" on:click={() => append("kh")}>kh</button>
      <button type="button" class="ok" on:click={() => append("kl")}>kl</button>

      <button type="button" class="secondary" on:click={() => append("d12")}>d12</button>
      <button type="button" class="secondary" on:click={() => append("d100")}>d100</button>
      <button type="button" class="secondary d20" on:click={() => append("d20")}>d20</button>
      <button type="button" class="ok" on:click={() => append(" ÷ ")}>÷</button>

      <button type="button" class="secondary" on:click={() => append("d8")}>d8</button>
      <button type="button" class="secondary" on:click={() => append("d10")}>d10</button>
      <button type="button" class="ok" on:click={() => append(" × ")}>×</button>

      <button type="button" class="secondary" on:click={() => append("d4")}>d4</button>
      <button type="button" class="secondary" on:click={() => append("d6")}>d6</button>
      <button type="button" class="ok" on:click={() => append(" - ")}>-</button>

      <button type="button" on:click={() => append("7")}>7</button>
      <button type="button" on:click={() => append("8")}>8</button>
      <button type="button" on:click={() => append("9")}>9</button>
      <button type="button" class="plus ok" on:click={() => append(" + ")}>+</button>

      <button type="button" on:click={() => append("4")}>4</button>
      <button type="button" on:click={() => append("5")}>5</button>
      <button type="button" on:click={() => append("6")}>6</button>

      <button type="button" on:click={() => append("1")}>1</button>
      <button type="button" on:click={() => append("2")}>2</button>
      <button type="button" on:click={() => append("3")}>3</button>
      <button type="button" class="enter secondary" on:click={() => void submit()} aria-label="Roll">↲</button>

      <button type="button" class="secondary" on:click={() => append("(")}>(</button>
      <button type="button" on:click={() => append("0")}>0</button>
      <button type="button" class="secondary" on:click={() => append(")")}>)</button>
    </div>
  </div>
</div>

<style lang="scss">
  .dice-panel {
    height: 100%;
    display: grid;
    grid-template-rows: 1fr auto;
    gap: var(--vertical-gap) var(--horizontal-gap);
  }

  .history {
    min-height: 0;
    overflow: auto;

    .entry {
      width: 100%;
      margin-bottom: var(--vertical-gap);
      padding-bottom: var(--vertical-gap);
      border-bottom: 1px solid var(--dim-foreground);
    }

    .rolls {
      display: flex;
      gap: var(--horizontal-gap);
      flex-wrap: wrap;
      margin-top: var(--vertical-gap);
    }

    .roll {
      display: flex;
      gap: 0;
      align-items: center;
    }

    .keep {
      color: var(--chrome-high);
      border-color: var(--chrome-high);
    }

    .drop {
      color: var(--dim-foreground);
      border-color: var(--dim-foreground);
      text-decoration: line-through;
    }

    .result {
      font-weight: bold;
      font-size: 2rem;
      text-align: right;
      margin-top: var(--vertical-gap);
    }
  }

  .input {
    min-height: 0;
    overflow: auto;
    display: grid;
    grid-template-rows: auto 1fr;
    gap: var(--vertical-gap) var(--horizontal-gap);
    overflow: hidden;

    .display input {
      width: 100%;
    }

    .keypad {
      display: grid;
      gap: var(--vertical-gap) var(--horizontal-gap);
      grid-template-columns: repeat(4, 1fr);
      grid-auto-rows: 2.5rem;
      & > button {
        padding: 0;
        width: 100%;
        height: 100%;
        min-width: 0;
      }

      .clear {
        grid-column: span 2;
      }

      .backspace {
        grid-column: span 2;
      }

      .d20 {
        grid-row: span 3;
      }

      .plus {
        grid-row: span 2;
      }

      .enter {
        grid-row: span 2;
      }
    }
  }
</style>
