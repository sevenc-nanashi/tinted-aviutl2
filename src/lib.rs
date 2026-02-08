mod gui;
mod theme;

pub static EDIT_HANDLE: std::sync::OnceLock<aviutl2::generic::EditHandle> = std::sync::OnceLock::new();

#[aviutl2::plugin(GenericPlugin)]
struct TintedAviutl2 {
    window: aviutl2_eframe::EframeWindow,
}

impl aviutl2::generic::GenericPlugin for TintedAviutl2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        let window = aviutl2_eframe::EframeWindow::new("TintedAviutl2", move |cc, handle| {
            Ok(Box::new(gui::TintedAviutl2App::new(cc, handle)))
        })?;
        Ok(Self { window })
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {
        registry
            .register_window_client("Tinted AviUtl2", &self.window)
            .unwrap();
        EDIT_HANDLE
            .set(registry.create_edit_handle())
            .unwrap();
    }
}

aviutl2::register_generic_plugin!(TintedAviutl2);
