use crate::{
    cmds,
    config::Config,
    feat,
    utils::{dirs, resolve},
};
use anyhow::Result;
use tauri::{
    api, AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

pub struct Tray {}

impl Tray {
    pub fn tray_menu(app_handle: &AppHandle) -> SystemTrayMenu {
	let cn = { Config::verge().latest().language == Some("cn".into()) };
        let tw = { Config::verge().latest().language == Some("tw".into()) };
        let version = app_handle.package_info().version.to_string();

        macro_rules! t {
            ($tw: expr, $cn: expr, $en: expr) => {
                if cn {
                    $cn
                } else {
		    if tw {
			$tw
		    } else {
		    	$en
		    }
		}
            };
        }

        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new(
                "open_window",
                t!("显示主窗口", "打開主視窗", "Dashboard"),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "rule_mode",
                t!("规则模式","分流模式", "Rule Mode"),
            ))
            .add_item(CustomMenuItem::new(
                "global_mode",
                t!("全局模式", "全局模式", "Global Mode"),
            ))
            .add_item(CustomMenuItem::new(
                "direct_mode",
                t!("直连模式", "直連模式", "Direct Mode"),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "system_proxy",
                t!("打开系统代理", "啟用系統代理", "System Proxy"),
            ))
            .add_item(CustomMenuItem::new("tun_mode", t!("啟用 Tun 模式", "打开 Tun 模式", "TUN Mode")))
            .add_item(CustomMenuItem::new(
                "copy_env",
                t!("复制环境变量", "複製終端機代理指令", "Copy Env"),
            ))
            .add_submenu(SystemTraySubmenu::new(
                t!("打开目录", "打開檔案位置", "Open Dir"),
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new(
                        "open_logs_dir",
                        t!("日志目录", "連線記錄所在位置", "Logs Dir"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "open_app_dir",
                        t!("应用目录", "應用程式所在位置", "App Dir"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "open_core_dir",
                        t!("内核目录", "Clash 核心所在位置", "Core Dir"),
                    )),
            ))
            .add_submenu(SystemTraySubmenu::new(
                t!("更多", "更多功能", "More"),
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new(
                        "restart_clash",
                        t!("重启 Clash", "重啟 Clash", "Restart Clash"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "restart_app",
                        t!("重启 APP", "重啟 APP", "Restart App"),
                    ))
                    .add_item(
                        CustomMenuItem::new("app_version", format!("Version {version}")).disabled(),
                    ),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", t!("退出", "結束", "Quit")).accelerator("CmdOrControl+Q"))
    }

    pub fn update_systray(app_handle: &AppHandle) -> Result<()> {
        app_handle
            .tray_handle()
            .set_menu(Tray::tray_menu(app_handle))?;
        Tray::update_part(app_handle)?;
        Ok(())
    }

    pub fn update_part(app_handle: &AppHandle) -> Result<()> {
        let tw = { Config::verge().latest().language == Some("tw".into()) };
	let cn = { Config::verge().latest().language == Some("cn".into()) };

        let version = app_handle.package_info().version.to_string();

        macro_rules! t {
            ($tw: expr, $cn: expr, $en: expr) => {
                if cn {
                    $cn
                } else {
		    if tw {
			$tw
		    } else {
		    	$en
		    }
		}
            };
        }

        let mode = {
            Config::clash()
                .latest()
                .0
                .get("mode")
                .map(|val| val.as_str().unwrap_or("rule"))
                .unwrap_or("rule")
                .to_owned()
        };

        let tray = app_handle.tray_handle();

        let _ = tray.get_item("rule_mode").set_selected(mode == "rule");
        let _ = tray.get_item("global_mode").set_selected(mode == "global");
        let _ = tray.get_item("direct_mode").set_selected(mode == "direct");

        let verge = Config::verge();
        let verge = verge.latest();
        let system_proxy = verge.enable_system_proxy.as_ref().unwrap_or(&false);
        let tun_mode = verge.enable_tun_mode.as_ref().unwrap_or(&false);
        let common_tray_icon = verge.common_tray_icon.as_ref().unwrap_or(&false);
        let sysproxy_tray_icon = verge.sysproxy_tray_icon.as_ref().unwrap_or(&false);
        let tun_tray_icon = verge.tun_tray_icon.as_ref().unwrap_or(&false);

        let mut indication_icon = if *system_proxy {
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/tray-icon-sys.png").to_vec();
            #[cfg(target_os = "macos")]
            let mut icon = include_bytes!("../../icons/mac-tray-icon-sys.png").to_vec();
            if *sysproxy_tray_icon {
                let path = dirs::app_home_dir()?.join("icons").join("sysproxy.png");
                if path.exists() {
                    icon = std::fs::read(path).unwrap();
                }
            }
            icon
        } else {
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/tray-icon.png").to_vec();
            #[cfg(target_os = "macos")]
            let mut icon = include_bytes!("../../icons/mac-tray-icon.png").to_vec();
            if *common_tray_icon {
                let path = dirs::app_home_dir()?.join("icons").join("common.png");
                if path.exists() {
                    icon = std::fs::read(path).unwrap();
                }
            }
            icon
        };

        if *tun_mode {
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/tray-icon-tun.png").to_vec();
            #[cfg(target_os = "macos")]
            let mut icon = include_bytes!("../../icons/mac-tray-icon-tun.png").to_vec();
            if *tun_tray_icon {
                let path = dirs::app_home_dir()?.join("icons").join("tun.png");
                if path.exists() {
                    icon = std::fs::read(path).unwrap();
                }
            }
            indication_icon = icon
        }

        let _ = tray.set_icon(tauri::Icon::Raw(indication_icon));

        let _ = tray.get_item("system_proxy").set_selected(*system_proxy);
        let _ = tray.get_item("tun_mode").set_selected(*tun_mode);

        let switch_map = {
            let mut map = std::collections::HashMap::new();
            map.insert(true, "on");
            map.insert(false, "off");
            map
        };

        let _ = tray.set_tooltip(&format!(
            "Clash Verge {version}\n{}: {}\n{}: {}",
            t!("系统代理", "系統代理", "System Proxy"),
            switch_map[system_proxy],
            t!("Tun 模式", "Tun 模式", "TUN Mode"),
            switch_map[tun_mode]
        ));

        Ok(())
    }

    pub fn on_left_click(app_handle: &AppHandle) {
        let tray_event = { Config::verge().latest().tray_event.clone() };
        let tray_event = tray_event.unwrap_or("main_window".into());
        match tray_event.as_str() {
            "system_proxy" => feat::toggle_system_proxy(),
            "tun_mode" => feat::toggle_tun_mode(),
            "main_window" => resolve::create_window(app_handle),
            _ => {}
        }
    }

    pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::LeftClick { .. } => Tray::on_left_click(app_handle),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                mode @ ("rule_mode" | "global_mode" | "direct_mode") => {
                    let mode = &mode[0..mode.len() - 5];
                    feat::change_clash_mode(mode.into());
                }
                "open_window" => resolve::create_window(app_handle),
                "system_proxy" => feat::toggle_system_proxy(),
                "tun_mode" => feat::toggle_tun_mode(),
                "copy_env" => feat::copy_clash_env(app_handle),
                "open_logs_dir" => crate::log_err!(cmds::open_logs_dir()),
                "open_app_dir" => crate::log_err!(cmds::open_app_dir()),
                "open_core_dir" => crate::log_err!(cmds::open_core_dir()),
                "restart_clash" => feat::restart_clash_core(),
                "restart_app" => api::process::restart(&app_handle.env()),
                "quit" => cmds::exit_app(app_handle.clone()),

                _ => {}
            },
            _ => {}
        }
    }
}
