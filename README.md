**⚠️WARNING⚠️**

**This fork adds xdg icon support and color for workspace buttons. I'm not that gret at rust so the code may be very unoptimized. Things are expected not to work or be implemented yet. This is mostly for me to mess around with functional hyprland ricing.**

# hyprland-workspaces
A multi-monitor aware Hyprland workspace widget. Follows the specified monitor and outputs the currently open workspaces. Designed to be used with [Eww](https://github.com/elkowar/eww), but may function with other bars. Compatible with [hyprland-autoname-workspaces](https://github.com/cyrinux/hyprland-autoname-workspaces).

## Installation Instructions
### Dependencies
[Hyprland](https://github.com/hyprwm/Hyprland)
### ~~Arch Linux~~
~~Arch users can install from AUR using your favourite package manager.~~
```
  pikaur -S hyprland-workspaces
```
### Building from source
```
git clone https://github.com/FieldofClay/hyprland-workspaces.git
cd hyprland-workspaces
cargo build --release
```

## Usage
Pass the name of the monitor to follow as the only argument. 
```
./hyprland-workspaces eDP-1
```
If you wish to get all workspaces across all monitors, pass the special argument "_".
```
./hyprland-workspaces _
```
It will then follow that monitor(s) and output the workspaces details in JSON to stdout.
```json
[
  {"active":false,"class":"workspace-button w1","color":[62,165,251],"icon_path":"/etc/profiles/per-user/ben/share/icons/kora/apps/scalable/code.svg","id":1,"name":"1","windows":2
  },
  {"active":false,"class":"workspace-button w2","color":[71,84,226],"icon_path":"/etc/profiles/per-user/ben/share/icons/kora/apps/scalable/webcord.svg","id":2,"name":"2","windows":1
  },
  {"active":false,"class":"workspace-button w3","color":[79,146,26],"icon_path":"/etc/profiles/per-user/ben/share/icons/kora/apps/scalable/spotify.svg","id":3,"name":"3","windows":1
  },
  ...
]

```
You can get the names of your monitors by running:
```
hyprctl monitors -j
```

It can be used as a workspaces widget in Eww with config similar to below.
```yuck
(deflisten workspace0 "hyprland-workspaces `hyprctl monitors -j | jq -r \".[0].name\"`")
(deflisten workspace1 "hyprland-workspaces `hyprctl monitors -j | jq -r \".[1].name\"`")

(defwidget workspaces0 []
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces" :space-evenly false :spacing 2
      (for i in workspace0
        (button
            :class "${i.class} ${i.windows <= 0 ? "empty" : ""}" ; add 'empty' to class if it's empty, for more customizable theming
            :onclick "hyprctl dispatch workspace ${i.id}"
            :visible {i.id <= 0 ? false : true } ; hide special workspace

            (overlay ; overlay to add color tint and icon over the button
                (box :class "highlight"
                    :style "background: rgba(${i.color[0]}, ${i.color[1]}, ${i.color[2]}, 0.15);")
                (box :class "icon"
                    :style "background-image: url('${i.icon_path}')")))))))

;;; old method
(defwidget workspaces0 []
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces"
      (for i in workspace0
        (button
          :onclick "hyprctl dispatch workspace ${i.id}"
          :class "${i.class}"
          "${i.name}")))))

(defwindow bar0 []
  :monitor 0
  (box 
    (workspaces0)
    (other_widget)))
```

The following classes are output, to provide multiple options for theming your workspaces widget.
* `workspace-button`: all workspaces will have this class
* `workspace-active`: only the active workspace will have this class. Will not be present if workspace is active, but focus is on another monitor.
* `w<WORKSPACEID>`: Each workspace will have this class to allow for unique CSS per workspace.
* `wa<WORKSPACEID>`: The active workspace will have this to allow for unique CSS per workspace, when it is active. Like `workspace-active`, this does not appear when the focus is on another monitor.
