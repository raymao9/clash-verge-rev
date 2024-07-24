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
        let tw = { Config::verge().latest().language == Some("tw".into()) };
        let cn = { Config::verge().latest().language == Some("cn".into()) };

        let version = app_handle.package_info().version.to_string();

        macro_rules! t {
            ($tw: expr, $cn: expr, $en: expr) => {
                if tw {
                    $tw
                } else {
		    if cn {
		    	$cn
		    } else {
                    	$en
                    }
            	}
	     };
        }

        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new(
                "open_window",
                t!("打開主視窗", "打开面板", "Dashboard"),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "rule_mode",
                t!("分流模式", "规则模式", "Rule Mode"),
            ))
            .add_item(CustomMenuItem::new(
                "global_mode",
                t!("全局模式", "全局模式", "Global Mode"),
            ))
            .add_item(CustomMenuItem::new(
                "direct_mode",
                t!("直連模式", "直连模式", "Direct Mode"),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new(
                "system_proxy",
                t!("啟用系統代理", "系统代理", "System Proxy"),
            ))
            .add_item(CustomMenuItem::new("tun_mode", t!("啟用 Tun 模式", "打开 Tun 模式", "TUN Mode")))
            .add_item(CustomMenuItem::new(
                "copy_env",
                t!("複製終端機代理指令", "复制环境变量", "Copy Env"),
            ))
            .add_submenu(SystemTraySubmenu::new(
                t!("打開檔案位置", "打开目录", "Open Dir"),
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new(
                        "open_app_dir",
                        t!("應用程式所在位置", "应用目录", "App Dir"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "open_core_dir",
                        t!("Clash 核心所在位置", "内核目录", "Core Dir"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "open_logs_dir",
                        t!("連線記錄所在位置", "日志目录", "Logs Dir"),
                    )),
            ))
            .add_submenu(SystemTraySubmenu::new(
                t!("更多功能", "更多", "More"),
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new(
                        "restart_clash",
                        t!("重啟 Clash", "重启 Clash", "Restart Clash"),
                    ))
                    .add_item(CustomMenuItem::new(
                        "restart_app",
                        t!("重啟 APP", "重启应用", "Restart App"),
                    ))
                    .add_item(
                        CustomMenuItem::new("app_version", format!("Version {version}")).disabled(),
                    ),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", t!("結束", "退出", "Quit")).accelerator("CmdOrControl+Q"))
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
                if tw {
                    $tw
                } else {
		   if cn {
			$cn
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

        #[cfg(target_os = "linux")]
        match mode.as_str() {
            "rule" => {
                let _ = tray
                    .get_item("rule_mode")
                    .set_title(t!("分流模式  ✔", "规则模式  ✔", "Rule Mode  ✔"));
                let _ = tray
                    .get_item("global_mode")
                    .set_title(t!("全局模式", "全局模式", "Global Mode"));
                let _ = tray
                    .get_item("direct_mode")
                    .set_title(t!("直連模式", "直连模式", "Direct Mode"));
            }
            "global" => {
                let _ = tray
                    .get_item("rule_mode")
                    .set_title(t!("分流模式", "规则模式", "Rule Mode"));
                let _ = tray
                    .get_item("global_mode")
                    .set_title(t!("全局模式  ✔", "全局模式  ✔", "Global Mode  ✔"));
                let _ = tray
                    .get_item("direct_mode")
                    .set_title(t!("直連模式", "直连模式", "Direct Mode"));
            }
            "direct" => {
                let _ = tray
                    .get_item("rule_mode")
                    .set_title(t!("分流模式", "规则模式", "Rule Mode"));
                let _ = tray
                    .get_item("global_mode")
                    .set_title(t!("全局模式", "全局模式", "Global Mode"));
                let _ = tray
                    .get_item("direct_mode")
                    .set_title(t!("直連模式  ✔", "直连模式  ✔", "Direct Mode  ✔"));
            }
            _ => {}
        }

        let verge = Config::verge();
        let verge = verge.latest();
        let system_proxy = verge.enable_system_proxy.as_ref().unwrap_or(&false);
        let tun_mode = verge.enable_tun_mode.as_ref().unwrap_or(&false);
        #[cfg(target_os = "macos")]
        let tray_icon = verge.tray_icon.clone().unwrap_or("monochrome".to_string());
        let common_tray_icon = verge.common_tray_icon.as_ref().unwrap_or(&false);
        let sysproxy_tray_icon = verge.sysproxy_tray_icon.as_ref().unwrap_or(&false);
        let tun_tray_icon = verge.tun_tray_icon.as_ref().unwrap_or(&false);
        #[cfg(target_os = "macos")]
        match tray_icon.as_str() {
            "monochrome" => {
                let _ = tray.set_icon_as_template(true);
            }
            "colorful" => {
                let _ = tray.set_icon_as_template(false);
            }
            _ => {}
        }
        let mut indication_icon = if *system_proxy {
            #[cfg(target_os = "macos")]
            let mut icon = match tray_icon.as_str() {
                "monochrome" => include_bytes!("../../icons/mac-tray-icon-sys.png").to_vec(),
                "colorful" => include_bytes!("../../icons/tray-icon-sys.ico").to_vec(),
                _ => include_bytes!("../../icons/mac-tray-icon-sys.ico").to_vec(),
            };
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/mac-tray-icon-sys.png").to_vec();

            if *sysproxy_tray_icon {
                let icon_dir_path = dirs::app_home_dir()?.join("icons");
                let png_path = icon_dir_path.join("sysproxy.png");
                let ico_path = icon_dir_path.join("sysproxy.ico");
                if ico_path.exists() {
                    icon = std::fs::read(ico_path).unwrap();
                } else if png_path.exists() {
                    icon = std::fs::read(png_path).unwrap();
                }
            }
            icon
        } else {
            #[cfg(target_os = "macos")]
            let mut icon = match tray_icon.as_str() {
                "monochrome" => include_bytes!("../../icons/mac-tray-icon.png").to_vec(),
                "colorful" => include_bytes!("../../icons/tray-icon.ico").to_vec(),
                _ => include_bytes!("../../icons/mac-tray-icon.png").to_vec(),
            };
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/tray-icon.png").to_vec();
            if *common_tray_icon {
                let icon_dir_path = dirs::app_home_dir()?.join("icons");
                let png_path = icon_dir_path.join("common.png");
                let ico_path = icon_dir_path.join("common.ico");
                if ico_path.exists() {
                    icon = std::fs::read(ico_path).unwrap();
                } else if png_path.exists() {
                    icon = std::fs::read(png_path).unwrap();
                }
            }
            icon
        };

        if *tun_mode {
            #[cfg(target_os = "macos")]
            let mut icon = match tray_icon.as_str() {
                "monochrome" => include_bytes!("../../icons/mac-tray-icon-tun.png").to_vec(),
                "colorful" => include_bytes!("../../icons/tray-icon-tun.ico").to_vec(),
                _ => include_bytes!("../../icons/mac-tray-icon-tun.ico").to_vec(),
            };
            #[cfg(not(target_os = "macos"))]
            let mut icon = include_bytes!("../../icons/tray-icon-tun.png").to_vec();
            if *tun_tray_icon {
                let icon_dir_path = dirs::app_home_dir()?.join("icons");
                let png_path = icon_dir_path.join("tun.png");
                let ico_path = icon_dir_path.join("tun.ico");
                if ico_path.exists() {
                    icon = std::fs::read(ico_path).unwrap();
                } else if png_path.exists() {
                    icon = std::fs::read(png_path).unwrap();
                }
            }
            indication_icon = icon
        }

        let _ = tray.set_icon(tauri::Icon::Raw(indication_icon));

        let _ = tray.get_item("system_proxy").set_selected(*system_proxy);
        let _ = tray.get_item("tun_mode").set_selected(*tun_mode);
        #[cfg(target_os = "linux")]
        {
            if *system_proxy {
                let _ = tray
                    .get_item("system_proxy")
                    .set_title(t!("系統代理  ✔", "系统代理  ✔", "System Proxy  ✔"));
            } else {
                let _ = tray
                    .get_item("system_proxy")
                    .set_title(t!("系統代理", "系统代理", "System Proxy"));
            }
            if *tun_mode {
                let _ = tray
                    .get_item("tun_mode")
                    .set_title(t!("Tun 模式  ✔", "Tun 模式  ✔", "TUN Mode  ✔"));
            } else {
                let _ = tray
                    .get_item("tun_mode")
                    .set_title(t!("Tun 模式", "Tun 模式", "TUN Mode"));
            }
        }

        let switch_map = {
            let mut map = std::collections::HashMap::new();
            map.insert(true, "on");
            map.insert(false, "off");
            map
        };

        let mut current_profile_name = "None".to_string();
        let profiles = Config::profiles();
        let profiles = profiles.latest();
        if let Some(current_profile_uid) = profiles.get_current() {
            let current_profile = profiles.get_item(&current_profile_uid);
            current_profile_name = match &current_profile.unwrap().name {
                Some(profile_name) => profile_name.to_string(),
                None => current_profile_name,
            };
        };
        let _ = tray.set_tooltip(&format!(
            "Clash Verge {version}\n{}: {}\n{}: {}\n{}: {}",
            t!("系統代理", "系统代理", "System Proxy"),
            switch_map[system_proxy],
            t!("Tun 模式", "Tun 模式", "TUN Mode"),
            switch_map[tun_mode],
            t!("現用訂閱", "当前订阅", "Curent Profile"),
            current_profile_name
        ));

        Ok(())
    }

    pub fn on_click(app_handle: &AppHandle) {
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
            #[cfg(not(target_os = "macos"))]
            SystemTrayEvent::LeftClick { .. } => Tray::on_click(app_handle),
            #[cfg(target_os = "macos")]
            SystemTrayEvent::RightClick { .. } => Tray::on_click(app_handle),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                mode @ ("rule_mode" | "global_mode" | "direct_mode") => {
                    let mode = &mode[0..mode.len() - 5];
                    feat::change_clash_mode(mode.into());
                }
                "open_window" => resolve::create_window(app_handle),
                "system_proxy" => feat::toggle_system_proxy(),
                "tun_mode" => feat::toggle_tun_mode(),
                "copy_env" => feat::copy_clash_env(app_handle),
                "open_app_dir" => crate::log_err!(cmds::open_app_dir()),
                "open_core_dir" => crate::log_err!(cmds::open_core_dir()),
                "open_logs_dir" => crate::log_err!(cmds::open_logs_dir()),
                "restart_clash" => feat::restart_clash_core(),
                "restart_app" => api::process::restart(&app_handle.env()),
                "quit" => cmds::exit_app(app_handle.clone()),

                _ => {}
            },
            _ => {}
        }
    }
}