// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

mod counter;
mod error;
pub mod hotkey;
mod platform_impl;

pub use self::error::*;
use hotkey::HotKey;

/// Contains the id of the triggered [`HotKey`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalHotKeyEvent(pub u32);

impl GlobalHotKeyEvent {
    /// Returns the id contained in this event
    pub fn id(&self) -> u32 {
        self.0
    }
}

/// A reciever that could be used to listen to tray events.
pub type GlobalHotKeyEventReceiver = Receiver<GlobalHotKeyEvent>;

static GLOBAL_HOTKEY_CHANNEL: Lazy<(Sender<GlobalHotKeyEvent>, GlobalHotKeyEventReceiver)> =
    Lazy::new(unbounded);

/// Gets a reference to the event channel's [TrayEventReceiver]
/// which can be used to listen for tray events.
pub fn global_hotkey_event_receiver<'a>() -> &'a GlobalHotKeyEventReceiver {
    &GLOBAL_HOTKEY_CHANNEL.1
}

pub struct GlobalHotKeyManager {
    platform_impl: platform_impl::GlobalHotKeyManager,
}

impl GlobalHotKeyManager {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            platform_impl: platform_impl::GlobalHotKeyManager::new()?,
        })
    }

    pub fn register(&self, hotkey: HotKey) -> crate::Result<()> {
        self.platform_impl.register(hotkey)
    }

    pub fn unregister(&self, hotkey: HotKey) -> crate::Result<()> {
        self.platform_impl.unregister(hotkey)
    }

    pub fn register_all(&self, hotkeys: &[HotKey]) -> crate::Result<()> {
        for hotkey in hotkeys {
            self.register(*hotkey)?;
        }
        Ok(())
    }

    pub fn unregister_all(&self, hotkeys: &[HotKey]) -> crate::Result<()> {
        for hotkey in hotkeys {
            self.register(*hotkey)?;
        }
        Ok(())
    }
}
