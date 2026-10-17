//! Windows media controls are owned by the UI lifetime; callbacks send service requests.
#[cfg(windows)]
mod windows_session {
    use crate::{protocol::Request, AppState, OrcaWindow};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use slint::{ComponentHandle, Timer, TimerMode};
    use std::{
        ffi::OsString,
        os::windows::ffi::{OsStrExt, OsStringExt},
        path::Path,
        sync::mpsc::Sender,
        time::Duration,
    };
    use windows::{
        core::{w, Interface, GUID, HSTRING, PCWSTR},
        Foundation::TypedEventHandler,
        Media::*,
        Storage::{StorageFile, Streams::RandomAccessStreamReference},
        Win32::{
            Foundation::{HWND, PROPERTYKEY},
            System::{
                Com::{
                    CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile,
                    StructuredStorage::PROPVARIANT, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
                    STGM_READWRITE,
                },
                Registry::*,
                WinRT::{
                    ISystemMediaTransportControlsInterop, RoInitialize, RoUninitialize,
                    RO_INIT_SINGLETHREADED,
                },
            },
            UI::Shell::{
                IShellLinkW, PropertiesSystem::IPropertyStore,
                SetCurrentProcessExplicitAppUserModelID,
            },
        },
    };

    const APP_ID: &str = "Orca.MusicPlayer";
    const APP_ID_KEY: PROPERTYKEY = PROPERTYKEY {
        fmtid: GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3),
        pid: 5,
    };
    pub(crate) fn identity() -> Result<(), Box<dyn std::error::Error>> {
        // The process and Start Menu shortcut must share one AUMID for taskbar grouping.
        unsafe {
            SetCurrentProcessExplicitAppUserModelID(&HSTRING::from(APP_ID))?;
        }
        let exe = std::env::current_exe()?;
        let icon = shell_icon_path()?;
        for (name, value) in [
            (w!("DisplayName"), OsString::from("Orca")),
            (w!("IconUri"), icon.into_os_string()),
        ] {
            let bytes: Vec<u16> = value.encode_wide().chain(Some(0)).collect();
            let status = unsafe {
                RegSetKeyValueW(
                    HKEY_CURRENT_USER,
                    w!("Software\\Classes\\AppUserModelId\\Orca.MusicPlayer"),
                    name,
                    REG_EXPAND_SZ.0,
                    Some(bytes.as_ptr().cast()),
                    (bytes.len() * 2) as u32,
                )
            };
            status.ok()?;
        }
        register_start_menu_shortcut(&exe)?;
        Ok(())
    }

    fn shell_icon_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let local = std::env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is missing")?;
        let directory = Path::new(&local).join("Orca");
        std::fs::create_dir_all(&directory)?;
        let icon = directory.join("shell-icon.png");
        let bytes = include_bytes!("../../ui/assets/orca-logo.png");
        if std::fs::read(&icon).ok().as_deref() != Some(bytes.as_slice()) {
            std::fs::write(&icon, bytes)?;
        }
        Ok(icon)
    }

    struct ComApartment;
    impl Drop for ComApartment {
        fn drop(&mut self) {
            unsafe { CoUninitialize() }
        }
    }

    fn register_start_menu_shortcut(exe: &Path) -> windows::core::Result<()> {
        let Some(roaming) = std::env::var_os("APPDATA") else {
            return Ok(());
        };
        let shortcut = Path::new(&roaming)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Orca.lnk");
        if !shortcut.is_file() {
            return Ok(());
        }
        set_shortcut_app_id(&shortcut, exe)
    }

    fn set_shortcut_app_id(shortcut: &Path, exe: &Path) -> windows::core::Result<()> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? };
        let _apartment = ComApartment;
        let clsid = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
        let link: IShellLinkW = unsafe { CoCreateInstance(&clsid, None, CLSCTX_INPROC_SERVER)? };
        let persist: IPersistFile = link.cast()?;
        let shortcut_wide: Vec<u16> = shortcut.as_os_str().encode_wide().chain(Some(0)).collect();
        unsafe { persist.Load(PCWSTR(shortcut_wide.as_ptr()), STGM_READWRITE)? };
        let mut target = [0u16; 32768];
        unsafe { link.GetPath(&mut target, std::ptr::null_mut(), 0)? };
        let end = target
            .iter()
            .position(|unit| *unit == 0)
            .unwrap_or(target.len());
        let target = std::path::PathBuf::from(OsString::from_wide(&target[..end]));
        // Portable builds must never claim an installed shortcut for another executable.
        let Ok(target) = std::fs::canonicalize(&target) else {
            return Ok(());
        };
        let Ok(exe) = std::fs::canonicalize(exe) else {
            return Ok(());
        };
        if target != exe {
            return Ok(());
        }
        let properties: IPropertyStore = link.cast()?;
        if unsafe { properties.GetValue(&APP_ID_KEY) }
            .ok()
            .and_then(|current| windows::core::BSTR::try_from(&current).ok())
            .is_some_and(|current| current == APP_ID)
        {
            return Ok(());
        }
        let app_id = PROPVARIANT::from(APP_ID);
        unsafe {
            properties.SetValue(&APP_ID_KEY, &app_id)?;
            properties.Commit()?;
            persist.Save(PCWSTR(shortcut_wide.as_ptr()), true)?;
        }
        Ok(())
    }
    struct Apartment;
    impl Drop for Apartment {
        fn drop(&mut self) {
            unsafe {
                RoUninitialize();
            }
        }
    }
    pub(crate) struct Session {
        controls: SystemMediaTransportControls,
        token: i64,
        timer: Timer,
        _apartment: Apartment,
    }
    impl Session {
        pub(crate) fn diagnose(&self) -> windows::core::Result<()> {
            let updater = self.controls.DisplayUpdater()?;
            updater.SetType(MediaPlaybackType::Music)?;
            updater.SetAppMediaId(&HSTRING::from(APP_ID))?;
            updater
                .MusicProperties()?
                .SetTitle(&HSTRING::from("Orca media controls check"))?;
            updater.Update()?;
            self.controls.SetIsEnabled(true)?;
            self.controls
                .SetPlaybackStatus(MediaPlaybackStatus::Playing)?;
            eprintln!("Windows media controls: metadata and playback published successfully");
            Ok(())
        }
        pub(crate) fn new(
            ui: &OrcaWindow,
            tx: Sender<Request>,
            track: std::rc::Rc<std::cell::RefCell<orca_services::types::Track>>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            unsafe {
                RoInitialize(RO_INIT_SINGLETHREADED)?;
            }
            let apartment = Apartment;
            let provider = ui.window().window_handle();
            let RawWindowHandle::Win32(handle) = provider.window_handle()?.as_raw() else {
                return Err("Windows media controls require a native window".into());
            };
            let interop: ISystemMediaTransportControlsInterop = windows::core::factory::<
                SystemMediaTransportControls,
                ISystemMediaTransportControlsInterop,
            >()?;
            let controls: SystemMediaTransportControls =
                unsafe { interop.GetForWindow(HWND(handle.hwnd.get() as *mut _))? };
            controls.SetIsEnabled(true)?;
            controls.SetIsPlayEnabled(true)?;
            controls.SetIsPauseEnabled(true)?;
            controls.SetIsNextEnabled(true)?;
            controls.SetIsPreviousEnabled(true)?;
            let token = controls.ButtonPressed(&TypedEventHandler::new(
                move |_,
                      args: windows::core::Ref<
                    '_,
                    SystemMediaTransportControlsButtonPressedEventArgs,
                >| {
                    if let Some(args) = args.as_ref() {
                        let action = match args.Button()? {
                            SystemMediaTransportControlsButton::Play => Some("resume"),
                            SystemMediaTransportControlsButton::Pause => Some("pause"),
                            SystemMediaTransportControlsButton::Next => Some("next"),
                            SystemMediaTransportControlsButton::Previous => Some("previous"),
                            _ => None,
                        };
                        if let Some(action) = action {
                            if let Ok(action) = crate::protocol::PlayerAction::parse(action) {
                                let _ = tx.send(Request::Command(action));
                            }
                        }
                    }
                    Ok(())
                },
            ))?;
            let timer = Timer::default();
            let weak = ui.as_weak();
            let polling = controls.clone();
            let mut last: Option<(String, String, String, String, String, bool)> = None;
            timer.start(TimerMode::Repeated, Duration::from_millis(250), move || {
                let Some(ui) = weak.upgrade() else {
                    return;
                };
                let s = ui.global::<AppState>();
                let song = track.borrow();
                let cover = [&song.artwork, &song.artwork_original, &song.artwork_thumb]
                    .into_iter()
                    .find(|path| !path.is_empty())
                    .map(|path| path.as_str())
                    .unwrap_or("");
                let current = (
                    song.path.clone(),
                    song.title.clone(),
                    song.artist.clone(),
                    song.album.clone(),
                    cover.to_string(),
                    s.get_playing(),
                );
                if last.as_ref() == Some(&current) {
                    return;
                }
                let metadata_changed = last.as_ref().is_none_or(|previous| {
                    previous.0 != current.0
                        || previous.1 != current.1
                        || previous.2 != current.2
                        || previous.3 != current.3
                        || previous.4 != current.4
                });
                let result = (|| -> windows::core::Result<()> {
                    polling.SetIsEnabled(!song.path.is_empty())?;
                    polling.SetPlaybackStatus(if song.path.is_empty() {
                        MediaPlaybackStatus::Stopped
                    } else if s.get_playing() {
                        MediaPlaybackStatus::Playing
                    } else {
                        MediaPlaybackStatus::Paused
                    })?;
                    if !metadata_changed {
                        return Ok(());
                    }
                    let updater = polling.DisplayUpdater()?;
                    updater.ClearAll()?;
                    updater.SetAppMediaId(&HSTRING::from(APP_ID))?;
                    updater.SetType(MediaPlaybackType::Music)?;
                    let properties = updater.MusicProperties()?;
                    properties.SetTitle(&HSTRING::from(song.title.as_str()))?;
                    properties.SetArtist(&HSTRING::from(song.artist.as_str()))?;
                    properties.SetAlbumTitle(&HSTRING::from(song.album.as_str()))?;
                    if let Some(path) = cover_path(cover) {
                        if let Ok(file) = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))
                            .and_then(|load| load.get())
                        {
                            if let Ok(stream) = RandomAccessStreamReference::CreateFromFile(&file) {
                                updater.SetThumbnail(&stream)?;
                            }
                        }
                    }
                    updater.Update()
                })();
                if let Err(error) = &result {
                    eprintln!("Media controls update: {error}");
                }
                if result.is_ok() {
                    last = Some(current);
                }
            });
            Ok(Self {
                controls,
                token,
                timer,
                _apartment: apartment,
            })
        }
    }
    fn cover_path(path: &str) -> Option<String> {
        if path.is_empty() {
            return None;
        }
        let path = path.replace('/', "\\");
        Some(if let Some(unc) = path.strip_prefix(r"\\?\UNC\") {
            format!(r"\\{unc}")
        } else {
            path.strip_prefix(r"\\?\").unwrap_or(&path).to_string()
        })
    }
    impl Drop for Session {
        fn drop(&mut self) {
            self.timer.stop();
            let _ = self.controls.RemoveButtonPressed(self.token);
            let _ = self.controls.SetIsEnabled(false);
        }
    }
    #[cfg(test)]
    mod tests {
        use super::{cover_path, set_shortcut_app_id, ComApartment, APP_ID, APP_ID_KEY};
        use std::os::windows::ffi::OsStrExt;
        use windows::{
            core::{Interface, GUID, PCWSTR},
            Win32::{
                System::Com::{
                    CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER,
                    COINIT_APARTMENTTHREADED, STGM_READWRITE,
                },
                UI::Shell::{IShellLinkW, PropertiesSystem::IPropertyStore},
            },
        };

        #[test]
        fn cover_paths_preserve_unicode_and_unc_paths() {
            let name = format!(r"C:\Music\a #{}.png", '\u{e9}');
            assert_eq!(cover_path(&format!(r"\\?\{name}")).unwrap(), name);
            assert_eq!(
                cover_path(r"\\?\UNC\server\music\a.png").unwrap(),
                r"\\server\music\a.png"
            );
            assert!(cover_path("").is_none());
        }

        #[test]
        fn installed_shortcut_receives_the_process_app_id() {
            let directory = std::env::temp_dir().join(format!(
                "orca-shortcut-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir(&directory).unwrap();
            let shortcut = directory.join("Orca.lnk");
            let exe = std::env::current_exe().unwrap();
            let shortcut_wide: Vec<u16> =
                shortcut.as_os_str().encode_wide().chain(Some(0)).collect();
            let exe_wide: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
            unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().unwrap() };
            let _apartment = ComApartment;
            let clsid = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
            let link: IShellLinkW =
                unsafe { CoCreateInstance(&clsid, None, CLSCTX_INPROC_SERVER).unwrap() };
            let persist: IPersistFile = link.cast().unwrap();
            unsafe {
                link.SetPath(PCWSTR(exe_wide.as_ptr())).unwrap();
                persist.Save(PCWSTR(shortcut_wide.as_ptr()), true).unwrap();
            }
            drop(persist);
            drop(link);

            set_shortcut_app_id(&shortcut, &exe).unwrap();
            let link: IShellLinkW =
                unsafe { CoCreateInstance(&clsid, None, CLSCTX_INPROC_SERVER).unwrap() };
            let persist: IPersistFile = link.cast().unwrap();
            unsafe {
                persist
                    .Load(PCWSTR(shortcut_wide.as_ptr()), STGM_READWRITE)
                    .unwrap();
            }
            let properties: IPropertyStore = link.cast().unwrap();
            let app_id = unsafe { properties.GetValue(&APP_ID_KEY).unwrap() };
            assert_eq!(windows::core::BSTR::try_from(&app_id).unwrap(), APP_ID);
            drop(properties);
            drop(persist);
            drop(link);
            std::fs::remove_dir_all(directory).unwrap();
        }
    }
}
#[cfg(windows)]
pub(crate) use windows_session::{identity, Session};
