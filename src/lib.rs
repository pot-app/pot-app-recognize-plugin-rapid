use dirs::{cache_dir, config_dir};
use num_cpus::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn recognize(
    _base64: &str,
    _lang: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let config_dir_path = config_dir().unwrap();

    let plugin_path = config_dir_path
        .join("com.pot-app.desktop")
        .join("plugins")
        .join("recognize")
        .join("[plugin].com.pot-app.rapid");
    #[cfg(not(target_os = "windows"))]
    let rapid_exe_path = plugin_path.join("RapidOcrOnnx");
    #[cfg(target_os = "windows")]
    let rapid_exe_path = plugin_path.join("RapidOcrOnnx.exe");

    let cache_dir_path = cache_dir().unwrap();
    let image_path = cache_dir_path
        .join("com.pot-app.desktop")
        .join("pot_screenshot_cut.png");
    let thread_num = get();
    let output = std::process::Command::new(rapid_exe_path)
        .current_dir(plugin_path)
        .args([
            "--models",
            "models",
            "--det",
            "ch_PP-OCRv3_det_infer.onnx",
            "--cls",
            "ch_ppocr_mobile_v2.0_cls_infer.onnx",
            "--rec",
            "ch_PP-OCRv3_rec_infer.onnx",
            "--keys",
            "ppocr_keys_v1.txt",
            "--image",
            image_path.to_str().unwrap(),
            "--numThread",
            thread_num.to_string().as_str(),
            "--padding",
            "50",
            "--maxSideLen",
            "1024",
            "--boxScoreThresh",
            "0.5",
            "--boxThresh",
            "0.3",
            "--unClipRatio",
            "1.6",
            "--doAngle",
            "1",
            "--mostAngle",
            "1",
        ])
        .output()?;
    let result = String::from_utf8_lossy(&output.stdout);
    let list = result.split("=====End detect=====\n");
    let result = list.last().unwrap();
    let list = result.split("s)\n");
    let result = list.last().unwrap();
    Ok(Value::String(result.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = recognize("", "", needs);
        println!("{result:?}");
    }
}
