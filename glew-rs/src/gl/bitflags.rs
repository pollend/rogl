use std::fmt;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FfdMaskSGIX(pub(crate) std::os::raw::c_uint);
impl FfdMaskSGIX {
    pub const GL_TEXTURE_DEFORMATION_BIT_SGIX: Self = Self(0x0000_0001);
    pub const GL_GEOMETRY_DEFORMATION_BIT_SGIX: Self = Self(0x0000_0002);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClientAttribMask(pub(crate) std::os::raw::c_uint);
impl ClientAttribMask {
    pub const GL_CLIENT_VERTEX_ARRAY_BIT: Self = Self(0x0000_0002);
    pub const GL_CLIENT_PIXEL_STORE_BIT: Self = Self(0x0000_0001);
    pub const GL_CLIENT_ALL_ATTRIB_BITS: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathRenderingMaskNV(pub(crate) std::os::raw::c_uint);
impl PathRenderingMaskNV {
    pub const GL_FONT_ASCENDER_BIT_NV: Self = Self(0x0020_0000);
    pub const GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: Self = Self(0x04);
    pub const GL_FONT_X_MAX_BOUNDS_BIT_NV: Self = Self(0x0004_0000);
    pub const GL_FONT_HEIGHT_BIT_NV: Self = Self(0x0080_0000);
    pub const GL_ITALIC_BIT_NV: Self = Self(0x02);
    pub const GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: Self = Self(0x80);
    pub const GL_FONT_UNITS_PER_EM_BIT_NV: Self = Self(0x0010_0000);
    pub const GL_FONT_UNDERLINE_POSITION_BIT_NV: Self = Self(0x0400_0000);
    pub const GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: Self = Self(0x40);
    pub const GL_FONT_UNDERLINE_THICKNESS_BIT_NV: Self = Self(0x0800_0000);
    pub const GL_GLYPH_WIDTH_BIT_NV: Self = Self(0x01);
    pub const GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: Self = Self(0x10);
    pub const GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: Self = Self(0x08);
    pub const GL_FONT_X_MIN_BOUNDS_BIT_NV: Self = Self(0x0001_0000);
    pub const GL_GLYPH_HEIGHT_BIT_NV: Self = Self(0x02);
    pub const GL_BOLD_BIT_NV: Self = Self(0x01);
    pub const GL_FONT_Y_MAX_BOUNDS_BIT_NV: Self = Self(0x0008_0000);
    pub const GL_FONT_DESCENDER_BIT_NV: Self = Self(0x0040_0000);
    pub const GL_GLYPH_HAS_KERNING_BIT_NV: Self = Self(0x100);
    pub const GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: Self = Self(0x20);
    pub const GL_FONT_HAS_KERNING_BIT_NV: Self = Self(0x1000_0000);
    pub const GL_FONT_NUM_GLYPH_INDICES_BIT_NV: Self = Self(0x2000_0000);
    pub const GL_FONT_Y_MIN_BOUNDS_BIT_NV: Self = Self(0x0002_0000);
    pub const GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: Self = Self(0x0200_0000);
    pub const GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: Self = Self(0x0100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupSupportedFeatures(pub(crate) std::os::raw::c_uint);
impl SubgroupSupportedFeatures {
    pub const GL_SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: Self = Self(0x0000_0040);
    pub const GL_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: Self = Self(0x0000_0100);
    pub const GL_SUBGROUP_FEATURE_BASIC_BIT_KHR: Self = Self(0x0000_0001);
    pub const GL_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: Self = Self(0x0000_0020);
    pub const GL_SUBGROUP_FEATURE_QUAD_BIT_KHR: Self = Self(0x0000_0080);
    pub const GL_SUBGROUP_FEATURE_VOTE_BIT_KHR: Self = Self(0x0000_0002);
    pub const GL_SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: Self = Self(0x0000_0004);
    pub const GL_SUBGROUP_FEATURE_BALLOT_BIT_KHR: Self = Self(0x0000_0008);
    pub const GL_SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: Self = Self(0x0000_0010);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShaderDestModMaskATI(pub(crate) std::os::raw::c_uint);
impl FragmentShaderDestModMaskATI {
    pub const GL_HALF_BIT_ATI: Self = Self(0x0000_0008);
    pub const GL_QUARTER_BIT_ATI: Self = Self(0x0000_0010);
    pub const GL_EIGHTH_BIT_ATI: Self = Self(0x0000_0020);
    pub const GL_SATURATE_BIT_ATI: Self = Self(0x0000_0040);
    pub const GL_4X_BIT_ATI: Self = Self(0x0000_0002);
    pub const GL_8X_BIT_ATI: Self = Self(0x0000_0004);
    pub const GL_2X_BIT_ATI: Self = Self(0x0000_0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextProfileMask(pub(crate) std::os::raw::c_uint);
impl ContextProfileMask {
    pub const GL_CONTEXT_CORE_PROFILE_BIT: Self = Self(0x0000_0001);
    pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: Self = Self(0x0000_0002);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncObjectMask(pub(crate) std::os::raw::c_uint);
impl SyncObjectMask {
    pub const GL_SYNC_FLUSH_COMMANDS_BIT: Self = Self(0x0000_0001);
    pub const GL_SYNC_FLUSH_COMMANDS_BIT_APPLE: Self = Self(0x0000_0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttribMask(pub(crate) std::os::raw::c_uint);
impl AttribMask {
    pub const GL_LIGHTING_BIT: Self = Self(0x0000_0040);
    pub const GL_COLOR_BUFFER_BIT: Self = Self(0x0000_4000);
    pub const GL_HINT_BIT: Self = Self(0x0000_8000);
    pub const GL_TEXTURE_BIT: Self = Self(0x0004_0000);
    pub const GL_MULTISAMPLE_BIT_3DFX: Self = Self(0x2000_0000);
    pub const GL_MULTISAMPLE_BIT_EXT: Self = Self(0x2000_0000);
    pub const GL_EVAL_BIT: Self = Self(0x0001_0000);
    pub const GL_POLYGON_BIT: Self = Self(0x0000_0008);
    pub const GL_LINE_BIT: Self = Self(0x0000_0004);
    pub const GL_TRANSFORM_BIT: Self = Self(0x0000_1000);
    pub const GL_MULTISAMPLE_BIT_ARB: Self = Self(0x2000_0000);
    pub const GL_ENABLE_BIT: Self = Self(0x0000_2000);
    pub const GL_PIXEL_MODE_BIT: Self = Self(0x0000_0020);
    pub const GL_POINT_BIT: Self = Self(0x0000_0002);
    pub const GL_POLYGON_STIPPLE_BIT: Self = Self(0x0000_0010);
    pub const GL_SCISSOR_BIT: Self = Self(0x0008_0000);
    pub const GL_LIST_BIT: Self = Self(0x0002_0000);
    pub const GL_CURRENT_BIT: Self = Self(0x0000_0001);
    pub const GL_MULTISAMPLE_BIT: Self = Self(0x2000_0000);
    pub const GL_STENCIL_BUFFER_BIT: Self = Self(0x0000_0400);
    pub const GL_ALL_ATTRIB_BITS: Self = Self(0xffff_ffff);
    pub const GL_DEPTH_BUFFER_BIT: Self = Self(0x0000_0100);
    pub const GL_VIEWPORT_BIT: Self = Self(0x0000_0800);
    pub const GL_ACCUM_BUFFER_BIT: Self = Self(0x0000_0200);
    pub const GL_FOG_BIT: Self = Self(0x0000_0080);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UseProgramStageMask(pub(crate) std::os::raw::c_uint);
impl UseProgramStageMask {
    pub const GL_VERTEX_SHADER_BIT: Self = Self(0x0000_0001);
    pub const GL_GEOMETRY_SHADER_BIT_EXT: Self = Self(0x0000_0004);
    pub const GL_TESS_EVALUATION_SHADER_BIT: Self = Self(0x0000_0010);
    pub const GL_TESS_CONTROL_SHADER_BIT: Self = Self(0x0000_0008);
    pub const GL_TESS_CONTROL_SHADER_BIT_OES: Self = Self(0x0000_0008);
    pub const GL_FRAGMENT_SHADER_BIT_EXT: Self = Self(0x0000_0002);
    pub const GL_TESS_CONTROL_SHADER_BIT_EXT: Self = Self(0x0000_0008);
    pub const GL_MESH_SHADER_BIT_NV: Self = Self(0x0000_0040);
    pub const GL_TESS_EVALUATION_SHADER_BIT_EXT: Self = Self(0x0000_0010);
    pub const GL_GEOMETRY_SHADER_BIT_OES: Self = Self(0x0000_0004);
    pub const GL_COMPUTE_SHADER_BIT: Self = Self(0x0000_0020);
    pub const GL_ALL_SHADER_BITS: Self = Self(0xffff_ffff);
    pub const GL_TESS_EVALUATION_SHADER_BIT_OES: Self = Self(0x0000_0010);
    pub const GL_ALL_SHADER_BITS_EXT: Self = Self(0xffff_ffff);
    pub const GL_VERTEX_SHADER_BIT_EXT: Self = Self(0x0000_0001);
    pub const GL_FRAGMENT_SHADER_BIT: Self = Self(0x0000_0002);
    pub const GL_TASK_SHADER_BIT_NV: Self = Self(0x0000_0080);
    pub const GL_GEOMETRY_SHADER_BIT: Self = Self(0x0000_0004);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryBarrierMask(pub(crate) std::os::raw::c_uint);
impl MemoryBarrierMask {
    pub const GL_ALL_BARRIER_BITS_EXT: Self = Self(0xffff_ffff);
    pub const GL_COMMAND_BARRIER_BIT_EXT: Self = Self(0x0000_0040);
    pub const GL_QUERY_BUFFER_BARRIER_BIT: Self = Self(0x0000_8000);
    pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT_EXT: Self = Self(0x0000_0020);
    pub const GL_PIXEL_BUFFER_BARRIER_BIT: Self = Self(0x0000_0080);
    pub const GL_TEXTURE_FETCH_BARRIER_BIT: Self = Self(0x0000_0008);
    pub const GL_UNIFORM_BARRIER_BIT: Self = Self(0x0000_0004);
    pub const GL_TEXTURE_FETCH_BARRIER_BIT_EXT: Self = Self(0x0000_0008);
    pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: Self = Self(0x0000_0001);
    pub const GL_SHADER_GLOBAL_ACCESS_BARRIER_BIT_NV: Self = Self(0x0000_0010);
    pub const GL_UNIFORM_BARRIER_BIT_EXT: Self = Self(0x0000_0004);
    pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: Self = Self(0x0000_0800);
    pub const GL_BUFFER_UPDATE_BARRIER_BIT_EXT: Self = Self(0x0000_0200);
    pub const GL_ATOMIC_COUNTER_BARRIER_BIT_EXT: Self = Self(0x0000_1000);
    pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: Self = Self(0x0000_0020);
    pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: Self = Self(0x0000_4000);
    pub const GL_FRAMEBUFFER_BARRIER_BIT: Self = Self(0x0000_0400);
    pub const GL_TEXTURE_UPDATE_BARRIER_BIT: Self = Self(0x0000_0100);
    pub const GL_ELEMENT_ARRAY_BARRIER_BIT: Self = Self(0x0000_0002);
    pub const GL_PIXEL_BUFFER_BARRIER_BIT_EXT: Self = Self(0x0000_0080);
    pub const GL_ALL_BARRIER_BITS: Self = Self(0xffff_ffff);
    pub const GL_COMMAND_BARRIER_BIT: Self = Self(0x0000_0040);
    pub const GL_ATOMIC_COUNTER_BARRIER_BIT: Self = Self(0x0000_1000);
    pub const GL_ELEMENT_ARRAY_BARRIER_BIT_EXT: Self = Self(0x0000_0002);
    pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT_EXT: Self = Self(0x0000_0001);
    pub const GL_FRAMEBUFFER_BARRIER_BIT_EXT: Self = Self(0x0000_0400);
    pub const GL_TEXTURE_UPDATE_BARRIER_BIT_EXT: Self = Self(0x0000_0100);
    pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT_EXT: Self = Self(0x0000_0800);
    pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: Self = Self(0x0000_4000);
    pub const GL_SHADER_STORAGE_BARRIER_BIT: Self = Self(0x0000_2000);
    pub const GL_BUFFER_UPDATE_BARRIER_BIT: Self = Self(0x0000_0200);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureStorageMaskAMD(pub(crate) std::os::raw::c_uint);
impl TextureStorageMaskAMD {
    pub const GL_TEXTURE_STORAGE_SPARSE_BIT_AMD: Self = Self(0x0000_0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextFlagMask(pub(crate) std::os::raw::c_uint);
impl ContextFlagMask {
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: Self = Self(0x0000_0004);
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB: Self = Self(0x0000_0004);
    pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: Self = Self(0x0000_0008);
    pub const GL_CONTEXT_FLAG_PROTECTED_CONTENT_BIT_EXT: Self = Self(0x0000_0010);
    pub const GL_CONTEXT_FLAG_NO_ERROR_BIT_KHR: Self = Self(0x0000_0008);
    pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: Self = Self(0x0000_0002);
    pub const GL_CONTEXT_FLAG_DEBUG_BIT: Self = Self(0x0000_0002);
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: Self = Self(0x0000_0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraceMaskMESA(pub(crate) std::os::raw::c_uint);
impl TraceMaskMESA {
    pub const GL_TRACE_PIXELS_BIT_MESA: Self = Self(0x0010);
    pub const GL_TRACE_ERRORS_BIT_MESA: Self = Self(0x0020);
    pub const GL_TRACE_ALL_BITS_MESA: Self = Self(0xffff);
    pub const GL_TRACE_OPERATIONS_BIT_MESA: Self = Self(0x0001);
    pub const GL_TRACE_ARRAYS_BIT_MESA: Self = Self(0x0004);
    pub const GL_TRACE_TEXTURES_BIT_MESA: Self = Self(0x0008);
    pub const GL_TRACE_PRIMITIVES_BIT_MESA: Self = Self(0x0002);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShaderColorModMaskATI(pub(crate) std::os::raw::c_uint);
impl FragmentShaderColorModMaskATI {
    pub const GL_NEGATE_BIT_ATI: Self = Self(0x0000_0004);
    pub const GL_BIAS_BIT_ATI: Self = Self(0x0000_0008);
    pub const GL_COMP_BIT_ATI: Self = Self(0x0000_0002);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFontStyle(pub(crate) std::os::raw::c_uint);
impl PathFontStyle {
    pub const GL_ITALIC_BIT_NV: Self = Self(0x02);
    pub const GL_NONE: Self = Self(0);
    pub const GL_BOLD_BIT_NV: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapBufferAccessMask(pub(crate) std::os::raw::c_uint);
impl MapBufferAccessMask {
    pub const GL_MAP_INVALIDATE_RANGE_BIT_EXT: Self = Self(0x0004);
    pub const GL_MAP_INVALIDATE_BUFFER_BIT_EXT: Self = Self(0x0008);
    pub const GL_MAP_COHERENT_BIT_EXT: Self = Self(0x0080);
    pub const GL_MAP_PERSISTENT_BIT_EXT: Self = Self(0x0040);
    pub const GL_MAP_INVALIDATE_BUFFER_BIT: Self = Self(0x0008);
    pub const GL_MAP_READ_BIT_EXT: Self = Self(0x0001);
    pub const GL_MAP_READ_BIT: Self = Self(0x0001);
    pub const GL_MAP_UNSYNCHRONIZED_BIT: Self = Self(0x0020);
    pub const GL_MAP_WRITE_BIT: Self = Self(0x0002);
    pub const GL_MAP_PERSISTENT_BIT: Self = Self(0x0040);
    pub const GL_MAP_COHERENT_BIT: Self = Self(0x0080);
    pub const GL_MAP_WRITE_BIT_EXT: Self = Self(0x0002);
    pub const GL_MAP_INVALIDATE_RANGE_BIT: Self = Self(0x0004);
    pub const GL_MAP_FLUSH_EXPLICIT_BIT_EXT: Self = Self(0x0010);
    pub const GL_MAP_UNSYNCHRONIZED_BIT_EXT: Self = Self(0x0020);
    pub const GL_MAP_FLUSH_EXPLICIT_BIT: Self = Self(0x0010);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexHintsMaskPGI(pub(crate) std::os::raw::c_uint);
impl VertexHintsMaskPGI {
    pub const GL_MAT_AMBIENT_AND_DIFFUSE_BIT_PGI: Self = Self(0x0020_0000);
    pub const GL_COLOR3_BIT_PGI: Self = Self(0x0001_0000);
    pub const GL_MAT_COLOR_INDEXES_BIT_PGI: Self = Self(0x0100_0000);
    pub const GL_TEXCOORD3_BIT_PGI: Self = Self(0x4000_0000);
    pub const GL_TEXCOORD2_BIT_PGI: Self = Self(0x2000_0000);
    pub const GL_MAT_SPECULAR_BIT_PGI: Self = Self(0x0400_0000);
    pub const GL_TEXCOORD1_BIT_PGI: Self = Self(0x1000_0000);
    pub const GL_MAT_DIFFUSE_BIT_PGI: Self = Self(0x0040_0000);
    pub const GL_VERTEX23_BIT_PGI: Self = Self(0x0000_0004);
    pub const GL_EDGEFLAG_BIT_PGI: Self = Self(0x0004_0000);
    pub const GL_TEXCOORD4_BIT_PGI: Self = Self(0x8000_0000);
    pub const GL_MAT_SHININESS_BIT_PGI: Self = Self(0x0200_0000);
    pub const GL_INDEX_BIT_PGI: Self = Self(0x0008_0000);
    pub const GL_MAT_AMBIENT_BIT_PGI: Self = Self(0x0010_0000);
    pub const GL_COLOR4_BIT_PGI: Self = Self(0x0002_0000);
    pub const GL_MAT_EMISSION_BIT_PGI: Self = Self(0x0080_0000);
    pub const GL_VERTEX4_BIT_PGI: Self = Self(0x0000_0008);
    pub const GL_NORMAL_BIT_PGI: Self = Self(0x0800_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferBitQCOM(pub(crate) std::os::raw::c_uint);
impl BufferBitQCOM {
    pub const GL_DEPTH_BUFFER_BIT5_QCOM: Self = Self(0x0000_2000);
    pub const GL_STENCIL_BUFFER_BIT6_QCOM: Self = Self(0x0040_0000);
    pub const GL_DEPTH_BUFFER_BIT7_QCOM: Self = Self(0x0000_8000);
    pub const GL_STENCIL_BUFFER_BIT5_QCOM: Self = Self(0x0020_0000);
    pub const GL_COLOR_BUFFER_BIT2_QCOM: Self = Self(0x0000_0004);
    pub const GL_MULTISAMPLE_BUFFER_BIT6_QCOM: Self = Self(0x4000_0000);
    pub const GL_COLOR_BUFFER_BIT5_QCOM: Self = Self(0x0000_0020);
    pub const GL_COLOR_BUFFER_BIT1_QCOM: Self = Self(0x0000_0002);
    pub const GL_COLOR_BUFFER_BIT0_QCOM: Self = Self(0x0000_0001);
    pub const GL_STENCIL_BUFFER_BIT3_QCOM: Self = Self(0x0008_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT7_QCOM: Self = Self(0x8000_0000);
    pub const GL_DEPTH_BUFFER_BIT3_QCOM: Self = Self(0x0000_0800);
    pub const GL_DEPTH_BUFFER_BIT1_QCOM: Self = Self(0x0000_0200);
    pub const GL_MULTISAMPLE_BUFFER_BIT0_QCOM: Self = Self(0x0100_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT4_QCOM: Self = Self(0x1000_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT3_QCOM: Self = Self(0x0800_0000);
    pub const GL_STENCIL_BUFFER_BIT2_QCOM: Self = Self(0x0004_0000);
    pub const GL_COLOR_BUFFER_BIT3_QCOM: Self = Self(0x0000_0008);
    pub const GL_DEPTH_BUFFER_BIT0_QCOM: Self = Self(0x0000_0100);
    pub const GL_COLOR_BUFFER_BIT4_QCOM: Self = Self(0x0000_0010);
    pub const GL_COLOR_BUFFER_BIT6_QCOM: Self = Self(0x0000_0040);
    pub const GL_DEPTH_BUFFER_BIT6_QCOM: Self = Self(0x0000_4000);
    pub const GL_STENCIL_BUFFER_BIT4_QCOM: Self = Self(0x0010_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT1_QCOM: Self = Self(0x0200_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT5_QCOM: Self = Self(0x2000_0000);
    pub const GL_COLOR_BUFFER_BIT7_QCOM: Self = Self(0x0000_0080);
    pub const GL_STENCIL_BUFFER_BIT1_QCOM: Self = Self(0x0002_0000);
    pub const GL_STENCIL_BUFFER_BIT0_QCOM: Self = Self(0x0001_0000);
    pub const GL_STENCIL_BUFFER_BIT7_QCOM: Self = Self(0x0080_0000);
    pub const GL_MULTISAMPLE_BUFFER_BIT2_QCOM: Self = Self(0x0400_0000);
    pub const GL_DEPTH_BUFFER_BIT2_QCOM: Self = Self(0x0000_0400);
    pub const GL_DEPTH_BUFFER_BIT4_QCOM: Self = Self(0x0000_1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathMetricMask(pub(crate) std::os::raw::c_uint);
impl PathMetricMask {
    pub const GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: Self = Self(0x08);
    pub const GL_FONT_X_MAX_BOUNDS_BIT_NV: Self = Self(0x0004_0000);
    pub const GL_FONT_HAS_KERNING_BIT_NV: Self = Self(0x1000_0000);
    pub const GL_FONT_Y_MIN_BOUNDS_BIT_NV: Self = Self(0x0002_0000);
    pub const GL_GLYPH_HAS_KERNING_BIT_NV: Self = Self(0x100);
    pub const GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: Self = Self(0x80);
    pub const GL_GLYPH_WIDTH_BIT_NV: Self = Self(0x01);
    pub const GL_FONT_ASCENDER_BIT_NV: Self = Self(0x0020_0000);
    pub const GL_GLYPH_HEIGHT_BIT_NV: Self = Self(0x02);
    pub const GL_FONT_DESCENDER_BIT_NV: Self = Self(0x0040_0000);
    pub const GL_FONT_HEIGHT_BIT_NV: Self = Self(0x0080_0000);
    pub const GL_FONT_UNDERLINE_POSITION_BIT_NV: Self = Self(0x0400_0000);
    pub const GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: Self = Self(0x40);
    pub const GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: Self = Self(0x10);
    pub const GL_FONT_X_MIN_BOUNDS_BIT_NV: Self = Self(0x0001_0000);
    pub const GL_FONT_Y_MAX_BOUNDS_BIT_NV: Self = Self(0x0008_0000);
    pub const GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: Self = Self(0x0200_0000);
    pub const GL_FONT_UNITS_PER_EM_BIT_NV: Self = Self(0x0010_0000);
    pub const GL_FONT_UNDERLINE_THICKNESS_BIT_NV: Self = Self(0x0800_0000);
    pub const GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: Self = Self(0x04);
    pub const GL_FONT_NUM_GLYPH_INDICES_BIT_NV: Self = Self(0x2000_0000);
    pub const GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: Self = Self(0x20);
    pub const GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: Self = Self(0x0100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferStorageMask(pub(crate) std::os::raw::c_uint);
impl BufferStorageMask {
    pub const GL_DYNAMIC_STORAGE_BIT_EXT: Self = Self(0x0100);
    pub const GL_SPARSE_STORAGE_BIT_ARB: Self = Self(0x0400);
    pub const GL_MAP_WRITE_BIT_EXT: Self = Self(0x0002);
    pub const GL_PER_GPU_STORAGE_BIT_NV: Self = Self(0x0800);
    pub const GL_DYNAMIC_STORAGE_BIT: Self = Self(0x0100);
    pub const GL_CLIENT_STORAGE_BIT_EXT: Self = Self(0x0200);
    pub const GL_EXTERNAL_STORAGE_BIT_NVX: Self = Self(0x2000);
    pub const GL_MAP_PERSISTENT_BIT_EXT: Self = Self(0x0040);
    pub const GL_MAP_COHERENT_BIT_EXT: Self = Self(0x0080);
    pub const GL_LGPU_SEPARATE_STORAGE_BIT_NVX: Self = Self(0x0800);
    pub const GL_CLIENT_STORAGE_BIT: Self = Self(0x0200);
    pub const GL_MAP_COHERENT_BIT: Self = Self(0x0080);
    pub const GL_MAP_READ_BIT: Self = Self(0x0001);
    pub const GL_MAP_WRITE_BIT: Self = Self(0x0002);
    pub const GL_MAP_PERSISTENT_BIT: Self = Self(0x0040);
    pub const GL_MAP_READ_BIT_EXT: Self = Self(0x0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShaderDestMaskATI(pub(crate) std::os::raw::c_uint);
impl FragmentShaderDestMaskATI {
    pub const GL_RED_BIT_ATI: Self = Self(0x0000_0001);
    pub const GL_GREEN_BIT_ATI: Self = Self(0x0000_0002);
    pub const GL_BLUE_BIT_ATI: Self = Self(0x0000_0004);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceQueryCapsMaskINTEL(pub(crate) std::os::raw::c_uint);
impl PerformanceQueryCapsMaskINTEL {
    pub const GL_PERFQUERY_GLOBAL_CONTEXT_INTEL: Self = Self(0x0000_0001);
    pub const GL_PERFQUERY_SINGLE_CONTEXT_INTEL: Self = Self(0x0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoveationConfigBitQCOM(pub(crate) std::os::raw::c_uint);
impl FoveationConfigBitQCOM {
    pub const GL_FOVEATION_SCALED_BIN_METHOD_BIT_QCOM: Self = Self(0x0000_0002);
    pub const GL_FOVEATION_SUBSAMPLED_LAYOUT_METHOD_BIT_QCOM: Self = Self(0x0000_0004);
    pub const GL_FOVEATION_ENABLE_BIT_QCOM: Self = Self(0x0000_0001);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcclusionQueryEventMaskAMD(pub(crate) std::os::raw::c_uint);
impl OcclusionQueryEventMaskAMD {
    pub const GL_QUERY_DEPTH_FAIL_EVENT_BIT_AMD: Self = Self(0x0000_0002);
    pub const GL_QUERY_STENCIL_FAIL_EVENT_BIT_AMD: Self = Self(0x0000_0004);
    pub const GL_QUERY_DEPTH_BOUNDS_FAIL_EVENT_BIT_AMD: Self = Self(0x0000_0008);
    pub const GL_QUERY_DEPTH_PASS_EVENT_BIT_AMD: Self = Self(0x0000_0001);
    pub const GL_QUERY_ALL_EVENT_BITS_AMD: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClearBufferMask(pub(crate) std::os::raw::c_uint);
impl ClearBufferMask {
    pub const GL_COLOR_BUFFER_BIT: Self = Self(0x0000_4000);
    pub const GL_STENCIL_BUFFER_BIT: Self = Self(0x0000_0400);
    pub const GL_COVERAGE_BUFFER_BIT_NV: Self = Self(0x0000_8000);
    pub const GL_DEPTH_BUFFER_BIT: Self = Self(0x0000_0100);
    pub const GL_ACCUM_BUFFER_BIT: Self = Self(0x0000_0200);
}
