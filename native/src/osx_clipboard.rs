use neon::prelude::*;
use neon::register_module;

use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSObject, NSString};
use objc_id::Id;

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

// objc_foundation::object_struct!(NSPasteboard);
// objc_foundation::object_struct!(NSPasteboardItem);

fn get_file_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pasteboard: Id<NSObject> =
        unsafe { Id::from_ptr(msg_send![class!(NSPasteboard), generalPasteboard]) };
    #[allow(non_snake_case)]
    let NSPasteboardTypeFileURL: Id<NSString> = NSString::from_str("public.file-url");
    let types: Id<NSArray<NSString>> = unsafe { Id::from_ptr(msg_send![pasteboard, types]) };
    let contains = types
        .object_enumerator()
        .any(|x| NSPasteboardTypeFileURL.is_equal(x));

    let js_array = cx.empty_array();
    if contains {
        #[allow(non_snake_case)]
        let NSURL = class!(NSURL);
        let item_list: Id<NSArray<NSObject>> =
            unsafe { Id::from_ptr(msg_send![pasteboard, pasteboardItems]) };

        let mut i: u32 = 0;
        for item in item_list.object_enumerator() {
            // let types: Id<NSArray<NSString>> = unsafe { Id::from_ptr(msg_send![item, types]) };
            // let item_type = types.first_object();
            // println!("types = {:?}", types.to_vec());
            unsafe {
                let public_file_url: *mut NSString =
                    msg_send![item, stringForType: &*NSPasteboardTypeFileURL];
                if !public_file_url.is_null() {
                    let file_url: Id<NSObject> =
                        Id::from_ptr(msg_send![NSURL, URLWithString: public_file_url]);
                    let file_path: Id<NSString> = Id::from_ptr(msg_send![file_url, path]);

                    let js_string = cx.string(file_path.as_str());
                    js_array.set(&mut cx, i, js_string).unwrap();
                    i = i + 1;
                }
            };
        }
    }
    return Ok(js_array);
}
register_module!(mut m, { m.export_function("getFileList", get_file_list) });
