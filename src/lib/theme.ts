// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { get, writable } from "svelte/store";

export type Theme = "Arcane" | "Dungeoneer" | "High Contrast" | "Intrepid" | "Mystic" | "Woodland";
export type ThemeMode = "Dawn" | "Dusk" | "Noon" | "Night" | "Dark" | "Light";
export type FontSize = "Small" | "Medium" | "Large";
export type FontStyle = "Serif" | "Sans";
export type RulesVersion = "srd51" | "srd52";

export const themes = [
  "Arcane",
  "Dungeoneer",
  "High Contrast",
  "Intrepid",
  "Mystic",
  "Woodland",
];
export const modes = [
  "Dawn",
  "Dusk",
  "Noon",
  "Night",
  "Dark",
  "Light",
];
export const fontSizes = ["Small", "Medium", "Large"];
export const fontStyles = ["Serif", "Sans"];

export const defaultTheme: Theme = "Dungeoneer";
export const currentTheme = writable<Theme>("Dungeoneer");
export const currentThemeMode = writable<ThemeMode>("Dusk");
export const currentFontSize = writable<FontSize>("Medium");
export const currentFontStyle = writable<FontStyle>("Serif");
export const currentRulesVersion = writable<RulesVersion>("srd51");

export function toId(name: string): string {
  return name.toLowerCase().replace(" ", "-");
}

export function availableModes(theme: Theme): ThemeMode[] {
  if (theme === "High Contrast") {
    return ["Dark", "Light"];
  } else {
    return ["Dawn", "Dusk", "Noon", "Night"];
  }
}

export function isDarkMode(mode: ThemeMode): boolean {
  return mode === "Dusk" || mode === "Night" || mode === "Dark";
}

currentTheme.subscribe(theme => {
  const mode = get(currentThemeMode);
  updateTheme(theme, mode);
});

currentThemeMode.subscribe(mode => {
  const theme = get(currentTheme);
  updateTheme(theme, mode);
});

function updateTheme(theme: Theme, mode: ThemeMode) {
  let classes = [toId(theme) + "-" + toId(mode)];
  if (isDarkMode(mode)) {
    classes.push("dark");
  } else {
    classes.push("light");
  }

  if (theme === "High Contrast") {
    classes.push("high-contrast");
  }

  document.body.className = classes.join(" ");
}

const fontPixelSizes = {
  Small: "11px",
  Medium: "13px",
  Large: "15px",
};

currentFontSize.subscribe(newFontSize => {
  document.documentElement.style.setProperty(
    "--base-font-size",
    fontPixelSizes[newFontSize],
  );
});

currentFontStyle.subscribe(newFontStyle => {
  switch (newFontStyle) {
    case "Serif":
      document.documentElement.style.setProperty(
        "--base-font",
        "SerifBodyFont",
      );
      break;
    case "Sans":
      document.documentElement.style.setProperty(
        "--base-font",
        "SansBodyFont",
      );
      break;
  }
});

currentTheme.set(defaultTheme);
currentFontSize.set("Medium");
currentFontStyle.set("Serif");
