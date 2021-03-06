// Generated by Molecule 0.6.1

use super::basic::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct ETHBridgeTypeArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "bridge_lock_hash", self.bridge_lock_hash())?;
        write!(
            f,
            ", {}: {}",
            "recipient_lock_hash",
            self.recipient_lock_hash()
        )?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for ETHBridgeTypeArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        ETHBridgeTypeArgs::new_unchecked(v.into())
    }
}
impl ETHBridgeTypeArgs {
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bridge_lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(0..32))
    }
    pub fn recipient_lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(32..64))
    }
    pub fn as_reader<'r>(&'r self) -> ETHBridgeTypeArgsReader<'r> {
        ETHBridgeTypeArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ETHBridgeTypeArgs {
    type Builder = ETHBridgeTypeArgsBuilder;
    const NAME: &'static str = "ETHBridgeTypeArgs";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ETHBridgeTypeArgs(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .bridge_lock_hash(self.bridge_lock_hash())
            .recipient_lock_hash(self.recipient_lock_hash())
    }
}
#[derive(Clone, Copy)]
pub struct ETHBridgeTypeArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "bridge_lock_hash", self.bridge_lock_hash())?;
        write!(
            f,
            ", {}: {}",
            "recipient_lock_hash",
            self.recipient_lock_hash()
        )?;
        write!(f, " }}")
    }
}
impl<'r> ETHBridgeTypeArgsReader<'r> {
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bridge_lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[0..32])
    }
    pub fn recipient_lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[32..64])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ETHBridgeTypeArgsReader<'r> {
    type Entity = ETHBridgeTypeArgs;
    const NAME: &'static str = "ETHBridgeTypeArgsReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ETHBridgeTypeArgsReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ETHBridgeTypeArgsBuilder {
    pub(crate) bridge_lock_hash: Byte32,
    pub(crate) recipient_lock_hash: Byte32,
}
impl ETHBridgeTypeArgsBuilder {
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bridge_lock_hash(mut self, v: Byte32) -> Self {
        self.bridge_lock_hash = v;
        self
    }
    pub fn recipient_lock_hash(mut self, v: Byte32) -> Self {
        self.recipient_lock_hash = v;
        self
    }
}
impl molecule::prelude::Builder for ETHBridgeTypeArgsBuilder {
    type Entity = ETHBridgeTypeArgs;
    const NAME: &'static str = "ETHBridgeTypeArgsBuilder";
    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        writer.write_all(self.bridge_lock_hash.as_slice())?;
        writer.write_all(self.recipient_lock_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ETHBridgeTypeArgs::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct ETHBridgeTypeData(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ETHBridgeTypeData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ETHBridgeTypeData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ETHBridgeTypeData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "owner_lock_script", self.owner_lock_script())?;
        write!(f, ", {}: {}", "fee", self.fee())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for ETHBridgeTypeData {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            32, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        ETHBridgeTypeData::new_unchecked(v.into())
    }
}
impl ETHBridgeTypeData {
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn owner_lock_script(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn fee(&self) -> Uint128 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            Uint128::new_unchecked(self.0.slice(start..end))
        } else {
            Uint128::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> ETHBridgeTypeDataReader<'r> {
        ETHBridgeTypeDataReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ETHBridgeTypeData {
    type Builder = ETHBridgeTypeDataBuilder;
    const NAME: &'static str = "ETHBridgeTypeData";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ETHBridgeTypeData(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeDataReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeDataReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .owner_lock_script(self.owner_lock_script())
            .fee(self.fee())
    }
}
#[derive(Clone, Copy)]
pub struct ETHBridgeTypeDataReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ETHBridgeTypeDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ETHBridgeTypeDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ETHBridgeTypeDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "owner_lock_script", self.owner_lock_script())?;
        write!(f, ", {}: {}", "fee", self.fee())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> ETHBridgeTypeDataReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn owner_lock_script(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn fee(&self) -> Uint128Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            Uint128Reader::new_unchecked(&self.as_slice()[start..end])
        } else {
            Uint128Reader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ETHBridgeTypeDataReader<'r> {
    type Entity = ETHBridgeTypeData;
    const NAME: &'static str = "ETHBridgeTypeDataReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ETHBridgeTypeDataReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % 4 != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        let field_count = offset_first / 4 - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let header_size = molecule::NUMBER_SIZE * (field_count + 1);
        if slice_len < header_size {
            return ve!(Self, HeaderIsBroken, header_size, slice_len);
        }
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..]
            .chunks(molecule::NUMBER_SIZE)
            .take(field_count)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        BytesReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Uint128Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ETHBridgeTypeDataBuilder {
    pub(crate) owner_lock_script: Bytes,
    pub(crate) fee: Uint128,
}
impl ETHBridgeTypeDataBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub fn owner_lock_script(mut self, v: Bytes) -> Self {
        self.owner_lock_script = v;
        self
    }
    pub fn fee(mut self, v: Uint128) -> Self {
        self.fee = v;
        self
    }
}
impl molecule::prelude::Builder for ETHBridgeTypeDataBuilder {
    type Entity = ETHBridgeTypeData;
    const NAME: &'static str = "ETHBridgeTypeDataBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.owner_lock_script.as_slice().len()
            + self.fee.as_slice().len()
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.owner_lock_script.as_slice().len();
        offsets.push(total_size);
        total_size += self.fee.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.owner_lock_script.as_slice())?;
        writer.write_all(self.fee.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ETHBridgeTypeData::new_unchecked(inner.into())
    }
}
