#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    #[serde(skip)]
    file: String,

    pub name: String,
    pub author: String,

    pub background: String,
    pub border: String,
    pub foreground: String,

    pub variant: ThemeVariant,
}

static THEME_INFOS: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/themes");
static THEME_CONFS: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/themes");

pub static THEMES: std::sync::LazyLock<Vec<Theme>> = std::sync::LazyLock::new(|| {
    let mut themes = Vec::new();

    for file in THEME_INFOS.files() {
        if let Some(ext) = file.path().extension()
            && ext == "json"
            && let Some(content) = file.contents_utf8()
            && let Ok(mut theme) = (serde_json::from_str::<Theme>(content).inspect_err(|e| {
                aviutl2::lprintln!(error, "{e}");
            }))
        {
            theme.file = file
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            themes.push(aviutl2::ldbg!(theme));
        }
    }

    themes
});
impl Theme {
    pub fn load(&self) -> String {
        let file = THEME_CONFS
            .get_file(format!("{}.style.conf", self.file))
            .expect("Failed to get theme config file");
        let content = file
            .contents_utf8()
            .expect("Failed to read theme config file as UTF-8");
        content.to_string()
    }
}
