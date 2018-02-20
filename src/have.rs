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
pub struct Have {
    // message fields
    start: ::std::option::Option<u64>,
    length: ::std::option::Option<u64>,
    bitfield: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Have {}

impl Have {
    pub fn new() -> Have {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Have {
        static mut instance: ::protobuf::lazy::Lazy<Have> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Have,
        };
        unsafe {
            instance.get(Have::new)
        }
    }

    // required uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 length = 2;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(1u64)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // optional bytes bitfield = 3;

    pub fn clear_bitfield(&mut self) {
        self.bitfield.clear();
    }

    pub fn has_bitfield(&self) -> bool {
        self.bitfield.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bitfield(&mut self, v: ::std::vec::Vec<u8>) {
        self.bitfield = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bitfield(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bitfield.is_none() {
            self.bitfield.set_default();
        }
        self.bitfield.as_mut().unwrap()
    }

    // Take field
    pub fn take_bitfield(&mut self) -> ::std::vec::Vec<u8> {
        self.bitfield.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bitfield(&self) -> &[u8] {
        match self.bitfield.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_bitfield_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bitfield
    }

    fn mut_bitfield_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bitfield
    }
}

impl ::protobuf::Message for Have {
    fn is_initialized(&self) -> bool {
        if self.start.is_none() {
            return false;
        }
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
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bitfield)?;
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.bitfield.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.bitfield.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for Have {
    fn new() -> Have {
        Have::new()
    }

    fn descriptor_static(_: ::std::option::Option<Have>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    Have::get_start_for_reflect,
                    Have::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    Have::get_length_for_reflect,
                    Have::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bitfield",
                    Have::get_bitfield_for_reflect,
                    Have::mut_bitfield_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Have>(
                    "Have",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Have {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_length();
        self.clear_bitfield();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Have {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Have {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nhave.proto\"S\n\x04Have\x12\x14\n\x05start\x18\x01\x20\x02(\x04R\x05\
    start\x12\x19\n\x06length\x18\x02\x20\x01(\x04:\x011R\x06length\x12\x1a\
    \n\x08bitfield\x18\x03\x20\x01(\x0cR\x08bitfield\
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
