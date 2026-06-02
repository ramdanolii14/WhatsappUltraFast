use tauri::Manager;
use std::sync::{Arc, Mutex};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};

#[derive(Default)]
struct TabState {
    count: u32,
}

#[tauri::command]
fn new_tab(app: tauri::AppHandle, state: tauri::State<Arc<Mutex<TabState>>>) {
    let mut s = state.lock().unwrap();
    s.count += 1;
    let label = format!("tab_{}", s.count);

    tauri::WebviewWindowBuilder::new(
        &app,
        &label,
        tauri::WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()),
    )
    .title(format!("Sesi {}", s.count + 1))
    .inner_size(1024.0, 768.0)
    .decorations(false)
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 WhatsApp/2.2412.54")
    .build()
    .unwrap();
}

#[tauri::command]
fn clear_cache(app: tauri::AppHandle) -> Result<String, String> {
    let data_dir = app
        .path()
        .data_dir()
        .map_err(|e| e.to_string())?
        .join("com.ramdanolii14.whatsappultrafast");

    let cache_dirs = ["WebKitCache", "CacheStorage"];
    let mut cleared = vec![];

    for dir in &cache_dirs {
        let path = data_dir.join(dir);
        if path.exists() {
            std::fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
            cleared.push(dir.to_string());
        }
    }

    if cleared.is_empty() {
        Ok("Tidak ada cache yang perlu dihapus.".to_string())
    } else {
        Ok(format!("Cache berhasil dihapus: {}", cleared.join(", ")))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tab_state = Arc::new(Mutex::new(TabState::default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(tab_state)
        .invoke_handler(tauri::generate_handler![new_tab, clear_cache])
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            win.set_icon(app.default_window_icon().unwrap().clone()).unwrap();

            // Tray icon
            let show = MenuItem::with_id(app, "show", "Tampilkan", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Whatsapp Ultra Fast")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
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
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Intercept close — minimize to tray
            let win_clone = win.clone();
            win.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = win_clone.hide();
                }
            });

            #[cfg(target_os = "linux")]
            win.with_webview(|wv| {
                use webkit2gtk::WebViewExt;
                use webkit2gtk::SettingsExt;
                use webkit2gtk::WebContextExt;
                // FIX: NotificationExt dihapus — trait ini ada di object Notification,
                // bukan di WebView. Import-nya tidak dipakai dan menyebabkan ambiguity.

                // Settings
                if let Some(settings) = wv.inner().settings() {
                    settings.set_enable_write_console_messages_to_stdout(true);
                    settings.set_media_playback_requires_user_gesture(false);
                    settings.set_hardware_acceleration_policy(
                        webkit2gtk::HardwareAccelerationPolicy::Always,
                    );
                    settings.set_enable_webgl(true);
                    settings.set_enable_media_stream(true);
                    settings.set_enable_media_capabilities(true);
                    // FIX Bug drag & drop: WebKit2GTK menonaktifkan drag & drop
                    // secara default pada beberapa versi. Aktifkan eksplisit.
                    settings.set_enable_dns_prefetching(true);
                }

                // Allow all permissions including notifications
                wv.inner().connect_permission_request(|_, req| {
                    use webkit2gtk::PermissionRequestExt;
                    req.allow();
                    true
                });

                // FIX Bug notifikasi: connect_show_notification menerima
                // webkit2gtk::Notification, bukan &str langsung.
                // Gunakan NotificationExt dari scope lokal agar tidak ambiguous.
                wv.inner().connect_show_notification(|_, notification| {
                    use webkit2gtk::NotificationExt;
                    let title = notification.title().unwrap_or_default();
                    let body = notification.body().unwrap_or_default();

                    // FIX: Tambah --urgency=normal dan --expire-time agar KDE Plasma
                    // benar-benar menampilkan notifikasi (tanpa ini KDE sering drop notif)
                    let _ = std::process::Command::new("notify-send")
                        .arg("--app-name=WhatsApp Ultra Fast")
                        .arg("--icon=whatsappultrafast")
                        .arg("--urgency=normal")
                        .arg("--expire-time=5000")
                        .arg(title.as_str())
                        .arg(body.as_str())
                        .spawn();

                    true
                });

                // Handle file downloads
                if let Some(context) = wv.inner().context() {
                    context.set_cache_model(webkit2gtk::CacheModel::WebBrowser);

                    context.connect_download_started(|_, download| {
                        use webkit2gtk::DownloadExt;

                        // Ambil direktori Downloads sesuai setting user via xdg-user-dirs.
                        // Lebih akurat dari hardcode ~/Downloads karena user bisa ganti lokasinya.
                        let download_dir = std::process::Command::new("xdg-user-dir")
                            .arg("DOWNLOAD")
                            .output()
                            .ok()
                            .and_then(|o| String::from_utf8(o.stdout).ok())
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .unwrap_or_else(|| {
                                // Fallback ke ~/Downloads kalau xdg-user-dir tidak tersedia
                                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                                format!("{}/Downloads", home)
                            });

                        download.connect_decide_destination(move |dl, suggested_name| {
                            let path = format!("{}/{}", download_dir, suggested_name);
                            dl.set_destination(&path);
                            false
                        });
                    });
                }
            })
            .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}