import { writable } from "svelte/store";
import { dawnTheme } from "./dawn";
import { duskTheme } from "./dusk";
import { nightTheme } from "./night";
import { noonTheme } from "./noon";

export type FontSize = "small" | "medium" | "large";

export interface Theme {
    name: string,
    backgroundImage: string,
    primaryForeground: string,
    intenseForeground: string,
    dimForeground: string,
    contentBackground: string,
    cardBackground: string,
    pageBackground: string,
    selectionBackground: string,
    dimSelectionBackground: string,
    contentBackgroundTransparent: string,
    cardBackgroundTransparent: string,
    chromeForeground: string,
    chromeHigh: string,
    chrome: string,
    chromeShadow: string,
    chromeLow: string,
    chromeBackground: string,
    captionForeground: string,
    link: string,
    linkHover: string,
    linkActive: string,
    linkDisabled: string,
    label: string,
    labelHover: string,
    labelActive: string,
    labelDisabled: string,
    controlForeground: string,
    controlBackground: string,
    controlHoverForeground: string,
    controlHoverBackground: string,
    controlActiveForeground: string,
    controlActiveBackground: string,
    controlDisabledForeground: string,
    controlDisabledBackground: string,
    secondaryControlForeground: string,
    secondaryControlBackground: string,
    secondaryControlHoverForeground: string,
    secondaryControlHoverBackground: string,
    secondaryControlActiveForeground: string,
    secondaryControlActiveBackground: string,
    secondaryControlDisabledForeground: string,
    secondaryControlDisabledBackground: string,
    okControlForeground: string,
    okControlBackground: string,
    okControlHoverForeground: string,
    okControlHoverBackground: string,
    okControlActiveForeground: string,
    okControlActiveBackground: string,
    okControlDisabledForeground: string,
    okControlDisabledBackground: string,
    dangerControlForeground: string,
    dangerControlBackground: string,
    dangerControlHoverForeground: string,
    dangerControlHoverBackground: string,
    dangerControlActiveForeground: string,
    dangerControlActiveBackground: string,
    dangerControlDisabledForeground: string,
    dangerControlDisabledBackground: string,
    cautionControlForeground: string,
    cautionControlBackground: string,
    cautionControlHoverForeground: string,
    cautionControlHoverBackground: string,
    cautionControlActiveForeground: string,
    cautionControlActiveBackground: string,
    cautionControlDisabledForeground: string,
    cautionControlDisabledBackground: string,
    okShadow: string,
    warningShadow: string,
    errorShadow: string,
    accent1Shadow: string,
    accent2Shadow: string,
    accent3Shadow: string,
    accent4Shadow: string,
    accent5Shadow: string,
    accent6Shadow: string,
    redShadow: string,
    greenShadow: string,
    blueShadow: string,
    cyanShadow: string,
    purpleShadow: string,
    yellowShadow: string,
    dimShadow: string,
    intenseShadow: string,
    inputBackground: string,
    inputBorder: string,
    inputHoverBackground: string,
    inputHoverBorder: string,
    inputActiveBackground: string,
    inputActiveBorder: string,
    accent1Foreground: string,
    accent2Foreground: string,
    accent3Foreground: string,
    accent4Foreground: string,
    accent5Foreground: string,
    accent6Foreground: string,
    accent1Background: string,
    accent2Background: string,
    accent3Background: string,
    accent4Background: string,
    accent5Background: string,
    accent6Background: string,
    okForeground: string,
    okBackground: string,
    warningForeground: string,
    warningBackground: string,
    errorForeground: string,
    errorBackground: string,
    redForeground: string,
    greenForeground: string,
    blueForeground: string,
    cyanForeground: string,
    purpleForeground: string,
    yellowForeground: string,
    redBackground: string,
    greenBackground: string,
    blueBackground: string,
    cyanBackground: string,
    purpleBackground: string,
    yellowBackground: string,
};

export const allThemes = [nightTheme, noonTheme, duskTheme, dawnTheme];
export const defaultTheme = allThemes[0];
export const currentTheme = writable<string>("dusk");
export const currentFontSize = writable<FontSize>("medium");

currentTheme.subscribe(newTheme => {
    const theme = allThemes.find((t) => t.name === newTheme) || defaultTheme;

    function toCssProperty(key: string) {
        return "--" + key.replace(/([A-Z])/g, "-$1").replace(/accent([0-9])/, "accent-$1").toLowerCase();
    }

    for (let [key, value] of Object.entries(theme)) {
        if (key !== "name" && typeof value === "string") {
            document.documentElement.style.setProperty(
                toCssProperty(key),
                value.toString(),
            );
        }
    }
});

const fontSizes = {
    small: "11px",
    medium: "12px",
    large: "14px",
};

currentFontSize.subscribe(newFontSize => {
    document.documentElement.style.setProperty(
        "--base-font-size",
        fontSizes[newFontSize],
    );
});

currentTheme.set(defaultTheme.name);
currentFontSize.set("medium");

// Some globals
document.documentElement.style.setProperty("--base-font", "IosevkaArcane");
document.documentElement.style.setProperty("--base-line-height", "1.19");
document.documentElement.style.setProperty("--ui-border-width", "2px");
document.documentElement.style.setProperty("--ui-border-radius", "0px");
document.documentElement.style.setProperty("--button-border-radius", "4px");
document.documentElement.style.setProperty("--panel-border-width", "3px 1px 3px 1px");
document.documentElement.style.setProperty("--panel-border-style", "solid");
document.documentElement.style.setProperty("--panel-border-radius", "2px");
document.documentElement.style.setProperty("--ui-transition-time", "50ms");
document.documentElement.style.setProperty("--ui-hydrate-time", "100ms");
