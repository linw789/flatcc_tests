// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod ddsettings {

    use std::cmp::Ordering;
    use std::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};
    #[allow(unused_imports, dead_code)]
    pub mod rpc {

        use std::cmp::Ordering;
        use std::mem;

        extern crate flatbuffers;
        use self::flatbuffers::{EndianScalar, Follow};

        #[deprecated(
            since = "2.0.0",
            note = "Use associated constants instead. This will no longer be generated in 2021."
        )]
        pub const ENUM_MIN_SETTINGS_TYPE: i8 = 0;
        #[deprecated(
            since = "2.0.0",
            note = "Use associated constants instead. This will no longer be generated in 2021."
        )]
        pub const ENUM_MAX_SETTINGS_TYPE: i8 = 10;
        #[deprecated(
            since = "2.0.0",
            note = "Use associated constants instead. This will no longer be generated in 2021."
        )]
        #[allow(non_camel_case_types)]
        pub const ENUM_VALUES_SETTINGS_TYPE: [SettingsType; 11] = [
            SettingsType::Bool,
            SettingsType::Int8,
            SettingsType::Uint8,
            SettingsType::Int16,
            SettingsType::Uint16,
            SettingsType::Int32,
            SettingsType::Uint32,
            SettingsType::Int64,
            SettingsType::Uint64,
            SettingsType::Float,
            SettingsType::String,
        ];

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct SettingsType(pub i8);
        #[allow(non_upper_case_globals)]
        impl SettingsType {
            pub const Bool: Self = Self(0);
            pub const Int8: Self = Self(1);
            pub const Uint8: Self = Self(2);
            pub const Int16: Self = Self(3);
            pub const Uint16: Self = Self(4);
            pub const Int32: Self = Self(5);
            pub const Uint32: Self = Self(6);
            pub const Int64: Self = Self(7);
            pub const Uint64: Self = Self(8);
            pub const Float: Self = Self(9);
            pub const String: Self = Self(10);

            pub const ENUM_MIN: i8 = 0;
            pub const ENUM_MAX: i8 = 10;
            pub const ENUM_VALUES: &'static [Self] = &[
                Self::Bool,
                Self::Int8,
                Self::Uint8,
                Self::Int16,
                Self::Uint16,
                Self::Int32,
                Self::Uint32,
                Self::Int64,
                Self::Uint64,
                Self::Float,
                Self::String,
            ];
            /// Returns the variant's name or "" if unknown.
            pub fn variant_name(self) -> Option<&'static str> {
                match self {
                    Self::Bool => Some("Bool"),
                    Self::Int8 => Some("Int8"),
                    Self::Uint8 => Some("Uint8"),
                    Self::Int16 => Some("Int16"),
                    Self::Uint16 => Some("Uint16"),
                    Self::Int32 => Some("Int32"),
                    Self::Uint32 => Some("Uint32"),
                    Self::Int64 => Some("Int64"),
                    Self::Uint64 => Some("Uint64"),
                    Self::Float => Some("Float"),
                    Self::String => Some("String"),
                    _ => None,
                }
            }
        }
        impl std::fmt::Debug for SettingsType {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if let Some(name) = self.variant_name() {
                    f.write_str(name)
                } else {
                    f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
                }
            }
        }
        impl<'a> flatbuffers::Follow<'a> for SettingsType {
            type Inner = Self;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                let b = unsafe { flatbuffers::read_scalar_at::<i8>(buf, loc) };
                Self(b)
            }
        }

        impl flatbuffers::Push for SettingsType {
            type Output = SettingsType;
            #[inline]
            fn push(&self, dst: &mut [u8], _rest: &[u8]) {
                unsafe {
                    flatbuffers::emplace_scalar::<i8>(dst, self.0);
                }
            }
        }

        impl flatbuffers::EndianScalar for SettingsType {
            #[inline]
            fn to_little_endian(self) -> Self {
                let b = i8::to_le(self.0);
                Self(b)
            }
            #[inline]
            #[allow(clippy::wrong_self_convention)]
            fn from_little_endian(self) -> Self {
                let b = i8::from_le(self.0);
                Self(b)
            }
        }

        impl<'a> flatbuffers::Verifiable for SettingsType {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                i8::run_verifier(v, pos)
            }
        }

        impl flatbuffers::SimpleToVerifyInSlice for SettingsType {}
        pub enum SettingsValueOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SettingsValue<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SettingsValue<'a> {
            type Inner = SettingsValue<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> SettingsValue<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SettingsValue { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SettingsValueArgs<'args>,
            ) -> flatbuffers::WIPOffset<SettingsValue<'bldr>> {
                let mut builder = SettingsValueBuilder::new(_fbb);
                if let Some(x) = args.value {
                    builder.add_value(x);
                }
                builder.add_size_(args.size_);
                builder.add_hash(args.hash);
                builder.add_type_(args.type_);
                builder.finish()
            }

            pub const VT_HASH: flatbuffers::VOffsetT = 4;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 6;
            pub const VT_SIZE_: flatbuffers::VOffsetT = 8;
            pub const VT_VALUE: flatbuffers::VOffsetT = 10;

            #[inline]
            pub fn hash(&self) -> u32 {
                self._tab
                    .get::<u32>(SettingsValue::VT_HASH, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn type_(&self) -> SettingsType {
                self._tab
                    .get::<SettingsType>(SettingsValue::VT_TYPE_, Some(SettingsType::Bool))
                    .unwrap()
            }
            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab
                    .get::<u32>(SettingsValue::VT_SIZE_, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn value(&self) -> Option<&'a [u8]> {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                        SettingsValue::VT_VALUE,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
        }

        impl flatbuffers::Verifiable for SettingsValue<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<u32>(&"hash", Self::VT_HASH, false)?
                    .visit_field::<SettingsType>(&"type_", Self::VT_TYPE_, false)?
                    .visit_field::<u32>(&"size_", Self::VT_SIZE_, false)?
                    .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                        &"value",
                        Self::VT_VALUE,
                        false,
                    )?
                    .finish();
                Ok(())
            }
        }
        pub struct SettingsValueArgs<'a> {
            pub hash: u32,
            pub type_: SettingsType,
            pub size_: u32,
            pub value: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        }
        impl<'a> Default for SettingsValueArgs<'a> {
            #[inline]
            fn default() -> Self {
                SettingsValueArgs {
                    hash: 0,
                    type_: SettingsType::Bool,
                    size_: 0,
                    value: None,
                }
            }
        }
        pub struct SettingsValueBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SettingsValueBuilder<'a, 'b> {
            #[inline]
            pub fn add_hash(&mut self, hash: u32) {
                self.fbb_.push_slot::<u32>(SettingsValue::VT_HASH, hash, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: SettingsType) {
                self.fbb_.push_slot::<SettingsType>(
                    SettingsValue::VT_TYPE_,
                    type_,
                    SettingsType::Bool,
                );
            }
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_
                    .push_slot::<u32>(SettingsValue::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_value(
                &mut self,
                value: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(SettingsValue::VT_VALUE, value);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SettingsValueBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SettingsValueBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SettingsValue<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl std::fmt::Debug for SettingsValue<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut ds = f.debug_struct("SettingsValue");
                ds.field("hash", &self.hash());
                ds.field("type_", &self.type_());
                ds.field("size_", &self.size_());
                ds.field("value", &self.value());
                ds.finish()
            }
        }
        pub enum SettingsComponentValuesOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SettingsComponentValues<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SettingsComponentValues<'a> {
            type Inner = SettingsComponentValues<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> SettingsComponentValues<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SettingsComponentValues { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SettingsComponentValuesArgs<'args>,
            ) -> flatbuffers::WIPOffset<SettingsComponentValues<'bldr>> {
                let mut builder = SettingsComponentValuesBuilder::new(_fbb);
                builder.add_component_hash(args.component_hash);
                if let Some(x) = args.values {
                    builder.add_values(x);
                }
                if let Some(x) = args.component_name {
                    builder.add_component_name(x);
                }
                builder.finish()
            }

            pub const VT_COMPONENT_NAME: flatbuffers::VOffsetT = 4;
            pub const VT_COMPONENT_HASH: flatbuffers::VOffsetT = 6;
            pub const VT_VALUES: flatbuffers::VOffsetT = 8;

            #[inline]
            pub fn component_name(&self) -> Option<&'a str> {
                self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(
                    SettingsComponentValues::VT_COMPONENT_NAME,
                    None,
                )
            }
            #[inline]
            pub fn component_hash(&self) -> u64 {
                self._tab
                    .get::<u64>(SettingsComponentValues::VT_COMPONENT_HASH, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn values(
                &self,
            ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SettingsValue<'a>>>>
            {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SettingsValue>>,
                >>(SettingsComponentValues::VT_VALUES, None)
            }
        }

        impl flatbuffers::Verifiable for SettingsComponentValues<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        &"component_name",
                        Self::VT_COMPONENT_NAME,
                        false,
                    )?
                    .visit_field::<u64>(&"component_hash", Self::VT_COMPONENT_HASH, false)?
                    .visit_field::<flatbuffers::ForwardsUOffset<
                        flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<SettingsValue>>,
                    >>(&"values", Self::VT_VALUES, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct SettingsComponentValuesArgs<'a> {
            pub component_name: Option<flatbuffers::WIPOffset<&'a str>>,
            pub component_hash: u64,
            pub values: Option<
                flatbuffers::WIPOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SettingsValue<'a>>>,
                >,
            >,
        }
        impl<'a> Default for SettingsComponentValuesArgs<'a> {
            #[inline]
            fn default() -> Self {
                SettingsComponentValuesArgs {
                    component_name: None,
                    component_hash: 0,
                    values: None,
                }
            }
        }
        pub struct SettingsComponentValuesBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SettingsComponentValuesBuilder<'a, 'b> {
            #[inline]
            pub fn add_component_name(&mut self, component_name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    SettingsComponentValues::VT_COMPONENT_NAME,
                    component_name,
                );
            }
            #[inline]
            pub fn add_component_hash(&mut self, component_hash: u64) {
                self.fbb_.push_slot::<u64>(
                    SettingsComponentValues::VT_COMPONENT_HASH,
                    component_hash,
                    0,
                );
            }
            #[inline]
            pub fn add_values(
                &mut self,
                values: flatbuffers::WIPOffset<
                    flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<SettingsValue<'b>>>,
                >,
            ) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    SettingsComponentValues::VT_VALUES,
                    values,
                );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SettingsComponentValuesBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SettingsComponentValuesBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SettingsComponentValues<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl std::fmt::Debug for SettingsComponentValues<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut ds = f.debug_struct("SettingsComponentValues");
                ds.field("component_name", &self.component_name());
                ds.field("component_hash", &self.component_hash());
                ds.field("values", &self.values());
                ds.finish()
            }
        }
        pub enum SettingsValuesAllOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SettingsValuesAll<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SettingsValuesAll<'a> {
            type Inner = SettingsValuesAll<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> SettingsValuesAll<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SettingsValuesAll { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SettingsValuesAllArgs<'args>,
            ) -> flatbuffers::WIPOffset<SettingsValuesAll<'bldr>> {
                let mut builder = SettingsValuesAllBuilder::new(_fbb);
                if let Some(x) = args.components {
                    builder.add_components(x);
                }
                builder.finish()
            }

            pub const VT_COMPONENTS: flatbuffers::VOffsetT = 4;

            #[inline]
            pub fn components(
                &self,
            ) -> Option<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SettingsComponentValues<'a>>>,
            > {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SettingsComponentValues>>,
                >>(SettingsValuesAll::VT_COMPONENTS, None)
            }
        }

        impl flatbuffers::Verifiable for SettingsValuesAll<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<
                        flatbuffers::Vector<
                            '_,
                            flatbuffers::ForwardsUOffset<SettingsComponentValues>,
                        >,
                    >>(&"components", Self::VT_COMPONENTS, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct SettingsValuesAllArgs<'a> {
            pub components: Option<
                flatbuffers::WIPOffset<
                    flatbuffers::Vector<
                        'a,
                        flatbuffers::ForwardsUOffset<SettingsComponentValues<'a>>,
                    >,
                >,
            >,
        }
        impl<'a> Default for SettingsValuesAllArgs<'a> {
            #[inline]
            fn default() -> Self {
                SettingsValuesAllArgs { components: None }
            }
        }
        pub struct SettingsValuesAllBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SettingsValuesAllBuilder<'a, 'b> {
            #[inline]
            pub fn add_components(
                &mut self,
                components: flatbuffers::WIPOffset<
                    flatbuffers::Vector<
                        'b,
                        flatbuffers::ForwardsUOffset<SettingsComponentValues<'b>>,
                    >,
                >,
            ) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    SettingsValuesAll::VT_COMPONENTS,
                    components,
                );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SettingsValuesAllBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SettingsValuesAllBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SettingsValuesAll<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl std::fmt::Debug for SettingsValuesAll<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut ds = f.debug_struct("SettingsValuesAll");
                ds.field("components", &self.components());
                ds.finish()
            }
        }
    } // pub mod Rpc
} // pub mod DDSettings
