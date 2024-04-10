// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum ClaimV1Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ClaimV1<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ClaimV1<'a> {
  type Inner = ClaimV1<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ClaimV1<'a> {
  pub const VT_EXECUTION_ID: flatbuffers::VOffsetT = 4;
  pub const VT_BLOCK_COMMITMENT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ClaimV1 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ClaimV1Args<'args>
  ) -> flatbuffers::WIPOffset<ClaimV1<'bldr>> {
    let mut builder = ClaimV1Builder::new(_fbb);
    builder.add_block_commitment(args.block_commitment);
    if let Some(x) = args.execution_id { builder.add_execution_id(x); }
    builder.finish()
  }


  #[inline]
  pub fn execution_id(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ClaimV1::VT_EXECUTION_ID, None)}
  }
  #[inline]
  pub fn block_commitment(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(ClaimV1::VT_BLOCK_COMMITMENT, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for ClaimV1<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("execution_id", Self::VT_EXECUTION_ID, false)?
     .visit_field::<u64>("block_commitment", Self::VT_BLOCK_COMMITMENT, false)?
     .finish();
    Ok(())
  }
}
pub struct ClaimV1Args<'a> {
    pub execution_id: Option<flatbuffers::WIPOffset<&'a str>>,
    pub block_commitment: u64,
}
impl<'a> Default for ClaimV1Args<'a> {
  #[inline]
  fn default() -> Self {
    ClaimV1Args {
      execution_id: None,
      block_commitment: 0,
    }
  }
}

pub struct ClaimV1Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ClaimV1Builder<'a, 'b> {
  #[inline]
  pub fn add_execution_id(&mut self, execution_id: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ClaimV1::VT_EXECUTION_ID, execution_id);
  }
  #[inline]
  pub fn add_block_commitment(&mut self, block_commitment: u64) {
    self.fbb_.push_slot::<u64>(ClaimV1::VT_BLOCK_COMMITMENT, block_commitment, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ClaimV1Builder<'a, 'b> {
    let start = _fbb.start_table();
    ClaimV1Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ClaimV1<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ClaimV1<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ClaimV1");
      ds.field("execution_id", &self.execution_id());
      ds.field("block_commitment", &self.block_commitment());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `ClaimV1`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_claim_v1_unchecked`.
pub fn root_as_claim_v1(buf: &[u8]) -> Result<ClaimV1, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ClaimV1>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ClaimV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_claim_v1_unchecked`.
pub fn size_prefixed_root_as_claim_v1(buf: &[u8]) -> Result<ClaimV1, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ClaimV1>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ClaimV1` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_claim_v1_unchecked`.
pub fn root_as_claim_v1_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ClaimV1<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ClaimV1<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ClaimV1` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_claim_v1_unchecked`.
pub fn size_prefixed_root_as_claim_v1_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ClaimV1<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ClaimV1<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ClaimV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ClaimV1`.
pub unsafe fn root_as_claim_v1_unchecked(buf: &[u8]) -> ClaimV1 {
  flatbuffers::root_unchecked::<ClaimV1>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ClaimV1 and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ClaimV1`.
pub unsafe fn size_prefixed_root_as_claim_v1_unchecked(buf: &[u8]) -> ClaimV1 {
  flatbuffers::size_prefixed_root_unchecked::<ClaimV1>(buf)
}
#[inline]
pub fn finish_claim_v1_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ClaimV1<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_claim_v1_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ClaimV1<'a>>) {
  fbb.finish_size_prefixed(root, None);
}