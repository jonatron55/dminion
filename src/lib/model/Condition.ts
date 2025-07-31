export interface Condition {
  name: "blinded" | "bloodied" | "charmed" | "concentrating" | "dead" | "deafened" | "frightened" | "grappled" | "incapacitated" | "invisible" | "marked" | "paralyzed" | "petrified" | "poisoned" | "prone" | "restrained" | "stunned" | "surprised" | "unconscious";
  startTime?: number,
  duration?: number,
  instigator?: string,
}

export const conditionNames: Condition["name"][] = ["blinded", "bloodied", "charmed", "concentrating", "dead", "deafened", "frightened", "grappled", "incapacitated", "invisible", "marked", "paralyzed", "petrified", "poisoned", "prone", "restrained", "stunned", "surprised", "unconscious"];

export function conditionClasses(conditions: Condition[]): string {
  return conditions.map(c => c.name.toLowerCase()).join(" ");
}
