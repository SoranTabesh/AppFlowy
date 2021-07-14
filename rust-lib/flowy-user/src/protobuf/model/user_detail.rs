// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `user_detail.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UserDetail {
    // message fields
    pub id: ::std::string::String,
    pub email: ::std::string::String,
    pub name: ::std::string::String,
    pub status: UserStatus,
    pub workspace: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserDetail {
    fn default() -> &'a UserDetail {
        <UserDetail as ::protobuf::Message>::default_instance()
    }
}

impl UserDetail {
    pub fn new() -> UserDetail {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string email = 2;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string name = 3;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .UserStatus status = 4;


    pub fn get_status(&self) -> UserStatus {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = UserStatus::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: UserStatus) {
        self.status = v;
    }

    // string workspace = 5;


    pub fn get_workspace(&self) -> &str {
        &self.workspace
    }
    pub fn clear_workspace(&mut self) {
        self.workspace.clear();
    }

    // Param is passed by value, moved
    pub fn set_workspace(&mut self, v: ::std::string::String) {
        self.workspace = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workspace(&mut self) -> &mut ::std::string::String {
        &mut self.workspace
    }

    // Take field
    pub fn take_workspace(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.workspace, ::std::string::String::new())
    }
}

impl ::protobuf::Message for UserDetail {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.workspace)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.email);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if self.status != UserStatus::Unknown {
            my_size += ::protobuf::rt::enum_size(4, self.status);
        }
        if !self.workspace.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.workspace);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.email.is_empty() {
            os.write_string(2, &self.email)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if self.status != UserStatus::Unknown {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.status))?;
        }
        if !self.workspace.is_empty() {
            os.write_string(5, &self.workspace)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UserDetail {
        UserDetail::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &UserDetail| { &m.id },
                |m: &mut UserDetail| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &UserDetail| { &m.email },
                |m: &mut UserDetail| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &UserDetail| { &m.name },
                |m: &mut UserDetail| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<UserStatus>>(
                "status",
                |m: &UserDetail| { &m.status },
                |m: &mut UserDetail| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "workspace",
                |m: &UserDetail| { &m.workspace },
                |m: &mut UserDetail| { &mut m.workspace },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserDetail>(
                "UserDetail",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserDetail {
        static instance: ::protobuf::rt::LazyV2<UserDetail> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserDetail::new)
    }
}

impl ::protobuf::Clear for UserDetail {
    fn clear(&mut self) {
        self.id.clear();
        self.email.clear();
        self.name.clear();
        self.status = UserStatus::Unknown;
        self.workspace.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserDetail {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UserStatus {
    Unknown = 0,
    Login = 1,
    Expired = 2,
}

impl ::protobuf::ProtobufEnum for UserStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UserStatus> {
        match value {
            0 => ::std::option::Option::Some(UserStatus::Unknown),
            1 => ::std::option::Option::Some(UserStatus::Login),
            2 => ::std::option::Option::Some(UserStatus::Expired),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UserStatus] = &[
            UserStatus::Unknown,
            UserStatus::Login,
            UserStatus::Expired,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<UserStatus>("UserStatus", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for UserStatus {
}

impl ::std::default::Default for UserStatus {
    fn default() -> Self {
        UserStatus::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for UserStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11user_detail.proto\"\x89\x01\n\nUserDetail\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\tR\x02id\x12\x14\n\x05email\x18\x02\x20\x01(\tR\x05email\x12\
    \x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12#\n\x06status\x18\x04\x20\
    \x01(\x0e2\x0b.UserStatusR\x06status\x12\x1c\n\tworkspace\x18\x05\x20\
    \x01(\tR\tworkspace*1\n\nUserStatus\x12\x0b\n\x07Unknown\x10\0\x12\t\n\
    \x05Login\x10\x01\x12\x0b\n\x07Expired\x10\x02J\xd0\x03\n\x06\x12\x04\0\
    \0\r\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\
    \x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x12\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x03\x04\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x03\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x15\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x13\x14\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x14\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\x0f\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x12\x13\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x06\x04\x1a\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x06\x04\
    \x0e\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\x0f\x15\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x06\x18\x19\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x07\
    \x04\x19\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\
    \0\x02\x04\x01\x12\x03\x07\x0b\x14\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\
    \x07\x17\x18\n\n\n\x02\x05\0\x12\x04\t\0\r\x01\n\n\n\x03\x05\0\x01\x12\
    \x03\t\x05\x0f\n\x0b\n\x04\x05\0\x02\0\x12\x03\n\x04\x10\n\x0c\n\x05\x05\
    \0\x02\0\x01\x12\x03\n\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\n\x0e\
    \x0f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x0b\x04\x0e\n\x0c\n\x05\x05\0\x02\
    \x01\x01\x12\x03\x0b\x04\t\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x0b\x0c\
    \r\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x0c\x04\x10\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x03\x0c\x04\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x0c\
    \x0e\x0fb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
