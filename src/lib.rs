mod consts;
mod exports;
mod global;
mod helpers;
mod loader;
mod models;
mod types;

use std::fs::File;
use std::process::exit;

use anyhow::Result;
use ctor::ctor;
use global::CONFIG;
use loader::init_hooks;
use log::LevelFilter;
use native_dialog::{MessageDialog, MessageType};
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};

#[inline]
fn setup_logger() -> Result<()> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Debug)
            .set_time_format_str("%F %T%.3f")
            .build(),
        File::create(&CONFIG.uplay.log.path)?,
    )])?;

    Ok(())
}

#[ctor]
fn init() {
    check_config();

    if CONFIG.uplay.log.write {
        log_panics::init();

        if setup_logger().is_err() {
            exit(1);
        }
    }

    if CONFIG.uplay.install_hooks {
        unsafe {
            init_hooks();
        }
    }
}

#[inline]
fn fail_if(cond: bool, msg: &str) {
    if cond {
        let _ = MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error...")
            .set_text(msg)
            .show_alert();
        exit(1);
    }
}

#[inline]
fn check_config() {
    fail_if(CONFIG.uplay.saves.is_empty(), "Saves path is empty!");
    fail_if(CONFIG.uplay.profile.email.is_empty(), "Email is empty!");
    fail_if(
        CONFIG.uplay.profile.username.is_empty(),
        "Username is empty!",
    );
    fail_if(
        CONFIG.uplay.profile.password.is_empty(),
        "Password is empty!",
    );
    fail_if(
        CONFIG.uplay.profile.account_id.is_empty(),
        "Account id is empty!",
    );
    fail_if(
        CONFIG.uplay.log.write && CONFIG.uplay.log.path.is_empty(),
        "Log path is empty!",
    );
}

#[export_name = "SpaceCat"]
pub fn space_cat() {}
