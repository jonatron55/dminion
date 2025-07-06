<script lang="ts">
    import type { AppMode } from "$lib/AppMode";
    import Toolbar from "../components/Toolbar.svelte";
    import "../styles/app.scss";
    import { currentTheme } from "../themes/theme";

    let mode: AppMode = "map";
    currentTheme.set("Dungeoneer Dusk");
</script>

<div class="app">
    <Toolbar
        {mode}
        onModeChange={(newMode) => {
            mode = newMode;
        }}
    />

    <main class="panel">
        <p>
            Lorem <span class="color-1">ipsum</span> dolor sit amet, consectetur
            adipiscing elit, sed do eiusmod tempor
            <span class="color-2">incididunt</span>
            ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud
            <span class="color-3">exercitation</span>
            ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure
            dolor in <span class="color-4">reprehenderit</span> in voluptate
            velit esse cillum dolore eu fugiat nulla pariatur.
            <span class="color-5">Excepteur</span>
            sint occaecat cupidatat non proident, sunt in culpa qui
            <span class="color-6">officia deserunt</span> mollit anim id est laborum.
        </p>
        <p>
            Here is a <a href="https://www.lipsum.com/feed/html"
                >hyperlink to somewhere</a
            >.
        </p>
        <table>
            <tbody>
                <tr>
                    <td class="red">Red</td>
                    <td class="green">Green</td>
                    <td class="blue">Blue</td>
                    <td class="cyan">Cyan</td>
                    <td class="purple">Purple</td>
                    <td class="yellow">Yellow</td>
                </tr>
                <tr>
                    <td class="red-background">Red</td>
                    <td class="green-background">Green</td>
                    <td class="blue-background">Blue</td>
                    <td class="cyan-background">Cyan</td>
                    <td class="purple-background">Purple</td>
                    <td class="yellow-background">Yellow</td>
                </tr>
            </tbody>
        </table>
        <hr />
        <button type="button">Button</button>&nbsp;<button
            type="button"
            class="secondary">Secondary</button
        >&nbsp;<button type="button" disabled>Disabled</button>
        <br /><br />
        <button type="button" class="ok">Affirmative</button>&nbsp;<button
            type="button"
            class="danger">Negative</button
        >&nbsp;<button class="caution">Cautionary</button>
        <br /><br />
        <input type="text" placeholder="Input" />&nbsp;<input
            type="text"
            placeholder="Disabled"
            disabled
        /><br /><br />
        <textarea placeholder="Textarea"></textarea><br /><br />
        <input type="checkbox" id="checkbox1" /><label for="checkbox1"
            >Checkbox</label
        >&nbsp;<input type="checkbox" id="checkbox2" disabled /><label
            for="checkbox2">Disabled checkbox</label
        ><br /><br />
        <input type="checkbox" class="toggle-switch" id="checkbox3" /><label
            for="checkbox3">Toggle switch</label
        >&nbsp;<input
            type="checkbox"
            class="toggle-switch"
            id="checkbox4"
            disabled
        /><label for="checkbox4">Toggle switch</label><br /><br />
        <input type="radio" name="radios" id="radio1" /><label for="radio1"
            >Radio1</label
        ><br /><br />
        <input type="radio" name="radios" id="radio2" /><label for="radio2"
            >Radio2</label
        ><br /><br />
        <input type="radio" name="radios" id="radio3" disabled /><label
            for="radio3">Disabled</label
        ><br /><br />
        <hr />
        <p>
            Donec eleifend arcu nec lacus ultricies hendrerit. Aliquam fringilla
            odio risus, nec efficitur enim scelerisque sit amet. Integer tempor
            dui in nisi scelerisque, et consectetur velit eleifend. Integer
            porttitor tincidunt est id auctor. Maecenas sit amet hendrerit est.
            Phasellus in hendrerit enim. Fusce imperdiet viverra erat, et
            condimentum turpis ultricies non. Pellentesque blandit imperdiet
            mollis. Suspendisse metus dui, blandit ac purus eget, dignissim
            molestie dolor. Fusce dignissim ligula augue, et tristique nulla
            efficitur id.
        </p>
        <div class="card">
            <h1 class="caption">Card with Caption</h1>
            <p>
                I am not rightly able to apprehend the kind of confusion of
                ideas that could have provoked such a question.
            </p>
            <div class="buttons-container">
                <button class="secondary-button">Retry</button>&nbsp;
                <button class="secondary-button">Cancel</button>
            </div>
        </div>
    </main>
</div>

<style lang="scss">
    .app {
        display: grid;
        grid-template-rows: auto 1fr;
        gap: var(--vertical-gap) var(--horizontal-gap);
        height: calc(100vh - 2 * var(--vertical-gap));
    }

    main {
        overflow-y: auto;
    }
</style>
