export const SecondsPerRound = 6;

export function formatDuration(rounds: number): string {
  const totalSeconds = rounds * SecondsPerRound;
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  if (hours > 0) {
    return `${hours}h ${minutes}m ${seconds}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${seconds}s`;
  } else {
    return `${seconds}s`;
  }
}

export function formatTime(rounds: number): string {
  const totalSeconds = rounds * SecondsPerRound;
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  const pad = (num: number) => num.toString().padStart(2, "0");

  if (hours > 0) {
    return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
  } else {
    return `${minutes}:${pad(seconds)}`;
  }
}

export function totalRounds(hours: number, minutes: number, seconds: number): number {
  const totalSeconds = hours * 3600 + minutes * 60 + seconds;
  return Math.ceil(totalSeconds / SecondsPerRound);
}
