extern crate proc_macro;

use proc_macro2::TokenStream;

use quote::quote;

use syn::ReturnType;

use darling::FromMeta;

#[cfg(not(test))]
#[proc_macro_attribute]
pub fn main(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;
    let attrs = &func.attrs;

    if let ReturnType::Type(_, _) = func.sig.output {
        return syn::Error::new_spanned(func.sig.output, "should return '()'.")
            .to_compile_error()
            .into();
    }

    (quote! {
        #[export_name = "AppInfo"]
        pub extern "stdcall" fn app_info() -> *const ::std::os::raw::c_char {
            coolq_sdk_rust::api::Convert::from(format!("{},{}", coolq_sdk_rust::APIVER, include_str!(concat!(env!("OUT_DIR"), "/appid")))).into()
        }

        #[no_mangle]
        pub extern "stdcall" fn on_enable() -> i32 {
            #(#attrs)*
            #func
            #func_name();
            0
        }
    }).into()
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    event: String,
    #[darling(default)]
    priority: Option<String>,
}

#[proc_macro_attribute]
pub fn listener(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr2 = attr.clone();
    let args = MacroArgs::from_list(&syn::parse_macro_input!(attr as syn::AttributeArgs)).unwrap();
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;
    let attrs = &func.attrs;

    let event = args.event.parse::<TokenStream>().unwrap();
    if let Some(extern_func_info) = get_event_func(args.event.as_ref()) {
        let extern_func_name = if let Some(priority) = args.priority {
            format!("{}_{}", extern_func_info.0, priority)
        } else {
            extern_func_info.0
        }
        .parse::<TokenStream>()
        .unwrap();

        let args_name_t = extern_func_info.1.parse::<TokenStream>().unwrap();
        let args_name = extern_func_info.2.parse::<TokenStream>().unwrap();
        let result_type = extern_func_info.3.parse::<TokenStream>().unwrap();
        (quote! {
            #[no_mangle]
            pub extern "stdcall" fn #extern_func_name(#args_name_t) -> #result_type {
                #(#attrs)*
                #func
                Convert::from(#func_name(&mut coolq_sdk_rust::events::#event::new(#args_name))).into()
            }
        }).into()
    } else {
        return syn::Error::new_spanned(TokenStream::from(attr2), "Cannot find this event.")
            .to_compile_error()
            .into();
    }
}

macro_rules! gen_get_event_func {
    ($(($event: ident, $func_name: ident; $($arg: ident: $t: ty),* => $result_t: ty)),*) => {
        fn get_event_func(event: &str) -> Option<(String, String, String, String)> {
            match event {
                $(stringify!($event) => Some((
                    stringify!($func_name).to_string(),
                    stringify!($($arg: $t),*).to_string(),
                    stringify!($($arg),*).to_string(),
                    stringify!($result_t).to_string()
                ))),*,
                _ => None,
            }
        }
    }
}

gen_get_event_func!(
            (PrivateMessageEvent, on_private_msg;
                sub_type: i32,
                msg_id: i32,
                user_id: i64,
                msg: *const ::std::os::raw::c_char,
                font: i32
            => i32),
            (GroupMessageEvent, on_group_msg;
                sub_type: i32,
                msg_id: i32,
                group_id: i64,
                user_id: i64,
                anonymous_flag: *const ::std::os::raw::c_char,
                msg: *const ::std::os::raw::c_char,
                font: i32
            => i32),
            (DiscussMessageEvent, on_discuss_msg;
                sub_type: i32,
                msg_id: i32,
                discuss_id: i64,
                user_id: i64,
                msg: *const ::std::os::raw::c_char,
                font: i32
            => i32),
            (GroupUploadEvent, on_group_upload;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64,
                file: *const ::std::os::raw::c_char
            => i32),
            (GroupAdminEvent, on_group_admin;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64
            => i32),
            (GroupMemberDecreaseEvent, on_group_member_decrease;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64
            => i32),
            (GroupMemberIncreaseEvent, on_group_member_increase;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64
            => i32),
            (GroupBanEvent, on_group_ban;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64,
                time: i64
            => i32),
            (FriendAddEvent, on_friend_add;
                sub_type: i32,
                send_time: i32,
                user_id: i64
            => i32),
            (AddFriendRequestEvent, on_add_friend_request;
                sub_type: i32,
                send_time: i32,
                user_id: i64,
                msg: *const ::std::os::raw::c_char,
                flag: *const ::std::os::raw::c_char
            => i32),
            (AddGroupRequestEvent, on_add_group_request;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64,
                msg: *const ::std::os::raw::c_char,
                flag: *const ::std::os::raw::c_char
            => i32)
);
