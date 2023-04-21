use serde::Deserialize;

const API_XML: &str = include_str!("../../xml2-src/vendor/libxml2/doc/libxml2-api.xml");

#[derive(Debug, Deserialize)]
pub(crate) struct Api {
    #[serde(rename = "@name")]
    pub name: String,
    pub files: Files,
    pub symbols: Symbols,
}

impl Api {
    pub fn load() -> Self {
        quick_xml::de::from_str(API_XML).expect("api xml")
    }
}

#[derive(Debug, Deserialize)]
pub struct Files {
    #[serde(rename = "file")]
    pub items: Vec<File>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(rename = "@name")]
    pub name: String,
    pub summary: String,
    pub description: String,
    pub author: Option<String>,
    pub deprecated: Option<()>,
    pub exports: Vec<Export>,
}

#[derive(Debug, Deserialize)]
pub struct Export {
    #[serde(rename = "@symbol")]
    pub symbol: String,
    #[serde(rename = "@type")]
    pub r#type: ExportType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    Macro,
    Enum,
    Typedef,
    Struct,
    Function,
    Variable,
}

#[derive(Debug, Deserialize)]
pub struct Symbols {
    #[serde(rename = "$value")]
    pub items: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Symbol {
    Macro(Macro),
    Enum(Enum),
    Typedef(Typedef),
    Struct(Struct),
    Function(Function),
    Variable(Variable),
    FuncType(Function),
}

#[derive(Debug, Deserialize)]
pub struct Macro {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    pub info: Option<String>,
    #[serde(default, rename = "arg")]
    pub args: Vec<MacroArg>,
}

#[derive(Debug, Deserialize)]
pub struct MacroArg {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@info")]
    pub info: String,
}

#[derive(Debug, Deserialize)]
pub struct Enum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@info")]
    pub info: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Typedef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@type")]
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Struct {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(default, rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@info")]
    pub info: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@module")]
    pub module: String,
    pub cond: Option<String>,
    pub info: String,
    pub r#return: FunctionReturn,
    #[serde(default, rename = "arg")]
    pub args: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionReturn {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@info")]
    pub info: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@type")]
    pub r#type: String,
}
