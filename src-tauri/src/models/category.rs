use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub target_folder: PathBuf,
    pub rules: Vec<CategoryRule>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRule {
    pub rule_type: RuleType,
    pub pattern: String,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Extension,
    NameContains,
    NameRegex,
    MimeType,
    LlmKeyword,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            target_folder: PathBuf::new(),
            rules: Vec::new(),
            icon: None,
            color: None,
        }
    }
}

pub fn default_categories() -> Vec<Category> {
    vec![
        Category {
            id: "documents".to_string(),
            name: "文档".to_string(),
            description: Some("PDF、Word、Excel 等文档文件".to_string()),
            target_folder: PathBuf::from("Documents"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "pdf,doc,docx,xls,xlsx,ppt,pptx,txt,rtf,odt".to_string(),
                priority: 1,
            }],
            icon: Some("file-text".to_string()),
            color: Some("#3B82F6".to_string()),
        },
        Category {
            id: "images".to_string(),
            name: "图片".to_string(),
            description: Some("JPG、PNG、GIF 等图片文件".to_string()),
            target_folder: PathBuf::from("Images"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "jpg,jpeg,png,gif,webp,svg,bmp,ico,tiff,heic".to_string(),
                priority: 1,
            }],
            icon: Some("image".to_string()),
            color: Some("#10B981".to_string()),
        },
        Category {
            id: "videos".to_string(),
            name: "视频".to_string(),
            description: Some("MP4、MKV、AVI 等视频文件".to_string()),
            target_folder: PathBuf::from("Videos"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "mp4,mkv,avi,mov,wmv,flv,webm,m4v".to_string(),
                priority: 1,
            }],
            icon: Some("video".to_string()),
            color: Some("#8B5CF6".to_string()),
        },
        Category {
            id: "music".to_string(),
            name: "音乐".to_string(),
            description: Some("MP3、FLAC、WAV 等音频文件".to_string()),
            target_folder: PathBuf::from("Music"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "mp3,flac,wav,aac,ogg,wma,m4a".to_string(),
                priority: 1,
            }],
            icon: Some("music".to_string()),
            color: Some("#F59E0B".to_string()),
        },
        Category {
            id: "code".to_string(),
            name: "代码".to_string(),
            description: Some("JS、Python、Rust 等代码文件".to_string()),
            target_folder: PathBuf::from("Code"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "js,ts,jsx,tsx,py,rs,go,java,c,cpp,h,hpp,cs,rb,php".to_string(),
                priority: 1,
            }],
            icon: Some("code".to_string()),
            color: Some("#EC4899".to_string()),
        },
        Category {
            id: "archives".to_string(),
            name: "压缩包".to_string(),
            description: Some("ZIP、RAR、7z 等压缩文件".to_string()),
            target_folder: PathBuf::from("Archives"),
            rules: vec![CategoryRule {
                rule_type: RuleType::Extension,
                pattern: "zip,rar,7z,tar,gz,bz2,xz".to_string(),
                priority: 1,
            }],
            icon: Some("archive".to_string()),
            color: Some("#6366F1".to_string()),
        },
        Category {
            id: "others".to_string(),
            name: "其他".to_string(),
            description: Some("其他类型文件".to_string()),
            target_folder: PathBuf::from("Others"),
            rules: vec![],
            icon: Some("file".to_string()),
            color: Some("#71717A".to_string()),
        },
    ]
}
