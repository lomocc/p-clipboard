use clipboard_win::Clipboard;
use std::io::Result;
use std::path::PathBuf;

use neon::prelude::*;
use neon::register_module;

fn get_file_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let vec_path_buf: Result<Vec<PathBuf>> = Clipboard::new().unwrap().get_file_list();
    match vec_path_buf {
        Ok(vec_path_buf) => {
            let js_array = JsArray::new(&mut cx, vec_path_buf.len() as u32);
            for (i, obj) in vec_path_buf.iter().enumerate() {
                let file_path_str = obj.clone().into_os_string().into_string().unwrap();
                let js_string = cx.string(file_path_str);
                js_array.set(&mut cx, i as u32, js_string).unwrap();
            }
            return Ok(js_array);
        }
        Err(_error) => {
            return Ok(cx.empty_array());
        }
    };
}
register_module!(mut m, { m.export_function("getFileList", get_file_list) });
