#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Navigation {
    Chart,
    Table,
    Add,
    Settings,
    Goals,
    Import,
    Export,
}

impl Navigation {
    pub fn get_icon_classes(self) -> String {
        format!("fa-solid {}", self.get_icon_class())
    }

    pub fn get_icon_class(self) -> String {
        let class = match self {
            Navigation::Chart => "fa-chart-line",
            Navigation::Table => "fa-table",
            Navigation::Add => "fa-plus",
            Navigation::Settings => "fa-gear",
            Navigation::Goals => "fa-ruler-vertical",
            Navigation::Import => "fa-upload",
            Navigation::Export => "fa-download",
        };
        class.to_owned()
    }
}
