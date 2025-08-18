<script lang="ts">
  import { currentTheme } from "$lib/theme";
  import alticon from "../images/alticon.svg";
  import appicon from "../images/icon.svg";

  currentTheme.subscribe((theme) => {
    icon = theme === "noon" || theme === "night" ? alticon : appicon;
  });

  $: icon = appicon;
</script>

<img src={icon} alt="App Icon" />
