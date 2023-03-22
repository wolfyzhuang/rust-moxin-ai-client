
use chrono::Utc;
use moxin_protocol::data::{Author, File, Model};

pub fn get_models() -> Vec<Model> {
    let open_hermes_files = vec![
        File {
            id: "1".to_string(),
            name: "openhermes-2.5-mistral-7b.Q2_K.gguf".to_string(),
            size: "3.08 GB".to_string(),
            quantization: "Q2_K".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec![],
            featured: false,
        },
        File {
            id: "2".to_string(),
            name: "openhermes-2.5-mistral-7b.Q3_K_S.gguf".to_string(),
            size: "3.16 GB".to_string(),
            quantization: "Q3_K_S".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec![],
            featured: false,
        },
        File {
            id: "3".to_string(),
            name: "openhermes-2.5-mistral-7b.Q3_K_M.gguf".to_string(),
            size: "3.52 GB".to_string(),
            quantization: "Q3_K_M".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec![],
            featured: false,
        },
        File {
            id: "4".to_string(),
            name: "openhermes-2.5-mistral-7b.Q3_K_L.gguf".to_string(),
            size: "3.82 GB".to_string(),
            quantization: "Q3_K_M".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec![],
            featured: false,
        },
        File {
            id: "5".to_string(),
            name: "openhermes-2.5-mistral-7b.Q4_0.gguf".to_string(),
            size: "4.11 GB".to_string(),
            quantization: "Q4_0".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec![],
            featured: false,
        },
        File {
            id: "6".to_string(),
            name: "stablelm-zephyr-3b.Q4_K_S.gguf".to_string(),
            size: "1.62 GB".to_string(),
            quantization: "Q4_K_S".to_string(),
            downloaded: true,
            downloaded_path: Some("/home/user/.moxin/stablelm-zephyr-3b.Q4_K_S.gguf".to_string()),
            tags: vec!["Small & Fast".to_string()],
            featured: true,
        },
        File {
            id: "7".to_string(),
            name: "stablelm-zephyr-3b.Q6_K.gguf".to_string(),
            size: "2.30 GB".to_string(),
            quantization: "Q6_K".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec!["Less Compressed".to_string(), "Might be slower".to_string()],
            featured: true,
        },
    ];

    let nexus_raven_files = vec![
        File {
            id: "8".to_string(),
            name: "nexusraven-v2-13b.Q4_K_S.gguf".to_string(),
            size: "7.41 GB".to_string(),
            quantization: "Q4_K_S".to_string(),
            downloaded: false,
            downloaded_path: None,
            tags: vec!["Small & Fast".to_string()],
            featured: true,
        },
        File {
            id: "9".to_string(),
            name: "nexusraven-v2-13b.Q6_K.gguf".to_string(),