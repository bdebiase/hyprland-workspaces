use hyprland::data::{Monitors, Workspace, Workspaces, Clients};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::HyprData;
use hyprland::shared::HyprDataActive;
use hyprland::Result;
use std::env;
use serde::Serialize;
use serde_json::json;

// CUSTOM IMPORTS
use std::fs;
use image::{open, Rgb};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const HELP: &str = "\
hyprland-workspaces: a multi monitor aware hyprland workspaces json widget generator for eww/waybar.

USAGE:
  hyprland-workspaces MONITOR

FLAGS:
  -h, --help            Prints help information

ARGS:
  <MONITOR>             Monitor to track windows/workspaces on or _ to track all monitors
";

#[derive(Serialize)]
struct WorkspaceCustom {
    pub name: String,
    pub id: i32,
    pub active: bool,
    pub class: String,

    // CUSTOM
    pub windows: u16,
    pub icon_path: String,
    pub color: [u8; 3],
}

fn output(monitor: &str) {
    // get all workspaces
    let mut workspaces: Vec<_> = Workspaces::get().expect("unable to get workspaces").into_iter().collect();
    workspaces.sort_by_key(|w| w.id);

    //get active workspace
    let active_workspace_id: i32;
    if monitor == "_" {
        active_workspace_id = Workspace::get_active().expect("unable to get active workspace").id;
    } else {
        active_workspace_id = Monitors::get()
            .expect("unable to get monitors")
            .find(|m| m.name == monitor)
            .unwrap()
            .active_workspace
            .id;
    }
    //active monitor name
    let active_monitor_name = Monitors::get()
        .expect("unable to get monitors")
        .find(|m| m.focused == true)
        .unwrap()
        .name;

    //let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();
    let mut out_workspaces: Vec<WorkspaceCustom> = (1..=10).map(|id| {
        WorkspaceCustom {
            name: format!("Workspace {}", id),
            id: id,
            active: false,
            class: format!("workspace-button w{}", id),
    
            // CUSTOM
            windows: 0,
            icon_path: "".to_string(),
            color: [0, 0, 0],
        }
    }).collect();

    for workspace in workspaces.iter().filter(|m| m.monitor == monitor || monitor == "_") {
        let mut class = format!("workspace-button w{}" ,workspace.id);
        let mut active = false;

        // CUSTOM IMPLEMENTATION
        let mut clients = Clients::get().expect("unable to get clients").into_iter();
        let last_client = match workspace.windows {
            1 => clients.find(|client| client.workspace.id == workspace.id),
            _ => clients.find(|client| client.title == workspace.last_window_title),
        };

        let mut icon_path = "".to_string();
        let mut color = [0, 0, 0];
        
        if let Some(client) = last_client {
            let mut class_name = client.initial_class;
            class_name = match class_name.as_str() {
                "code-url-handler" => "code".to_string(),
                "kitty-temp" => "kitty".to_string(),
                "WebCord" => "webcord".to_string(),
                _ => class_name,
            };

            if class_name == "" {
                class_name = client.initial_title;
            }

            match linicon::lookup_icon(class_name)
                .next() {
                Some(Ok(icon)) => {
                    match icon.path.to_str() {
                        Some(path_str) => {
                            icon_path = path_str.to_string();  
                            color = get_primary_color_svg(icon_path.clone()).unwrap_or([0, 0, 0]);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if active_workspace_id == workspace.id && (active_monitor_name == monitor || monitor == "_") {
            class = format!("{} workspace-active wa{}", class, workspace.id);
            active = true;
        }

        let ws: WorkspaceCustom = WorkspaceCustom {
            name: workspace.name.clone(),
            id: workspace.id,
            active,
            class,

            // CUSTOM
            windows: workspace.windows,
            icon_path,
            color,
        };

        // Find and update the corresponding workspace in out_workspaces
        if let Some(out_workspace) = out_workspaces.iter_mut().find(|w| w.id == ws.id) {
            *out_workspace = ws;
        }

        //out_workspaces.push(ws);
    }
    println!("{}", json!(out_workspaces).to_string());
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    //check args
    if args.len() != 2 || args[1].eq("-h") || args[1].eq("--help") {
        println!("{HELP}");
        std::process::exit(0);
    }

    let mon = env::args().nth(1).unwrap();
    if let None = Monitors::get()
        .expect("unable to get monitors")
        .find(|m| m.name == mon || mon == "_") {
            println!("Unable to find monitor {mon}");
            std::process::exit(0);
    }

    macro_rules! output {
        () => {
            output(&env::args().nth(1).unwrap());
        };
    }
    output!();
    // Create a event listener
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_added_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_destroy_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_moved_handler(|_, _| {
        output!();
    });
    event_listener.add_monitor_added_handler(|_, _| {
        output!();
    });
    event_listener.add_monitor_removed_handler(|_, _| {
        output!();
    });
    event_listener.add_window_close_handler(|_, _| {
        output!();
    });
    event_listener.add_window_open_handler(|_, _| {
        output!();
    });
    event_listener.add_active_monitor_change_handler(|_, _| {
        output!();
    });
    event_listener.add_active_window_change_handler(|_, _| {
        output!();
    });
    event_listener.add_window_close_handler(|_, _| {
        output!();
    });
    event_listener.add_fullscreen_state_change_handler(|_, _| {
        output!();
    });
    event_listener.add_window_moved_handler(|_, _| {
        output!();
    });
    event_listener.add_layer_open_handler(|_, _| {
        output!();
     });
    event_listener.add_layer_closed_handler(|_, _| {
        output!();
     });
    event_listener.add_urgent_state_handler(|_, _| {
        output!();
    });
    event_listener.add_window_title_change_handler(|_, _| {
        output!();
    });

    event_listener.start_listener()
    
}

fn get_primary_color_svg(string_path: String) -> Option<[u8; 3]> {
    if string_path == "" {
        return None;
    }

    let svg =
        nsvg::parse_file(Path::new(&string_path), nsvg::Units::Pixel, 96.0)
            .unwrap();

    let image = svg.rasterize(2.0).unwrap();
    let mut color_counts = HashMap::new();
    for pixel in image.pixels() {
        if pixel.data[3] <= 1 {
            continue;
        }

        let color = Rgb([pixel[0], pixel[1], pixel[2]]);

        let threshold = 100;
        if color[0] > threshold && color[1] > threshold && color[2] > threshold {
            continue;
        }

        *color_counts.entry(color).or_insert(0) += 1;
    }

    let (primary_color, _) = color_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    Some([primary_color[0], primary_color[1], primary_color[2]])
}

fn get_primary_color_png(string_path: String) -> Option<[u8; 3]> {
    if string_path == "" {
        return None;
    }

    let img = open(Path::new(&string_path)).unwrap().into_rgb8();
    let mut color_counts = HashMap::new();

    for pixel in img.pixels() {
        let color = Rgb([pixel[0], pixel[1], pixel[2]]);

        if color == Rgb([255, 255, 255]) {
            continue;
        }

        *color_counts.entry(color).or_insert(0) += 1;
    }

    let (primary_color, _) = color_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    Some([primary_color[0], primary_color[1], primary_color[2]])
}
