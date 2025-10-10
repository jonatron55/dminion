// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

export type AppMode = "map" | "encounter" | "trade" | "library";
export type AppSidebarMode = undefined | "dice" | "namegen" | "settings";
export const appModes: AppMode[] = ["map", "encounter", "trade", "library"];
export const appSidebarModes: AppSidebarMode[] = ["dice", "namegen", "settings"];
