# Hyprsnap snapping-tool

NOTE! This is specifically for ultra widescreen monitors (5120x1440) running Hyprland.

Hyprsnap is a lightweight, GTK-based overlay for the Hyprland window manager for widescreen monitors. It provides a quick, visual way to snap your active window to predefined screen locations using a grid of buttons or corresponding number keys.

The overlay appears on top of your current windows without stealing focus, allowing for seamless window management. Once you click a button or press a key, the command is executed, and the overlay closes.



---

## Features

-   **Visual Grid Overlay**: A simple grid of buttons to visually select window placement.
-   **Focus-Aware**: Uses `gtk4-layer-shell` to display as an overlay, ensuring it doesn't steal focus from your active window.
-   **Keyboard Shortcuts**: Each button is mapped to a number key (1-7) for fast, keyboard-driven operation.
-   **Hyprland Integration**: Uses `hyprctl` to send commands directly to the Hyprland compositor.
-   **Quit on Escape**: Press the `Esc` key at any time to close the overlay without performing an action.

