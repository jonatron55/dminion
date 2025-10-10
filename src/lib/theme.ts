// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

import { writable } from "svelte/store";

export type Theme = "dusk" | "dawn" | "night" | "noon";
export type FontSize = "small" | "medium" | "large";

export const Themes = ["dusk", "dawn", "night", "noon"];
export const FontSizes = ["small", "medium", "large"];

export const ThemeNames = {
  "Dungeoneer Dusk": "dusk",
  "Dungeoneer Dawn": "dawn",
  "Dungeoneer Night": "night",
  "Dungeoneer Noon": "noon",
}

export const defaultTheme: Theme = "dusk";
export const currentTheme = writable<Theme>("dusk");
export const currentFontSize = writable<FontSize>("medium");

currentTheme.subscribe(theme => {
  document.body.className = theme;
});

const fontSizes = {
  small: "11px",
  medium: "13px",
  large: "15px",
};

currentFontSize.subscribe(newFontSize => {
  document.documentElement.style.setProperty(
    "--base-font-size",
    fontSizes[newFontSize],
  );
});

currentTheme.set(defaultTheme);
currentFontSize.set("medium");
