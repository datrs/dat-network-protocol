// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Info {
    // message fields
    uploading: ::std::option::Option<bool>,
    downloading: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Info {}

impl Info {
    pub fn new() -> Info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Info {
        static mut instance: ::protobuf::lazy::Lazy<Info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Info,
        };
        unsafe {
            instance.get(Info::new)
        }
    }

    // optional bool uploading = 1;

    pub fn clear_uploading(&mut self) {
        self.uploading = ::std::option::Option::None;
    }

    pub fn has_uploading(&self) -> bool {
        self.uploading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uploading(&mut self, v: bool) {
        self.uploading = ::std::option::Option::Some(v);
    }

    pub fn get_uploading(&self) -> bool {
        self.uploading.unwrap_or(false)
    }

    fn get_uploading_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.uploading
    }

    fn mut_uploading_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.uploading
    }

    // optional bool downloading = 2;

    pub fn clear_downloading(&mut self) {
        self.downloading = ::std::option::Option::None;
    }

    pub fn has_downloading(&self) -> bool {
        self.downloading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downloading(&mut self, v: bool) {
        self.downloading = ::std::option::Option::Some(v);
    }

    pub fn get_downloading(&self) -> bool {
        self.downloading.unwrap_or(false)
    }

    fn get_downloading_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.downloading
    }

    fn mut_downloading_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.downloading
    }
}

impl ::protobuf::Message for Info {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.uploading = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.downloading = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.uploading {
            my_size += 2;
        }
        if let Some(v) = self.downloading {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uploading {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.downloading {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Info {
    fn new() -> Info {
        Info::new()
    }

    fn descriptor_static(_: ::std::option::Option<Info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "uploading",
                    Info::get_uploading_for_reflect,
                    Info::mut_uploading_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "downloading",
                    Info::get_downloading_for_reflect,
                    Info::mut_downloading_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Info>(
                    "Info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Info {
    fn clear(&mut self) {
        self.clear_uploading();
        self.clear_downloading();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ninfo.proto\"F\n\x04Info\x12\x1c\n\tuploading\x18\x01\x20\x01(\x08R\t\
    uploading\x12\x20\n\x0bdownloading\x18\x02\x20\x01(\x08R\x0bdownloading\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
