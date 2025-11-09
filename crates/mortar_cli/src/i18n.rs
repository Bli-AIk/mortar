use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn from_env() -> Self {
        // 检查环境变量来确定系统语言
        if let Ok(lang) = std::env::var("LANG")
            && (lang.contains("zh") || lang.contains("cn") || lang.contains("CN"))
        {
            return Language::Chinese;
        }

        // 检查其他常见的语言环境变量
        for env_var in &["LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
            if let Ok(lang) = std::env::var(env_var)
                && (lang.contains("zh") || lang.contains("cn") || lang.contains("CN"))
            {
                return Language::Chinese;
            }
        }

        Language::English
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(Language::English),
            "zh" | "chinese" | "zh-cn" | "zh_cn" => Ok(Language::Chinese),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}

static TEXTS: OnceLock<HashMap<&'static str, HashMap<Language, &'static str>>> = OnceLock::new();

fn init_texts() -> &'static HashMap<&'static str, HashMap<Language, &'static str>> {
    TEXTS.get_or_init(|| {
        let mut texts = HashMap::new();

        // CLI应用描述
        texts.insert(
            "app_about",
            [
                (Language::English, "Mortar language compiler"),
                (Language::Chinese, "Mortar 语言编译器"),
            ]
            .into(),
        );

        // 参数帮助文本
        texts.insert(
            "input_help",
            [
                (Language::English, "Input .mortar file"),
                (Language::Chinese, "输入 .mortar 文件"),
            ]
            .into(),
        );

        texts.insert(
            "output_help",
            [
                (Language::English, "Output file path"),
                (Language::Chinese, "输出文件路径"),
            ]
            .into(),
        );

        texts.insert(
            "pretty_help",
            [
                (
                    Language::English,
                    "Generate formatted JSON with indentation",
                ),
                (Language::Chinese, "生成带缩进的格式化 JSON"),
            ]
            .into(),
        );

        texts.insert(
            "verbose_lexer_help",
            [
                (Language::English, "Show verbose lexer output"),
                (Language::Chinese, "显示详细的词法分析输出"),
            ]
            .into(),
        );

        texts.insert(
            "show_source_help",
            [
                (Language::English, "Show original source text"),
                (Language::Chinese, "显示原始源代码文本"),
            ]
            .into(),
        );

        texts.insert(
            "check_only_help",
            [
                (
                    Language::English,
                    "Only check for errors and warnings without generating output",
                ),
                (Language::Chinese, "仅检查错误和警告，不生成输出文件"),
            ]
            .into(),
        );

        texts.insert(
            "language_help",
            [
                (Language::English, "Set display language (en, zh)"),
                (Language::Chinese, "设置显示语言 (en, zh)"),
            ]
            .into(),
        );

        // 运行时消息
        texts.insert(
            "error_reading_file",
            [
                (Language::English, "Error reading file:"),
                (Language::Chinese, "读取文件错误:"),
            ]
            .into(),
        );

        texts.insert(
            "original_source",
            [
                (Language::English, "--- Original Source ---"),
                (Language::Chinese, "--- 原始源代码 ---"),
            ]
            .into(),
        );

        texts.insert(
            "end_source",
            [
                (Language::English, "--- End Source ---"),
                (Language::Chinese, "--- 源代码结束 ---"),
            ]
            .into(),
        );

        texts.insert(
            "compilation_failed",
            [
                (Language::English, "Compilation failed due to errors."),
                (Language::Chinese, "编译因错误而失败。"),
            ]
            .into(),
        );

        texts.insert(
            "parsed_successfully",
            [
                (Language::English, "Parsed successfully!"),
                (Language::Chinese, "解析成功！"),
            ]
            .into(),
        );

        texts.insert(
            "generated_successfully",
            [
                (Language::English, "Successfully generated .mortared file"),
                (Language::Chinese, "成功生成 .mortared 文件"),
            ]
            .into(),
        );

        texts.insert(
            "failed_to_generate",
            [
                (Language::English, "Failed to generate .mortared file:"),
                (Language::Chinese, "生成 .mortared 文件失败:"),
            ]
            .into(),
        );

        texts.insert(
            "unsupported_language_error",
            [
                (
                    Language::English,
                    "Unsupported language. Supported languages: en (English), zh (Chinese)",
                ),
                (
                    Language::Chinese,
                    "不支持的语言。支持的语言: en (英语), zh (中文)",
                ),
            ]
            .into(),
        );

        // 编译器输出文本
        texts.insert(
            "checking_file",
            [
                (Language::English, "Checking file:"),
                (Language::Chinese, "检查文件:"),
            ]
            .into(),
        );

        texts.insert(
            "generated",
            [
                (Language::English, "Generated:"),
                (Language::Chinese, "生成文件:"),
            ]
            .into(),
        );

        texts.insert(
            "error_severity",
            [(Language::English, "error"), (Language::Chinese, "错误")].into(),
        );

        texts.insert(
            "warning_severity",
            [(Language::English, "warning"), (Language::Chinese, "警告")].into(),
        );

        texts
    })
}

pub fn get_text(key: &str, language: Language) -> &'static str {
    let texts = init_texts();
    texts
        .get(key)
        .and_then(|map| map.get(&language))
        .unwrap_or_else(|| {
            // 如果找不到对应语言的文本，回退到英文
            texts
                .get(key)
                .and_then(|map| map.get(&Language::English))
                .unwrap_or(&"")
        })
}
