export interface Stats {
  str: number,
  dex: number,
  con: number,
  int: number,
  wis: number,
  cha: number,
}

export function modifer(score: number): number {
  if (score >= 10) {
    return Math.floor((score - 10) / 2);
  } else {
    return Math.floor((score - 11) / 2);
  }
}
