use neon::prelude::*;
use neon::register_module;

use objc_foundation::{INSArray, INSString, NSArray, NSObject, NSString};
use objc_id::Id;

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

// objc_foundation::object_struct!(NSPasteboard);
// objc_foundation::object_struct!(NSPasteboardItem);

fn get_file_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pasteboard: Id<NSObject> =
        unsafe { Id::from_ptr(msg_send![class!(NSPasteboard), generalPasteboard]) };

    let item_list: Id<NSArray<NSObject>> =
        unsafe { Id::from_ptr(msg_send![pasteboard, pasteboardItems]) };
    let js_array = cx.empty_array();

    let mut i: u32 = 0;
    for item in item_list.object_enumerator() {
        // let types: Id<NSArray<NSString>> = unsafe { Id::from_ptr(msg_send![item, types]) };
        // let item_type = types.first_object();
        // println!("types = {:?}", types.to_vec());
        let public_file_url: Option<Id<NSString>> = unsafe {
            let public_file_url: *mut NSString =
                msg_send![item, stringForType: NSString::from_str("public.file-url")];
            if public_file_url.is_null() {
                None
            } else {
                Some(Id::from_ptr(public_file_url))
            }
        };

        match public_file_url {
            Some(public_file_url) => {
                let js_string = cx.string(public_file_url.as_str());
                js_array.set(&mut cx, i, js_string).unwrap();
                i = i + 1;
            }
            None => {}
        };
    }
    return Ok(js_array);
}
register_module!(mut m, { m.export_function("getFileList", get_file_list) });
