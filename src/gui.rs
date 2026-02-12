use aviutl2_eframe::{AviUtl2EframeHandle, eframe, egui};
use std::io::Write;

fn tr(text: &str) -> String {
    aviutl2::config::translate(text).unwrap_or_else(|_| text.to_string())
}

fn color_from_theme(hex: &str, fallback: egui::Color32) -> egui::Color32 {
    let trimmed = hex.trim();
    if trimmed.is_empty() {
        return fallback;
    }
    let prefixed;
    let value = if trimmed.starts_with('#') {
        trimmed
    } else {
        prefixed = format!("#{trimmed}");
        prefixed.as_str()
    };
    egui::Color32::from_hex(value).unwrap_or(fallback)
}

pub(crate) struct TintedAviutl2App {
    show_info: bool,
    suppress_info_close_once: bool,
    search_query: String,
    version: String,
    handle: AviUtl2EframeHandle,
}

impl TintedAviutl2App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>, handle: AviUtl2EframeHandle) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "M+ 1p".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "./fonts/mplus-1p-regular.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .expect("Failed to get Proportional font family")
            .insert(0, "M+ 1p".to_owned());

        cc.egui_ctx.all_styles_mut(|style| {
            style.visuals = aviutl2_eframe::aviutl2_visuals();
        });
        cc.egui_ctx.set_fonts(fonts);

        Self {
            show_info: false,
            suppress_info_close_once: false,
            search_query: String::new(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            handle,
        }
    }
}

impl eframe::App for TintedAviutl2App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_toolbar(ctx);
        self.render_main_panel(ctx);
        self.render_info_window(ctx);
    }
}

impl TintedAviutl2App {
    fn render_toolbar(&mut self, ctx: &egui::Context) {
        // TODO: toolbarの右クリックイベントに右クリックメニューを割り当てる
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let clicked = ui
                    .heading("Tinted AviUtl2 Themes")
                    .interact(egui::Sense::click());
                if clicked.secondary_clicked() {
                    let _ = self.handle.show_context_menu();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let resp = ui
                        .add_sized(
                            egui::vec2(
                                ui.text_style_height(&egui::TextStyle::Heading),
                                ui.text_style_height(&egui::TextStyle::Heading),
                            ),
                            egui::Button::new("i"),
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .on_hover_text(tr("プラグイン情報"));
                    if resp.clicked() {
                        self.show_info = true;
                        self.suppress_info_close_once = true;
                    }
                });
            });
        });
    }

    fn render_main_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("検索"));
                ui.add_sized(
                    egui::vec2(ui.available_width(), 24.0),
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text(tr("テーマ名または作者名で検索")),
                );
            });
            ui.add_space(8.0);

            let query = self.search_query.trim().to_lowercase();
            let mut has_results = false;
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for theme in crate::theme::THEMES.iter() {
                        if !query.is_empty()
                            && !theme.name.to_lowercase().contains(&query)
                            && !theme.author.to_lowercase().contains(&query)
                        {
                            continue;
                        }
                        has_results = true;
                        self.render_theme_card(ui, theme);
                        ui.add_space(8.0);
                    }

                    if !has_results {
                        ui.label(tr("一致するテーマがありません"));
                    }
                });
        });
    }

    fn render_info_window(&mut self, ctx: &egui::Context) {
        if !self.show_info {
            return;
        }
        let screen_rect = ctx.content_rect();
        let dim_color = egui::Color32::from_black_alpha(128);
        let dim_response = egui::Area::new(egui::Id::new("info_window_dim_layer"))
            .order(egui::Order::Middle)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(screen_rect.size());
                let (rect, response) =
                    ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
                ui.painter().rect_filled(rect, 0.0, dim_color);
                response
            })
            .inner;
        let mut open = true;
        let response = egui::Window::new("Tinted AviUtl2")
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let version_label = tr("バージョン: {version}");
                ui.label(version_label.replace("{version}", &self.version));
                ui.label(tr("Tinted ThemesのAviUtl2移植版。"));
                ui.add_space(8.0);
                ui.label(tr("開発者:"));
                ui.hyperlink_to("Nanashi.", "https://sevenc7c.com");
                ui.add_space(4.0);
                ui.label(tr("ソースコード:"));
                ui.hyperlink_to(
                    "sevenc-nanashi/aviutl2-rs",
                    "https://github.com/sevenc-nanashi/tinted-aviutl2",
                );
            });
        if self.suppress_info_close_once {
            self.suppress_info_close_once = false;
        } else if dim_response.clicked() {
            self.show_info = false;
        } else if let Some(response) = response
            && response.response.clicked_elsewhere()
        {
            self.show_info = false;
        }
        if !open {
            self.show_info = false;
        }
    }

    fn render_theme_card(&self, ui: &mut egui::Ui, theme: &crate::theme::Theme) {
        let available_width = ui.available_width();
        let response = ui.scope(|ui| {
            let background = color_from_theme(&theme.background, ui.visuals().faint_bg_color);
            let border = color_from_theme(
                &theme.border,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            );
            let foreground = color_from_theme(&theme.foreground, ui.visuals().text_color());

            let frame = egui::Frame::group(ui.style())
                .fill(background)
                .stroke(egui::Stroke {
                    color: border,
                    ..ui.visuals().widgets.noninteractive.bg_stroke
                })
                .inner_margin(egui::Margin::symmetric(8, 4));
            ui.allocate_ui_with_layout(
                egui::vec2(available_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    frame
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_min_height(24.0);
                                    ui.vertical(|ui| {
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(&theme.name)
                                                    .color(foreground)
                                                    .text_style(egui::TextStyle::Body),
                                            )
                                            .selectable(false),
                                        );
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(&theme.author)
                                                    .color(foreground)
                                                    .text_style(egui::TextStyle::Small),
                                            )
                                            .selectable(false),
                                        );
                                    });
                                },
                            );
                        })
                        .response
                },
            )
            .inner
        });
        let response = response
            .response
            .interact(egui::Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        if response.clicked()
            && native_dialog::DialogBuilder::message()
                .set_title(tr("Tinted AviUtl2"))
                .set_text(tr("このテーマを適用しますか？"))
                .confirm()
                .show()
                .unwrap_or(false)
        {
            self.install_style(theme).unwrap_or_else(|err| {
                let _ = native_dialog::DialogBuilder::message()
                    .set_title(tr("エラー"))
                    .set_text(format!("スタイルのインストールに失敗しました: {err}"))
                    .set_owner(&unsafe { crate::EDIT_HANDLE.get_host_app_window() }.unwrap())
                    .alert()
                    .show();
            });
        }
    }

    fn install_style(&self, theme: &crate::theme::Theme) -> aviutl2::AnyResult<()> {
        let data_dir = aviutl2::config::app_data_path();
        let style_conf_path = data_dir.join("style.conf");
        let existing_style_conf = if style_conf_path.try_exists()? {
            std::fs::read_to_string(&style_conf_path)?
        } else {
            String::new()
        };
        let mut style_conf = std::fs::File::create(&style_conf_path)?;
        let merged_style = crate::merge_style::merge_style(&existing_style_conf, &theme.load());
        style_conf.write_all(merged_style.as_bytes())?;

        if native_dialog::DialogBuilder::message()
            .set_title(tr("Tinted AviUtl2"))
            .set_text(tr("テーマを適用しました。AviUtl2を再起動しますか？"))
            .set_owner(&unsafe { crate::EDIT_HANDLE.get_host_app_window() }.unwrap())
            .confirm()
            .show()
            .unwrap_or(false)
        {
            crate::EDIT_HANDLE.restart_host_app();
        }

        Ok(())
    }
}
