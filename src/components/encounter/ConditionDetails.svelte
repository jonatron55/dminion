<script lang="ts">
  import type { Condition } from "$lib/model/Condition";

  export let condition: Condition;
</script>

{#if condition.name === "blinded"}
  <ul>
    <li>A blinded creature can't see and automatically fails any ability check that requires sight.</li>
    <li>Attack rolls against the creature have advantage, and the creature's attack rolls have disadvantage.</li>
  </ul>
{:else if condition.name === "bloodied"}
  <ul>
    <li>A creature becomes bloodied when its hit points are reduced below half their maximum.</li>
  </ul>
{:else if condition.name === "charmed"}
  <ul>
    <li>
      A charmed creature can’t attack the charmer or target the charmer with harmful abilities or magical effects.
    </li>
    <li>The charmer has advantage on any ability check to interact socially with the creature.</li>
  </ul>
{:else if condition.name === "concentrating"}
  <ul>
    <li>
      A spellcaster loses concentration on a spell if it casts another spell that requires concentration. A spellcaster
      can’t concentrate on two spells at once.
    </li>
    <li>
      Whenever a spellcaster takes damage while concentrating on a spell, they must make a Constitution saving throw to
      maintain their concentration. The DC equals 10 or half the damage taken, whichever number is higher. If the damage
      is taken from multiple sources, such as an arrow and a dragon's breath, the spellcaster makes a separate saving
      throw for each source of damage.
    </li>
    <li>A spellcaster loses concentration on a spell if they are incapacitated or die.</li>
  </ul>
{:else if condition.name === "dead"}
  <ul>
    <li>This creature is no more.</li>
    <li>It has ceased to be.</li>
    <li>It’s a stiff.</li>
    <li>It’s expired and gone to meet it’s maker.</li>
    <li>Bereft of life, it rests in peace.</li>
    <li>If you hadn’t vapourized it with a fireball, it’d be pushing up the daisies.</li>
    <li>It’s metabolic processes are now of interest only to historians.</li>
    <li>
      It’s kicked the bucket, shuffled off its mortal coil, run down the curtain, and joined the bleeding choir
      invisible.
    </li>
  </ul>
{:else if condition.name === "deafened"}
  <ul>
    <li>A deafened creature can’t hear and automatically fails any ability check that requires hearing.</li>
  </ul>
{:else if condition.name === "frightened"}
  <ul>
    <li>
      A frightened creature has disadvantage on ability checks and attack rolls while the source of its fear is within
      line of sight.
    </li>
    <li>The creature can’t willingly move closer to the source of its fear.</li>
  </ul>
{:else if condition.name === "grappled"}
  <ul>
    <li>A grappled creature’s speed becomes 0, and it can’t benefit from any bonus to its speed.</li>
    <li>The condition ends if the grappler is incapacitated (see the condition).</li>
    <li>
      The condition also ends if an effect removes the grappled creature from the reach of the grappler or grappling
      effect, such as when a creature is hurled away by the thunderwave spell.
    </li>
  </ul>
{:else if condition.name === "incapacitated"}
  <ul>
    <li>An incapacitated creature can’t take actions or reactions.</li>
  </ul>
{:else if condition.name === "invisible"}
  <ul>
    <li>
      An invisible creature is impossible to see without the aid of magic or a special sense. For the purpose of hiding,
      the creature is heavily obscured. The creature’s location can be detected by any noise it makes or any tracks it
      leaves.
    </li>
    <li>Attack rolls against the creature have disadvantage, and the creature’s attack rolls have advantage.</li>
  </ul>
{:else if condition.name === "paralyzed"}
  <ul>
    <li>A paralyzed creature is incapacitated and can’t move or speak.</li>
    <li>The creature automatically fails Strength and Dexterity saving throws.</li>
    <li>Attack rolls against the creature have advantage.</li>
    <li>Any attack that hits the creature is a critical hit if the attacker is within 5 feet of the creature.</li>
  </ul>
{:else if condition.name === "petrified"}
  <ul>
    <li>
      A petrified creature is transformed, along with any nonmagical object it is wearing or carrying, into a solid
      inanimate substance (usually stone). Its weight increases by a factor of ten, and it ceases aging.
    </li>
    <li>The creature is incapacitated (see the condition), can’t move or speak, and is unaware of its surroundings.</li>
    <li>Attack rolls against the creature have advantage.</li>
    <li>The creature automatically fails Strength and Dexterity saving throws.</li>
    <li>The creature has resistance to all damage.</li>
    <li>
      The creature is immune to poison and disease, although a poison or disease already in its system is suspended, not
      neutralized.
    </li>
  </ul>
{:else if condition.name === "poisoned"}
  <ul>
    <li>A poisoned creature has disadvantage on attack rolls and ability checks.</li>
  </ul>
{:else if condition.name === "prone"}
  <ul>
    <li>A prone creature’s only movement option is to crawl, unless it stands up and thereby ends the condition.</li>
    <li>The creature has disadvantage on attack rolls.</li>
    <li>
      An attack roll against the creature has advantage if the attacker is within 5 feet of the creature. Otherwise, the
      attack roll has disadvantage.
    </li>
  </ul>
{:else if condition.name === "restrained"}
  <ul>
    <li>A restrained creature’s speed becomes 0, and it can’t benefit from any bonus to its speed.</li>
    <li>Attack rolls against the creature have advantage, and the creature's attack rolls have disadvantage.</li>
    <li>The creature has disadvantage on Dexterity saving throws.</li>
  </ul>
{:else if condition.name === "stunned"}
  <ul>
    <li>A stunned creature is incapacitated (see the condition), can’t move, and can speak only falteringly.</li>
    <li>
      The creature automatically fails Strength and Dexterity saving throws. • Attack rolls against the creature have
      advantage.
    </li>
  </ul>
{:else if condition.name === "surprised"}
  <ul>
    <li>
      A surprised creature can’t move or take an action on the first turn of the combat, and can't take a reaction until
      that turn ends. A member of a group can be surprised even if the other members aren’t.
    </li>
  </ul>
{:else if condition.name === "unconscious"}
  <ul>
    <li>
      An unconscious creature is incapacitated (see the condition), can’t move or speak, and is unaware of its
      surroundings
    </li>
    <li>The creature drops whatever it’s holding and falls prone.</li>
    <li>The creature automatically fails Strength and Dexterity saving throws.</li>
    <li>Attack rolls against the creature have advantage.</li>
    <li>Any attack that hits the creature is a critical hit if the attacker is within 5 feet of the creature.</li>
  </ul>
{/if}
