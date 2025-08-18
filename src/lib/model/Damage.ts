export type Damage = {
  type: "damage",
  amount: number,
} | {
  type: "halfDamage",
  amount: number,
} | {
  type: "doubleDamage",
  amount: number,
} | {
  type: "kill",
}

export type Healing = {
  type: "heal",
  amount: number,
} | {
  type: "setHp",
  amount: number,
} | {
  type: "setTempHp",
  amount: number,
}
