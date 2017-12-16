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
pub struct ExecRequest {
    // message fields
    pub command: ::std::string::String,
    pub args: ::std::string::String,
    pub input: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecRequest {}

impl ExecRequest {
    pub fn new() -> ExecRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExecRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecRequest,
        };
        unsafe {
            instance.get(ExecRequest::new)
        }
    }

    // string command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::string::String) {
        self.command = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::string::String {
        &mut self.command
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.command, ::std::string::String::new())
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    fn get_command_for_reflect(&self) -> &::std::string::String {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.command
    }

    // string args = 2;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::std::string::String) {
        self.args = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_args(&mut self) -> &mut ::std::string::String {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.args, ::std::string::String::new())
    }

    pub fn get_args(&self) -> &str {
        &self.args
    }

    fn get_args_for_reflect(&self) -> &::std::string::String {
        &self.args
    }

    fn mut_args_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.args
    }

    // string input = 3;

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    // Param is passed by value, moved
    pub fn set_input(&mut self, v: ::std::string::String) {
        self.input = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input(&mut self) -> &mut ::std::string::String {
        &mut self.input
    }

    // Take field
    pub fn take_input(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.input, ::std::string::String::new())
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    fn get_input_for_reflect(&self) -> &::std::string::String {
        &self.input
    }

    fn mut_input_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.input
    }
}

impl ::protobuf::Message for ExecRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.command)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.args)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.input)?;
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
        if !self.command.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.command);
        }
        if !self.args.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.args);
        }
        if !self.input.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.input);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.command.is_empty() {
            os.write_string(1, &self.command)?;
        }
        if !self.args.is_empty() {
            os.write_string(2, &self.args)?;
        }
        if !self.input.is_empty() {
            os.write_string(3, &self.input)?;
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

impl ::protobuf::MessageStatic for ExecRequest {
    fn new() -> ExecRequest {
        ExecRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command",
                    ExecRequest::get_command_for_reflect,
                    ExecRequest::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "args",
                    ExecRequest::get_args_for_reflect,
                    ExecRequest::mut_args_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input",
                    ExecRequest::get_input_for_reflect,
                    ExecRequest::mut_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecRequest>(
                    "ExecRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecRequest {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_args();
        self.clear_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecResponse {
    // message fields
    pub result: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecResponse {}

impl ExecResponse {
    pub fn new() -> ExecResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecResponse {
        static mut instance: ::protobuf::lazy::Lazy<ExecResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecResponse,
        };
        unsafe {
            instance.get(ExecResponse::new)
        }
    }

    // string result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::std::string::String) {
        self.result = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ::std::string::String {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.result, ::std::string::String::new())
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }

    fn get_result_for_reflect(&self) -> &::std::string::String {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.result
    }
}

impl ::protobuf::Message for ExecResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.result)?;
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
        if !self.result.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.result);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.result.is_empty() {
            os.write_string(1, &self.result)?;
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

impl ::protobuf::MessageStatic for ExecResponse {
    fn new() -> ExecResponse {
        ExecResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "result",
                    ExecResponse::get_result_for_reflect,
                    ExecResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecResponse>(
                    "ExecResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClearCacheRequest {
    // message fields
    pub command: ::std::string::String,
    pub args: ::std::string::String,
    pub all: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClearCacheRequest {}

impl ClearCacheRequest {
    pub fn new() -> ClearCacheRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClearCacheRequest {
        static mut instance: ::protobuf::lazy::Lazy<ClearCacheRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClearCacheRequest,
        };
        unsafe {
            instance.get(ClearCacheRequest::new)
        }
    }

    // string command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::string::String) {
        self.command = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::string::String {
        &mut self.command
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.command, ::std::string::String::new())
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }

    fn get_command_for_reflect(&self) -> &::std::string::String {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.command
    }

    // string args = 2;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::std::string::String) {
        self.args = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_args(&mut self) -> &mut ::std::string::String {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.args, ::std::string::String::new())
    }

    pub fn get_args(&self) -> &str {
        &self.args
    }

    fn get_args_for_reflect(&self) -> &::std::string::String {
        &self.args
    }

    fn mut_args_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.args
    }

    // bool all = 3;

    pub fn clear_all(&mut self) {
        self.all = false;
    }

    // Param is passed by value, moved
    pub fn set_all(&mut self, v: bool) {
        self.all = v;
    }

    pub fn get_all(&self) -> bool {
        self.all
    }

    fn get_all_for_reflect(&self) -> &bool {
        &self.all
    }

    fn mut_all_for_reflect(&mut self) -> &mut bool {
        &mut self.all
    }
}

impl ::protobuf::Message for ClearCacheRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.command)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.args)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.all = tmp;
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
        if !self.command.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.command);
        }
        if !self.args.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.args);
        }
        if self.all != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.command.is_empty() {
            os.write_string(1, &self.command)?;
        }
        if !self.args.is_empty() {
            os.write_string(2, &self.args)?;
        }
        if self.all != false {
            os.write_bool(3, self.all)?;
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

impl ::protobuf::MessageStatic for ClearCacheRequest {
    fn new() -> ClearCacheRequest {
        ClearCacheRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClearCacheRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command",
                    ClearCacheRequest::get_command_for_reflect,
                    ClearCacheRequest::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "args",
                    ClearCacheRequest::get_args_for_reflect,
                    ClearCacheRequest::mut_args_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "all",
                    ClearCacheRequest::get_all_for_reflect,
                    ClearCacheRequest::mut_all_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClearCacheRequest>(
                    "ClearCacheRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClearCacheRequest {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_args();
        self.clear_all();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClearCacheRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClearCacheRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClearCacheResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClearCacheResponse {}

impl ClearCacheResponse {
    pub fn new() -> ClearCacheResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClearCacheResponse {
        static mut instance: ::protobuf::lazy::Lazy<ClearCacheResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClearCacheResponse,
        };
        unsafe {
            instance.get(ClearCacheResponse::new)
        }
    }
}

impl ::protobuf::Message for ClearCacheResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ClearCacheResponse {
    fn new() -> ClearCacheResponse {
        ClearCacheResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClearCacheResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ClearCacheResponse>(
                    "ClearCacheResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClearCacheResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClearCacheResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClearCacheResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckRequest {}

impl HealthCheckRequest {
    pub fn new() -> HealthCheckRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckRequest,
        };
        unsafe {
            instance.get(HealthCheckRequest::new)
        }
    }
}

impl ::protobuf::Message for HealthCheckRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for HealthCheckRequest {
    fn new() -> HealthCheckRequest {
        HealthCheckRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckRequest>(
                    "HealthCheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckResponse {
    // message fields
    pub status: ServerStatus,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckResponse {}

impl HealthCheckResponse {
    pub fn new() -> HealthCheckResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckResponse,
        };
        unsafe {
            instance.get(HealthCheckResponse::new)
        }
    }

    // .zothcore.ServerStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ServerStatus::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ServerStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> ServerStatus {
        self.status
    }

    fn get_status_for_reflect(&self) -> &ServerStatus {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ServerStatus {
        &mut self.status
    }
}

impl ::protobuf::Message for HealthCheckResponse {
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
                    let tmp = is.read_enum()?;
                    self.status = tmp;
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
        if self.status != ServerStatus::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != ServerStatus::UNKNOWN {
            os.write_enum(1, self.status.value())?;
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

impl ::protobuf::MessageStatic for HealthCheckResponse {
    fn new() -> HealthCheckResponse {
        HealthCheckResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServerStatus>>(
                    "status",
                    HealthCheckResponse::get_status_for_reflect,
                    HealthCheckResponse::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckResponse>(
                    "HealthCheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownRequest {
    // message fields
    pub msec_after: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownRequest {}

impl ShutdownRequest {
    pub fn new() -> ShutdownRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownRequest {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownRequest,
        };
        unsafe {
            instance.get(ShutdownRequest::new)
        }
    }

    // int32 msec_after = 1;

    pub fn clear_msec_after(&mut self) {
        self.msec_after = 0;
    }

    // Param is passed by value, moved
    pub fn set_msec_after(&mut self, v: i32) {
        self.msec_after = v;
    }

    pub fn get_msec_after(&self) -> i32 {
        self.msec_after
    }

    fn get_msec_after_for_reflect(&self) -> &i32 {
        &self.msec_after
    }

    fn mut_msec_after_for_reflect(&mut self) -> &mut i32 {
        &mut self.msec_after
    }
}

impl ::protobuf::Message for ShutdownRequest {
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
                    let tmp = is.read_int32()?;
                    self.msec_after = tmp;
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
        if self.msec_after != 0 {
            my_size += ::protobuf::rt::value_size(1, self.msec_after, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.msec_after != 0 {
            os.write_int32(1, self.msec_after)?;
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

impl ::protobuf::MessageStatic for ShutdownRequest {
    fn new() -> ShutdownRequest {
        ShutdownRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msec_after",
                    ShutdownRequest::get_msec_after_for_reflect,
                    ShutdownRequest::mut_msec_after_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownRequest>(
                    "ShutdownRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownRequest {
    fn clear(&mut self) {
        self.clear_msec_after();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownResponse {}

impl ShutdownResponse {
    pub fn new() -> ShutdownResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownResponse {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownResponse,
        };
        unsafe {
            instance.get(ShutdownResponse::new)
        }
    }
}

impl ::protobuf::Message for ShutdownResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ShutdownResponse {
    fn new() -> ShutdownResponse {
        ShutdownResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownResponse>(
                    "ShutdownResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServerStatus {
    UNKNOWN = 0,
    ALIVE = 1,
    NOT_ALIVE = 2,
}

impl ::protobuf::ProtobufEnum for ServerStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServerStatus> {
        match value {
            0 => ::std::option::Option::Some(ServerStatus::UNKNOWN),
            1 => ::std::option::Option::Some(ServerStatus::ALIVE),
            2 => ::std::option::Option::Some(ServerStatus::NOT_ALIVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServerStatus] = &[
            ServerStatus::UNKNOWN,
            ServerStatus::ALIVE,
            ServerStatus::NOT_ALIVE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ServerStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ServerStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ServerStatus {
}

impl ::std::default::Default for ServerStatus {
    fn default() -> Self {
        ServerStatus::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14proto/zothcore.proto\x12\x08zothcore\"Q\n\x0bExecRequest\x12\x18\n\
    \x07command\x18\x01\x20\x01(\tR\x07command\x12\x12\n\x04args\x18\x02\x20\
    \x01(\tR\x04args\x12\x14\n\x05input\x18\x03\x20\x01(\tR\x05input\"&\n\
    \x0cExecResponse\x12\x16\n\x06result\x18\x01\x20\x01(\tR\x06result\"S\n\
    \x11ClearCacheRequest\x12\x18\n\x07command\x18\x01\x20\x01(\tR\x07comman\
    d\x12\x12\n\x04args\x18\x02\x20\x01(\tR\x04args\x12\x10\n\x03all\x18\x03\
    \x20\x01(\x08R\x03all\"\x14\n\x12ClearCacheResponse\"\x14\n\x12HealthChe\
    ckRequest\"E\n\x13HealthCheckResponse\x12.\n\x06status\x18\x01\x20\x01(\
    \x0e2\x16.zothcore.ServerStatusR\x06status\"0\n\x0fShutdownRequest\x12\
    \x1d\n\nmsec_after\x18\x01\x20\x01(\x05R\tmsecAfter\"\x12\n\x10ShutdownR\
    esponse*5\n\x0cServerStatus\x12\x0b\n\x07UNKNOWN\x10\0\x12\t\n\x05ALIVE\
    \x10\x01\x12\r\n\tNOT_ALIVE\x10\x022\xa1\x02\n\x08ZothCore\x127\n\x04Exe\
    c\x12\x15.zothcore.ExecRequest\x1a\x16.zothcore.ExecResponse\"\0\x12I\n\
    \nClearCache\x12\x1b.zothcore.ClearCacheRequest\x1a\x1c.zothcore.ClearCa\
    cheResponse\"\0\x12L\n\x0bHealthCheck\x12\x1c.zothcore.HealthCheckReques\
    t\x1a\x1d.zothcore.HealthCheckResponse\"\0\x12C\n\x08Shutdown\x12\x19.zo\
    thcore.ShutdownRequest\x1a\x1a.zothcore.ShutdownResponse\"\0J\xd9\t\n\
    \x06\x12\x04\0\00\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x10\n\n\n\x02\x06\0\x12\x04\x04\0\t\x01\n\n\n\x03\x06\0\
    \x01\x12\x03\x04\x08\x10\n\x0b\n\x04\x06\0\x02\0\x12\x03\x05\x044\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03\x05\x08\x0c\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x05\x0e\x19\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x05$0\n\x0b\n\
    \x04\x06\0\x02\x01\x12\x03\x06\x04F\n\x0c\n\x05\x06\0\x02\x01\x01\x12\
    \x03\x06\x08\x12\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x06\x14%\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x03\x060B\n\x0b\n\x04\x06\0\x02\x02\x12\x03\
    \x07\x04I\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x07\x08\x13\n\x0c\n\x05\
    \x06\0\x02\x02\x02\x12\x03\x07\x15'\n\x0c\n\x05\x06\0\x02\x02\x03\x12\
    \x03\x072E\n\x0b\n\x04\x06\0\x02\x03\x12\x03\x08\x04@\n\x0c\n\x05\x06\0\
    \x02\x03\x01\x12\x03\x08\x08\x10\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\
    \x08\x12!\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\x08,<\n\n\n\x02\x04\0\
    \x12\x04\x0b\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x13\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x0c\x04\x17\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x0c\x04\x0b\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0c\x04\n\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x0c\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x0c\x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\r\x04\x14\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\r\x04\x0c\x17\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\r\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x0e\x04\x15\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0e\x04\r\x14\n\x0c\
    \n\x05\x04\0\x02\x02\x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x0e\x0b\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0e\x13\x14\n\
    \n\n\x02\x04\x01\x12\x04\x11\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\x11\
    \x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x12\x04\x16\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x12\x04\x11\x16\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x12\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x12\x0b\x11\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x12\x14\x15\n\n\n\x02\x04\x02\x12\x04\x15\0\
    \x19\x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\x08\x19\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x16\x04\x17\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x16\x04\
    \x15\x1b\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x16\x04\n\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03\x16\x0b\x12\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\
    \x16\x15\x16\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x17\x04\x14\n\r\n\x05\
    \x04\x02\x02\x01\x04\x12\x04\x17\x04\x16\x17\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x17\x0b\
    \x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x17\x12\x13\n\x0b\n\x04\x04\
    \x02\x02\x02\x12\x03\x18\x04\x11\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\
    \x18\x04\x17\x14\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x18\x04\x08\n\
    \x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x18\t\x0c\n\x0c\n\x05\x04\x02\x02\
    \x02\x03\x12\x03\x18\x0f\x10\n\n\n\x02\x04\x03\x12\x04\x1b\0\x1c\x01\n\n\
    \n\x03\x04\x03\x01\x12\x03\x1b\x08\x1a\n\n\n\x02\x05\0\x12\x04\x1e\0\"\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\x1e\x05\x11\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x1f\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x1f\x04\x0b\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x1f\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\
    \x03\x20\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x20\x04\t\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\x20\x0c\r\n\x0b\n\x04\x05\0\x02\x02\x12\
    \x03!\x04\x12\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03!\x04\r\n\x0c\n\x05\
    \x05\0\x02\x02\x02\x12\x03!\x10\x11\n\n\n\x02\x04\x04\x12\x04$\0%\x01\n\
    \n\n\x03\x04\x04\x01\x12\x03$\x08\x1a\n\n\n\x02\x04\x05\x12\x04'\0)\x01\
    \n\n\n\x03\x04\x05\x01\x12\x03'\x08\x1b\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x03(\x04\x1c\n\r\n\x05\x04\x05\x02\0\x04\x12\x04(\x04'\x1d\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x03(\x04\x10\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03(\x11\x17\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03(\x1a\x1b\n\n\n\x02\
    \x04\x06\x12\x04+\0-\x01\n\n\n\x03\x04\x06\x01\x12\x03+\x08\x17\n\x0b\n\
    \x04\x04\x06\x02\0\x12\x03,\x04\x19\n\r\n\x05\x04\x06\x02\0\x04\x12\x04,\
    \x04+\x19\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03,\x04\t\n\x0c\n\x05\x04\
    \x06\x02\0\x01\x12\x03,\n\x14\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03,\x17\
    \x18\n\n\n\x02\x04\x07\x12\x04/\00\x01\n\n\n\x03\x04\x07\x01\x12\x03/\
    \x08\x18b\x06proto3\
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
