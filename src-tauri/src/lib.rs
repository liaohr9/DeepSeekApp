use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 创建托盘菜单
            let show = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("AI 助手")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            match window.is_visible() {
                                Ok(true) => {
                                    let _ = window.hide();
                                }
                                Ok(false) => {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                                Err(e) => {
                                    println!("Error checking window visibility: {:?}", e);
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            // 获取主窗口并设置关闭行为
            if let Some(window) = app.get_webview_window("main") {
                let app_handle = window.app_handle().clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止默认关闭行为
                        api.prevent_close();
                        // 隐藏窗口到系统托盘
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                });
            }

            // 注册全局快捷键
            use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

            let alt_z_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyZ);
            
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app_handle, shortcut, _event| {
                    if shortcut == &alt_z_shortcut {
                        match _event.state() {
                            ShortcutState::Pressed => {
                                // 切换窗口显示/隐藏状态
                                if let Some(window) = _app_handle.get_webview_window("main") {
                                    match window.is_visible() {
                                        Ok(true) => {
                                            // println!("Hiding window");
                                            let _ = window.hide();
                                        }
                                        Ok(false) => {
                                            // println!("Showing window");
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                        Err(e) => {
                                            println!("Error checking window visibility: {:?}", e);
                                        }
                                    }
                                }
                            }
                            ShortcutState::Released => {
                                // println!("Shortcut released");
                            }
                        }
                    }
                })
                .build(),
            )?;
            
            // 尝试注册快捷键，如果失败就忽略（可能已经被注册了）
            let _ = app.global_shortcut().register(alt_z_shortcut);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
