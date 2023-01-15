use super::load;
use std::ffi::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

type DebugProc = Option<
    extern "system" fn(
        source: c_uint,
        gltype: c_uint,
        id: c_uint,
        severity: c_uint,
        length: isize,
        message: *const c_char,
        userParam: *mut c_void,
    ),
>;

type ActiveShaderProgramT = extern "system" fn(c_uint, c_uint);
type ActiveTextureT = extern "system" fn(c_uint);
type AlphaFuncT = extern "system" fn(c_uint, c_float);
type AlphaFuncxT = extern "system" fn(c_uint, c_int);
type AttachShaderT = extern "system" fn(c_uint, c_uint);
type BeginConditionalRenderT = extern "system" fn(c_uint, c_uint);
type BeginQueryT = extern "system" fn(c_uint, c_uint);
type BeginTransformFeedbackT = extern "system" fn(c_uint);
type BindAttribLocationT = extern "system" fn(c_uint, c_uint, *const c_char);
type BindBufferT = extern "system" fn(c_uint, c_uint);
type BindBufferBaseT = extern "system" fn(c_uint, c_uint, c_uint);
type BindBufferRangeT = extern "system" fn(c_uint, c_uint, c_uint, isize, isize);
type BindFragDataLocationT = extern "system" fn(c_uint, c_uint, *const c_char);
type BindFragDataLocationIndexedT = extern "system" fn(c_uint, c_uint, c_uint, *const c_char);
type BindFramebufferT = extern "system" fn(c_uint, c_uint);
type BindImageTextureT = extern "system" fn(c_uint, c_uint, c_int, c_uchar, c_int, c_uint, c_uint);
type BindProgramPipelineT = extern "system" fn(c_uint);
type BindRenderbufferT = extern "system" fn(c_uint, c_uint);
type BindSamplerT = extern "system" fn(c_uint, c_uint);
type BindTextureT = extern "system" fn(c_uint, c_uint);
type BindTransformFeedbackT = extern "system" fn(c_uint, c_uint);
type BindVertexArrayT = extern "system" fn(c_uint);
type BindVertexBufferT = extern "system" fn(c_uint, c_uint, isize, c_int);
type BlendBarrierT = extern "system" fn();
type BlendColorT = extern "system" fn(c_float, c_float, c_float, c_float);
type BlendEquationT = extern "system" fn(c_uint);
type BlendEquationSeparateT = extern "system" fn(c_uint, c_uint);
type BlendEquationSeparateiT = extern "system" fn(c_uint, c_uint, c_uint);
type BlendEquationiT = extern "system" fn(c_uint, c_uint);
type BlendFuncT = extern "system" fn(c_uint, c_uint);
type BlendFuncSeparateT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type BlendFuncSeparateiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_uint);
type BlendFunciT = extern "system" fn(c_uint, c_uint, c_uint);
type BlitFramebufferT =
    extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint);
type BufferDataT = extern "system" fn(c_uint, isize, *const c_void, c_uint);
type BufferSubDataT = extern "system" fn(c_uint, isize, isize, *const c_void);
type CheckFramebufferStatusT = extern "system" fn(c_uint) -> c_uint;
type ClampColorT = extern "system" fn(c_uint, c_uint);
type ClearT = extern "system" fn(c_uint);
type ClearBufferfiT = extern "system" fn(c_uint, c_int, c_float, c_int);
type ClearBufferfvT = extern "system" fn(c_uint, c_int, *const c_float);
type ClearBufferivT = extern "system" fn(c_uint, c_int, *const c_int);
type ClearBufferuivT = extern "system" fn(c_uint, c_int, *const c_uint);
type ClearColorT = extern "system" fn(c_float, c_float, c_float, c_float);
type ClearColorxT = extern "system" fn(c_int, c_int, c_int, c_int);
type ClearDepthT = extern "system" fn(c_double);
type ClearDepthfT = extern "system" fn(c_float);
type ClearDepthxT = extern "system" fn(c_int);
type ClearStencilT = extern "system" fn(c_int);
type ClientActiveTextureT = extern "system" fn(c_uint);
type ClientWaitSyncT = extern "system" fn(*mut c_void, c_uint, u64) -> c_uint;
type ClipPlanefT = extern "system" fn(c_uint, *const c_float);
type ClipPlanexT = extern "system" fn(c_uint, *const c_int);
type Color4fT = extern "system" fn(c_float, c_float, c_float, c_float);
type Color4ubT = extern "system" fn(c_uchar, c_uchar, c_uchar, c_uchar);
type Color4xT = extern "system" fn(c_int, c_int, c_int, c_int);
type ColorMaskT = extern "system" fn(c_uchar, c_uchar, c_uchar, c_uchar);
type ColorMaskiT = extern "system" fn(c_uint, c_uchar, c_uchar, c_uchar, c_uchar);
type ColorP3uiT = extern "system" fn(c_uint, c_uint);
type ColorP3uivT = extern "system" fn(c_uint, *const c_uint);
type ColorP4uiT = extern "system" fn(c_uint, c_uint);
type ColorP4uivT = extern "system" fn(c_uint, *const c_uint);
type ColorPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type CompileShaderT = extern "system" fn(c_uint);
type CompressedTexImage1DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, *const c_void);
type CompressedTexImage2DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, *const c_void);
type CompressedTexImage3DT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, c_int, *const c_void);
type CompressedTexSubImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_uint, c_int, *const c_void);
type CompressedTexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_int, *const c_void);
type CompressedTexSubImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_int,
    *const c_void,
);
type CopyBufferSubDataT = extern "system" fn(c_uint, c_uint, isize, isize, isize);
type CopyImageSubDataT = extern "system" fn(
    c_uint,
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
);
type CopyTexImage1DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int);
type CopyTexImage2DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage1DT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
type CopyTexSubImage3DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
type CreateProgramT = extern "system" fn() -> c_uint;
type CreateShaderT = extern "system" fn(c_uint) -> c_uint;
type CreateShaderProgramvT = extern "system" fn(c_uint, c_int, *const *const c_char) -> c_uint;
type CullFaceT = extern "system" fn(c_uint);
type DebugMessageCallbackT = extern "system" fn(DebugProc, *const c_void);
type DebugMessageControlT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *const c_uint, c_uchar);
type DebugMessageInsertT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int, *const c_char);
type DeleteBuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteFramebuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteProgramT = extern "system" fn(c_uint);
type DeleteProgramPipelinesT = extern "system" fn(c_int, *const c_uint);
type DeleteQueriesT = extern "system" fn(c_int, *const c_uint);
type DeleteRenderbuffersT = extern "system" fn(c_int, *const c_uint);
type DeleteSamplersT = extern "system" fn(c_int, *const c_uint);
type DeleteShaderT = extern "system" fn(c_uint);
type DeleteSyncT = extern "system" fn(*mut c_void);
type DeleteTexturesT = extern "system" fn(c_int, *const c_uint);
type DeleteTransformFeedbacksT = extern "system" fn(c_int, *const c_uint);
type DeleteVertexArraysT = extern "system" fn(c_int, *const c_uint);
type DepthFuncT = extern "system" fn(c_uint);
type DepthMaskT = extern "system" fn(c_uchar);
type DepthRangeT = extern "system" fn(c_double, c_double);
type DepthRangefT = extern "system" fn(c_float, c_float);
type DepthRangexT = extern "system" fn(c_int, c_int);
type DetachShaderT = extern "system" fn(c_uint, c_uint);
type DisableT = extern "system" fn(c_uint);
type DisableClientStateT = extern "system" fn(c_uint);
type DisableVertexAttribArrayT = extern "system" fn(c_uint);
type DisableiT = extern "system" fn(c_uint, c_uint);
type DispatchComputeT = extern "system" fn(c_uint, c_uint, c_uint);
type DispatchComputeIndirectT = extern "system" fn(isize);
type DrawArraysT = extern "system" fn(c_uint, c_int, c_int);
type DrawArraysIndirectT = extern "system" fn(c_uint, *const c_void);
type DrawArraysInstancedT = extern "system" fn(c_uint, c_int, c_int, c_int);
type DrawBufferT = extern "system" fn(c_uint);
type DrawBuffersT = extern "system" fn(c_int, *const c_uint);
type DrawElementsT = extern "system" fn(c_uint, c_int, c_uint, *const c_void);
type DrawElementsBaseVertexT = extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int);
type DrawElementsIndirectT = extern "system" fn(c_uint, c_uint, *const c_void);
type DrawElementsInstancedT = extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int);
type DrawElementsInstancedBaseVertexT =
    extern "system" fn(c_uint, c_int, c_uint, *const c_void, c_int, c_int);
type DrawRangeElementsT = extern "system" fn(c_uint, c_uint, c_uint, c_int, c_uint, *const c_void);
type DrawRangeElementsBaseVertexT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, c_uint, *const c_void, c_int);
type EnableT = extern "system" fn(c_uint);
type EnableClientStateT = extern "system" fn(c_uint);
type EnableVertexAttribArrayT = extern "system" fn(c_uint);
type EnableiT = extern "system" fn(c_uint, c_uint);
type EndConditionalRenderT = extern "system" fn();
type EndQueryT = extern "system" fn(c_uint);
type EndTransformFeedbackT = extern "system" fn();
type FenceSyncT = extern "system" fn(c_uint, c_uint) -> *mut c_void;
type FinishT = extern "system" fn();
type FlushT = extern "system" fn();
type FlushMappedBufferRangeT = extern "system" fn(c_uint, isize, isize);
type FogfT = extern "system" fn(c_uint, c_float);
type FogfvT = extern "system" fn(c_uint, *const c_float);
type FogxT = extern "system" fn(c_uint, c_int);
type FogxvT = extern "system" fn(c_uint, *const c_int);
type FramebufferParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type FramebufferRenderbufferT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type FramebufferTextureT = extern "system" fn(c_uint, c_uint, c_uint, c_int);
type FramebufferTexture1DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int);
type FramebufferTexture2DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int);
type FramebufferTexture3DT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_int, c_int);
type FramebufferTextureLayerT = extern "system" fn(c_uint, c_uint, c_uint, c_int, c_int);
type FrontFaceT = extern "system" fn(c_uint);
type FrustumfT = extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float);
type FrustumxT = extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int);
type GenBuffersT = extern "system" fn(c_int, *mut c_uint);
type GenFramebuffersT = extern "system" fn(c_int, *mut c_uint);
type GenProgramPipelinesT = extern "system" fn(c_int, *mut c_uint);
type GenQueriesT = extern "system" fn(c_int, *mut c_uint);
type GenRenderbuffersT = extern "system" fn(c_int, *mut c_uint);
type GenSamplersT = extern "system" fn(c_int, *mut c_uint);
type GenTexturesT = extern "system" fn(c_int, *mut c_uint);
type GenTransformFeedbacksT = extern "system" fn(c_int, *mut c_uint);
type GenVertexArraysT = extern "system" fn(c_int, *mut c_uint);
type GenerateMipmapT = extern "system" fn(c_uint);
type GetActiveAttribT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetActiveUniformT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetActiveUniformBlockNameT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetActiveUniformBlockivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetActiveUniformNameT = extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetActiveUniformsivT = extern "system" fn(c_uint, c_int, *const c_uint, c_uint, *mut c_int);
type GetAttachedShadersT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_uint);
type GetAttribLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetBooleanivT = extern "system" fn(c_uint, c_uint, *mut c_uchar);
type GetBooleanvT = extern "system" fn(c_uint, *mut c_uchar);
type GetBufferParameteri64vT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetBufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetBufferPointervT = extern "system" fn(c_uint, c_uint, *mut *mut c_void);
type GetBufferSubDataT = extern "system" fn(c_uint, isize, isize, *mut c_void);
type GetClipPlanefT = extern "system" fn(c_uint, *mut c_float);
type GetClipPlanexT = extern "system" fn(c_uint, *mut c_int);
type GetCompressedTexImageT = extern "system" fn(c_uint, c_int, *mut c_void);
type GetDebugMessageLogT = extern "system" fn(
    c_uint,
    c_int,
    *mut c_uint,
    *mut c_uint,
    *mut c_uint,
    *mut c_uint,
    *mut c_int,
    *mut c_char,
) -> c_uint;
type GetDoublevT = extern "system" fn(c_uint, *mut c_double);
type GetErrorT = extern "system" fn() -> c_uint;
type GetFixedvT = extern "system" fn(c_uint, *mut c_int);
type GetFloatvT = extern "system" fn(c_uint, *mut c_float);
type GetFragDataIndexT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetFragDataLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetFramebufferAttachmentParameterivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetFramebufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetGraphicsResetStatusT = extern "system" fn() -> c_uint;
type GetInteger64ivT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetInteger64vT = extern "system" fn(c_uint, *mut i64);
type GetIntegerivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetIntegervT = extern "system" fn(c_uint, *mut c_int);
type GetInternalformativT = extern "system" fn(c_uint, c_uint, c_uint, c_int, *mut c_int);
type GetLightfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetLightxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetMaterialfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetMaterialxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetMultisamplefvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetObjectLabelT = extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetObjectPtrLabelT = extern "system" fn(*const c_void, c_int, *mut c_int, *mut c_char);
type GetPointervT = extern "system" fn(c_uint, *mut *mut c_void);
type GetProgramBinaryT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_uint, *mut c_void);
type GetProgramInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramInterfaceivT = extern "system" fn(c_uint, c_uint, c_uint, *mut c_int);
type GetProgramPipelineInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramPipelineivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetProgramResourceIndexT = extern "system" fn(c_uint, c_uint, *const c_char) -> c_uint;
type GetProgramResourceLocationT = extern "system" fn(c_uint, c_uint, *const c_char) -> c_int;
type GetProgramResourceNameT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *mut c_int, *mut c_char);
type GetProgramResourceivT =
    extern "system" fn(c_uint, c_uint, c_uint, c_int, *const c_uint, c_int, *mut c_int, *mut c_int);
type GetProgramivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetQueryObjecti64vT = extern "system" fn(c_uint, c_uint, *mut i64);
type GetQueryObjectivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetQueryObjectui64vT = extern "system" fn(c_uint, c_uint, *mut u64);
type GetQueryObjectuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetQueryivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetRenderbufferParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetSamplerParameterIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetSamplerParameterIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetSamplerParameterfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetSamplerParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetShaderInfoLogT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetShaderPrecisionFormatT = extern "system" fn(c_uint, c_uint, *mut c_int, *mut c_int);
type GetShaderSourceT = extern "system" fn(c_uint, c_int, *mut c_int, *mut c_char);
type GetShaderivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetStringT = extern "system" fn(c_uint) -> *const c_uchar;
type GetStringiT = extern "system" fn(c_uint, c_uint) -> *const c_uchar;
type GetSyncivT = extern "system" fn(*mut c_void, c_uint, c_int, *mut c_int, *mut c_int);
type GetTexEnvfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetTexEnvivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexEnvxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexImageT = extern "system" fn(c_uint, c_int, c_uint, c_uint, *mut c_void);
type GetTexLevelParameterfvT = extern "system" fn(c_uint, c_int, c_uint, *mut c_float);
type GetTexLevelParameterivT = extern "system" fn(c_uint, c_int, c_uint, *mut c_int);
type GetTexParameterIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexParameterIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetTexParameterfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetTexParameterivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTexParameterxvT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetTransformFeedbackVaryingT =
    extern "system" fn(c_uint, c_uint, c_int, *mut c_int, *mut c_int, *mut c_uint, *mut c_char);
type GetUniformBlockIndexT = extern "system" fn(c_uint, *const c_char) -> c_uint;
type GetUniformIndicesT = extern "system" fn(c_uint, c_int, *const *const c_char, *mut c_uint);
type GetUniformLocationT = extern "system" fn(c_uint, *const c_char) -> c_int;
type GetUniformfvT = extern "system" fn(c_uint, c_int, *mut c_float);
type GetUniformivT = extern "system" fn(c_uint, c_int, *mut c_int);
type GetUniformuivT = extern "system" fn(c_uint, c_int, *mut c_uint);
type GetVertexAttribIivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetVertexAttribIuivT = extern "system" fn(c_uint, c_uint, *mut c_uint);
type GetVertexAttribPointervT = extern "system" fn(c_uint, c_uint, *mut *mut c_void);
type GetVertexAttribdvT = extern "system" fn(c_uint, c_uint, *mut c_double);
type GetVertexAttribfvT = extern "system" fn(c_uint, c_uint, *mut c_float);
type GetVertexAttribivT = extern "system" fn(c_uint, c_uint, *mut c_int);
type GetnUniformfvT = extern "system" fn(c_uint, c_int, c_int, *mut c_float);
type GetnUniformivT = extern "system" fn(c_uint, c_int, c_int, *mut c_int);
type GetnUniformuivT = extern "system" fn(c_uint, c_int, c_int, *mut c_uint);
type HintT = extern "system" fn(c_uint, c_uint);
type InvalidateFramebufferT = extern "system" fn(c_uint, c_int, *const c_uint);
type InvalidateSubFramebufferT =
    extern "system" fn(c_uint, c_int, *const c_uint, c_int, c_int, c_int, c_int);
type IsBufferT = extern "system" fn(c_uint) -> c_uchar;
type IsEnabledT = extern "system" fn(c_uint) -> c_uchar;
type IsEnablediT = extern "system" fn(c_uint, c_uint) -> c_uchar;
type IsFramebufferT = extern "system" fn(c_uint) -> c_uchar;
type IsProgramT = extern "system" fn(c_uint) -> c_uchar;
type IsProgramPipelineT = extern "system" fn(c_uint) -> c_uchar;
type IsQueryT = extern "system" fn(c_uint) -> c_uchar;
type IsRenderbufferT = extern "system" fn(c_uint) -> c_uchar;
type IsSamplerT = extern "system" fn(c_uint) -> c_uchar;
type IsShaderT = extern "system" fn(c_uint) -> c_uchar;
type IsSyncT = extern "system" fn(*mut c_void) -> c_uchar;
type IsTextureT = extern "system" fn(c_uint) -> c_uchar;
type IsTransformFeedbackT = extern "system" fn(c_uint) -> c_uchar;
type IsVertexArrayT = extern "system" fn(c_uint) -> c_uchar;
type LightModelfT = extern "system" fn(c_uint, c_float);
type LightModelfvT = extern "system" fn(c_uint, *const c_float);
type LightModelxT = extern "system" fn(c_uint, c_int);
type LightModelxvT = extern "system" fn(c_uint, *const c_int);
type LightfT = extern "system" fn(c_uint, c_uint, c_float);
type LightfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type LightxT = extern "system" fn(c_uint, c_uint, c_int);
type LightxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type LineWidthT = extern "system" fn(c_float);
type LineWidthxT = extern "system" fn(c_int);
type LinkProgramT = extern "system" fn(c_uint);
type LoadIdentityT = extern "system" fn();
type LoadMatrixfT = extern "system" fn(*const c_float);
type LoadMatrixxT = extern "system" fn(*const c_int);
type LogicOpT = extern "system" fn(c_uint);
type MapBufferT = extern "system" fn(c_uint, c_uint) -> *mut c_void;
type MapBufferRangeT = extern "system" fn(c_uint, isize, isize, c_uint) -> *mut c_void;
type MaterialfT = extern "system" fn(c_uint, c_uint, c_float);
type MaterialfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type MaterialxT = extern "system" fn(c_uint, c_uint, c_int);
type MaterialxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type MatrixModeT = extern "system" fn(c_uint);
type MemoryBarrierT = extern "system" fn(c_uint);
type MemoryBarrierByRegionT = extern "system" fn(c_uint);
type MinSampleShadingT = extern "system" fn(c_float);
type MultMatrixfT = extern "system" fn(*const c_float);
type MultMatrixxT = extern "system" fn(*const c_int);
type MultiDrawArraysT = extern "system" fn(c_uint, *const c_int, *const c_int, c_int);
type MultiDrawElementsT =
    extern "system" fn(c_uint, *const c_int, c_uint, *const *const c_void, c_int);
type MultiDrawElementsBaseVertexT =
    extern "system" fn(c_uint, *const c_int, c_uint, *const *const c_void, c_int, *const c_int);
type MultiTexCoord4fT = extern "system" fn(c_uint, c_float, c_float, c_float, c_float);
type MultiTexCoord4xT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type MultiTexCoordP1uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP1uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP2uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP2uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP3uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP3uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type MultiTexCoordP4uiT = extern "system" fn(c_uint, c_uint, c_uint);
type MultiTexCoordP4uivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type Normal3fT = extern "system" fn(c_float, c_float, c_float);
type Normal3xT = extern "system" fn(c_int, c_int, c_int);
type NormalP3uiT = extern "system" fn(c_uint, c_uint);
type NormalP3uivT = extern "system" fn(c_uint, *const c_uint);
type NormalPointerT = extern "system" fn(c_uint, c_int, *const c_void);
type ObjectLabelT = extern "system" fn(c_uint, c_uint, c_int, *const c_char);
type ObjectPtrLabelT = extern "system" fn(*const c_void, c_int, *const c_char);
type OrthofT = extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float);
type OrthoxT = extern "system" fn(c_int, c_int, c_int, c_int, c_int, c_int);
type PatchParameteriT = extern "system" fn(c_uint, c_int);
type PauseTransformFeedbackT = extern "system" fn();
type PixelStorefT = extern "system" fn(c_uint, c_float);
type PixelStoreiT = extern "system" fn(c_uint, c_int);
type PointParameterfT = extern "system" fn(c_uint, c_float);
type PointParameterfvT = extern "system" fn(c_uint, *const c_float);
type PointParameteriT = extern "system" fn(c_uint, c_int);
type PointParameterivT = extern "system" fn(c_uint, *const c_int);
type PointParameterxT = extern "system" fn(c_uint, c_int);
type PointParameterxvT = extern "system" fn(c_uint, *const c_int);
type PointSizeT = extern "system" fn(c_float);
type PointSizexT = extern "system" fn(c_int);
type PolygonModeT = extern "system" fn(c_uint, c_uint);
type PolygonOffsetT = extern "system" fn(c_float, c_float);
type PolygonOffsetxT = extern "system" fn(c_int, c_int);
type PopDebugGroupT = extern "system" fn();
type PopMatrixT = extern "system" fn();
type PrimitiveBoundingBoxT =
    extern "system" fn(c_float, c_float, c_float, c_float, c_float, c_float, c_float, c_float);
type PrimitiveRestartIndexT = extern "system" fn(c_uint);
type ProgramBinaryT = extern "system" fn(c_uint, c_uint, *const c_void, c_int);
type ProgramParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type ProgramUniform1fT = extern "system" fn(c_uint, c_int, c_float);
type ProgramUniform1fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform1iT = extern "system" fn(c_uint, c_int, c_int);
type ProgramUniform1ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform1uiT = extern "system" fn(c_uint, c_int, c_uint);
type ProgramUniform1uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform2fT = extern "system" fn(c_uint, c_int, c_float, c_float);
type ProgramUniform2fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform2iT = extern "system" fn(c_uint, c_int, c_int, c_int);
type ProgramUniform2ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform2uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint);
type ProgramUniform2uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform3fT = extern "system" fn(c_uint, c_int, c_float, c_float, c_float);
type ProgramUniform3fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform3iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type ProgramUniform3ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform3uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint, c_uint);
type ProgramUniform3uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniform4fT = extern "system" fn(c_uint, c_int, c_float, c_float, c_float, c_float);
type ProgramUniform4fvT = extern "system" fn(c_uint, c_int, c_int, *const c_float);
type ProgramUniform4iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int);
type ProgramUniform4ivT = extern "system" fn(c_uint, c_int, c_int, *const c_int);
type ProgramUniform4uiT = extern "system" fn(c_uint, c_int, c_uint, c_uint, c_uint, c_uint);
type ProgramUniform4uivT = extern "system" fn(c_uint, c_int, c_int, *const c_uint);
type ProgramUniformMatrix2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix2x3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix2x4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3x2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix3x4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4x2fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProgramUniformMatrix4x3fvT = extern "system" fn(c_uint, c_int, c_int, c_uchar, *const c_float);
type ProvokingVertexT = extern "system" fn(c_uint);
type PushDebugGroupT = extern "system" fn(c_uint, c_uint, c_int, *const c_char);
type PushMatrixT = extern "system" fn();
type QueryCounterT = extern "system" fn(c_uint, c_uint);
type ReadBufferT = extern "system" fn(c_uint);
type ReadPixelsT = extern "system" fn(c_int, c_int, c_int, c_int, c_uint, c_uint, *mut c_void);
type ReadnPixelsT =
    extern "system" fn(c_int, c_int, c_int, c_int, c_uint, c_uint, c_int, *mut c_void);
type ReleaseShaderCompilerT = extern "system" fn();
type RenderbufferStorageT = extern "system" fn(c_uint, c_uint, c_int, c_int);
type RenderbufferStorageMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int);
type ResumeTransformFeedbackT = extern "system" fn();
type RotatefT = extern "system" fn(c_float, c_float, c_float, c_float);
type RotatexT = extern "system" fn(c_int, c_int, c_int, c_int);
type SampleCoverageT = extern "system" fn(c_float, c_uchar);
type SampleCoveragexT = extern "system" fn(c_int, c_uchar);
type SampleMaskiT = extern "system" fn(c_uint, c_uint);
type SamplerParameterIivT = extern "system" fn(c_uint, c_uint, *const c_int);
type SamplerParameterIuivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type SamplerParameterfT = extern "system" fn(c_uint, c_uint, c_float);
type SamplerParameterfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type SamplerParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type SamplerParameterivT = extern "system" fn(c_uint, c_uint, *const c_int);
type ScalefT = extern "system" fn(c_float, c_float, c_float);
type ScalexT = extern "system" fn(c_int, c_int, c_int);
type ScissorT = extern "system" fn(c_int, c_int, c_int, c_int);
type SecondaryColorP3uiT = extern "system" fn(c_uint, c_uint);
type SecondaryColorP3uivT = extern "system" fn(c_uint, *const c_uint);
type ShadeModelT = extern "system" fn(c_uint);
type ShaderBinaryT = extern "system" fn(c_int, *const c_uint, c_uint, *const c_void, c_int);
type ShaderSourceT = extern "system" fn(c_uint, c_int, *const *const c_char, *const c_int);
type StencilFuncT = extern "system" fn(c_uint, c_int, c_uint);
type StencilFuncSeparateT = extern "system" fn(c_uint, c_uint, c_int, c_uint);
type StencilMaskT = extern "system" fn(c_uint);
type StencilMaskSeparateT = extern "system" fn(c_uint, c_uint);
type StencilOpT = extern "system" fn(c_uint, c_uint, c_uint);
type StencilOpSeparateT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type TexBufferT = extern "system" fn(c_uint, c_uint, c_uint);
type TexBufferRangeT = extern "system" fn(c_uint, c_uint, c_uint, isize, isize);
type TexCoordP1uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP1uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP2uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP2uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP3uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP3uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordP4uiT = extern "system" fn(c_uint, c_uint);
type TexCoordP4uivT = extern "system" fn(c_uint, *const c_uint);
type TexCoordPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type TexEnvfT = extern "system" fn(c_uint, c_uint, c_float);
type TexEnvfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type TexEnviT = extern "system" fn(c_uint, c_uint, c_int);
type TexEnvivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexEnvxT = extern "system" fn(c_uint, c_uint, c_int);
type TexEnvxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexImage2DMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_uchar);
type TexImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    *const c_void,
);
type TexImage3DMultisampleT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_uchar);
type TexParameterIivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexParameterIuivT = extern "system" fn(c_uint, c_uint, *const c_uint);
type TexParameterfT = extern "system" fn(c_uint, c_uint, c_float);
type TexParameterfvT = extern "system" fn(c_uint, c_uint, *const c_float);
type TexParameteriT = extern "system" fn(c_uint, c_uint, c_int);
type TexParameterivT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexParameterxT = extern "system" fn(c_uint, c_uint, c_int);
type TexParameterxvT = extern "system" fn(c_uint, c_uint, *const c_int);
type TexStorage2DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int);
type TexStorage2DMultisampleT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_uchar);
type TexStorage3DT = extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int);
type TexStorage3DMultisampleT =
    extern "system" fn(c_uint, c_int, c_uint, c_int, c_int, c_int, c_uchar);
type TexSubImage1DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexSubImage2DT =
    extern "system" fn(c_uint, c_int, c_int, c_int, c_int, c_int, c_uint, c_uint, *const c_void);
type TexSubImage3DT = extern "system" fn(
    c_uint,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_uint,
    *const c_void,
);
type TransformFeedbackVaryingsT = extern "system" fn(c_uint, c_int, *const *const c_char, c_uint);
type TranslatefT = extern "system" fn(c_float, c_float, c_float);
type TranslatexT = extern "system" fn(c_int, c_int, c_int);
type Uniform1fT = extern "system" fn(c_int, c_float);
type Uniform1fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform1iT = extern "system" fn(c_int, c_int);
type Uniform1ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform1uiT = extern "system" fn(c_int, c_uint);
type Uniform1uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform2fT = extern "system" fn(c_int, c_float, c_float);
type Uniform2fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform2iT = extern "system" fn(c_int, c_int, c_int);
type Uniform2ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform2uiT = extern "system" fn(c_int, c_uint, c_uint);
type Uniform2uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform3fT = extern "system" fn(c_int, c_float, c_float, c_float);
type Uniform3fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform3iT = extern "system" fn(c_int, c_int, c_int, c_int);
type Uniform3ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform3uiT = extern "system" fn(c_int, c_uint, c_uint, c_uint);
type Uniform3uivT = extern "system" fn(c_int, c_int, *const c_uint);
type Uniform4fT = extern "system" fn(c_int, c_float, c_float, c_float, c_float);
type Uniform4fvT = extern "system" fn(c_int, c_int, *const c_float);
type Uniform4iT = extern "system" fn(c_int, c_int, c_int, c_int, c_int);
type Uniform4ivT = extern "system" fn(c_int, c_int, *const c_int);
type Uniform4uiT = extern "system" fn(c_int, c_uint, c_uint, c_uint, c_uint);
type Uniform4uivT = extern "system" fn(c_int, c_int, *const c_uint);
type UniformBlockBindingT = extern "system" fn(c_uint, c_uint, c_uint);
type UniformMatrix2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix2x3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix2x4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3x2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix3x4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4x2fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UniformMatrix4x3fvT = extern "system" fn(c_int, c_int, c_uchar, *const c_float);
type UnmapBufferT = extern "system" fn(c_uint) -> c_uchar;
type UseProgramT = extern "system" fn(c_uint);
type UseProgramStagesT = extern "system" fn(c_uint, c_uint, c_uint);
type ValidateProgramT = extern "system" fn(c_uint);
type ValidateProgramPipelineT = extern "system" fn(c_uint);
type VertexAttrib1dT = extern "system" fn(c_uint, c_double);
type VertexAttrib1dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib1fT = extern "system" fn(c_uint, c_float);
type VertexAttrib1fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib1sT = extern "system" fn(c_uint, c_short);
type VertexAttrib1svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib2dT = extern "system" fn(c_uint, c_double, c_double);
type VertexAttrib2dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib2fT = extern "system" fn(c_uint, c_float, c_float);
type VertexAttrib2fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib2sT = extern "system" fn(c_uint, c_short, c_short);
type VertexAttrib2svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib3dT = extern "system" fn(c_uint, c_double, c_double, c_double);
type VertexAttrib3dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib3fT = extern "system" fn(c_uint, c_float, c_float, c_float);
type VertexAttrib3fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib3sT = extern "system" fn(c_uint, c_short, c_short, c_short);
type VertexAttrib3svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4NbvT = extern "system" fn(c_uint, *const c_char);
type VertexAttrib4NivT = extern "system" fn(c_uint, *const c_int);
type VertexAttrib4NsvT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4NubT = extern "system" fn(c_uint, c_uchar, c_uchar, c_uchar, c_uchar);
type VertexAttrib4NubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttrib4NuivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttrib4NusvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttrib4bvT = extern "system" fn(c_uint, *const c_char);
type VertexAttrib4dT = extern "system" fn(c_uint, c_double, c_double, c_double, c_double);
type VertexAttrib4dvT = extern "system" fn(c_uint, *const c_double);
type VertexAttrib4fT = extern "system" fn(c_uint, c_float, c_float, c_float, c_float);
type VertexAttrib4fvT = extern "system" fn(c_uint, *const c_float);
type VertexAttrib4ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttrib4sT = extern "system" fn(c_uint, c_short, c_short, c_short, c_short);
type VertexAttrib4svT = extern "system" fn(c_uint, *const c_short);
type VertexAttrib4ubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttrib4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttrib4usvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttribBindingT = extern "system" fn(c_uint, c_uint);
type VertexAttribDivisorT = extern "system" fn(c_uint, c_uint);
type VertexAttribFormatT = extern "system" fn(c_uint, c_int, c_uint, c_uchar, c_uint);
type VertexAttribI1iT = extern "system" fn(c_uint, c_int);
type VertexAttribI1ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI1uiT = extern "system" fn(c_uint, c_uint);
type VertexAttribI1uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI2iT = extern "system" fn(c_uint, c_int, c_int);
type VertexAttribI2ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI2uiT = extern "system" fn(c_uint, c_uint, c_uint);
type VertexAttribI2uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI3iT = extern "system" fn(c_uint, c_int, c_int, c_int);
type VertexAttribI3ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI3uiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint);
type VertexAttribI3uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI4bvT = extern "system" fn(c_uint, *const c_char);
type VertexAttribI4iT = extern "system" fn(c_uint, c_int, c_int, c_int, c_int);
type VertexAttribI4ivT = extern "system" fn(c_uint, *const c_int);
type VertexAttribI4svT = extern "system" fn(c_uint, *const c_short);
type VertexAttribI4ubvT = extern "system" fn(c_uint, *const c_uchar);
type VertexAttribI4uiT = extern "system" fn(c_uint, c_uint, c_uint, c_uint, c_uint);
type VertexAttribI4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexAttribI4usvT = extern "system" fn(c_uint, *const c_ushort);
type VertexAttribIFormatT = extern "system" fn(c_uint, c_int, c_uint, c_uint);
type VertexAttribIPointerT = extern "system" fn(c_uint, c_int, c_uint, c_int, *const c_void);
type VertexAttribP1uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP1uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP2uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP2uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP3uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP3uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribP4uiT = extern "system" fn(c_uint, c_uint, c_uchar, c_uint);
type VertexAttribP4uivT = extern "system" fn(c_uint, c_uint, c_uchar, *const c_uint);
type VertexAttribPointerT =
    extern "system" fn(c_uint, c_int, c_uint, c_uchar, c_int, *const c_void);
type VertexBindingDivisorT = extern "system" fn(c_uint, c_uint);
type VertexP2uiT = extern "system" fn(c_uint, c_uint);
type VertexP2uivT = extern "system" fn(c_uint, *const c_uint);
type VertexP3uiT = extern "system" fn(c_uint, c_uint);
type VertexP3uivT = extern "system" fn(c_uint, *const c_uint);
type VertexP4uiT = extern "system" fn(c_uint, c_uint);
type VertexP4uivT = extern "system" fn(c_uint, *const c_uint);
type VertexPointerT = extern "system" fn(c_int, c_uint, c_int, *const c_void);
type ViewportT = extern "system" fn(c_int, c_int, c_int, c_int);
type WaitSyncT = extern "system" fn(*mut c_void, c_uint, u64);

pub struct GlCmd {
    active_shader_program_p: ActiveShaderProgramT,
    active_texture_p: ActiveTextureT,
    alpha_func_p: AlphaFuncT,
    alpha_funcx_p: AlphaFuncxT,
    attach_shader_p: AttachShaderT,
    begin_conditional_render_p: BeginConditionalRenderT,
    begin_query_p: BeginQueryT,
    begin_transform_feedback_p: BeginTransformFeedbackT,
    bind_attrib_location_p: BindAttribLocationT,
    bind_buffer_p: BindBufferT,
    bind_buffer_base_p: BindBufferBaseT,
    bind_buffer_range_p: BindBufferRangeT,
    bind_frag_data_location_p: BindFragDataLocationT,
    bind_frag_data_location_indexed_p: BindFragDataLocationIndexedT,
    bind_framebuffer_p: BindFramebufferT,
    bind_image_texture_p: BindImageTextureT,
    bind_program_pipeline_p: BindProgramPipelineT,
    bind_renderbuffer_p: BindRenderbufferT,
    bind_sampler_p: BindSamplerT,
    bind_texture_p: BindTextureT,
    bind_transform_feedback_p: BindTransformFeedbackT,
    bind_vertex_array_p: BindVertexArrayT,
    bind_vertex_buffer_p: BindVertexBufferT,
    blend_barrier_p: BlendBarrierT,
    blend_color_p: BlendColorT,
    blend_equation_p: BlendEquationT,
    blend_equation_separate_p: BlendEquationSeparateT,
    blend_equation_separatei_p: BlendEquationSeparateiT,
    blend_equationi_p: BlendEquationiT,
    blend_func_p: BlendFuncT,
    blend_func_separate_p: BlendFuncSeparateT,
    blend_func_separatei_p: BlendFuncSeparateiT,
    blend_funci_p: BlendFunciT,
    blit_framebuffer_p: BlitFramebufferT,
    buffer_data_p: BufferDataT,
    buffer_sub_data_p: BufferSubDataT,
    check_framebuffer_status_p: CheckFramebufferStatusT,
    clamp_color_p: ClampColorT,
    clear_p: ClearT,
    clear_bufferfi_p: ClearBufferfiT,
    clear_bufferfv_p: ClearBufferfvT,
    clear_bufferiv_p: ClearBufferivT,
    clear_bufferuiv_p: ClearBufferuivT,
    clear_color_p: ClearColorT,
    clear_colorx_p: ClearColorxT,
    clear_depth_p: ClearDepthT,
    clear_depthf_p: ClearDepthfT,
    clear_depthx_p: ClearDepthxT,
    clear_stencil_p: ClearStencilT,
    client_active_texture_p: ClientActiveTextureT,
    client_wait_sync_p: ClientWaitSyncT,
    clip_planef_p: ClipPlanefT,
    clip_planex_p: ClipPlanexT,
    color4f_p: Color4fT,
    color4ub_p: Color4ubT,
    color4x_p: Color4xT,
    color_mask_p: ColorMaskT,
    color_maski_p: ColorMaskiT,
    color_p3ui_p: ColorP3uiT,
    color_p3uiv_p: ColorP3uivT,
    color_p4ui_p: ColorP4uiT,
    color_p4uiv_p: ColorP4uivT,
    color_pointer_p: ColorPointerT,
    compile_shader_p: CompileShaderT,
    compressed_tex_image1_d_p: CompressedTexImage1DT,
    compressed_tex_image2_d_p: CompressedTexImage2DT,
    compressed_tex_image3_d_p: CompressedTexImage3DT,
    compressed_tex_sub_image1_d_p: CompressedTexSubImage1DT,
    compressed_tex_sub_image2_d_p: CompressedTexSubImage2DT,
    compressed_tex_sub_image3_d_p: CompressedTexSubImage3DT,
    copy_buffer_sub_data_p: CopyBufferSubDataT,
    copy_image_sub_data_p: CopyImageSubDataT,
    copy_tex_image1_d_p: CopyTexImage1DT,
    copy_tex_image2_d_p: CopyTexImage2DT,
    copy_tex_sub_image1_d_p: CopyTexSubImage1DT,
    copy_tex_sub_image2_d_p: CopyTexSubImage2DT,
    copy_tex_sub_image3_d_p: CopyTexSubImage3DT,
    create_program_p: CreateProgramT,
    create_shader_p: CreateShaderT,
    create_shader_programv_p: CreateShaderProgramvT,
    cull_face_p: CullFaceT,
    debug_message_callback_p: DebugMessageCallbackT,
    debug_message_control_p: DebugMessageControlT,
    debug_message_insert_p: DebugMessageInsertT,
    delete_buffers_p: DeleteBuffersT,
    delete_framebuffers_p: DeleteFramebuffersT,
    delete_program_p: DeleteProgramT,
    delete_program_pipelines_p: DeleteProgramPipelinesT,
    delete_queries_p: DeleteQueriesT,
    delete_renderbuffers_p: DeleteRenderbuffersT,
    delete_samplers_p: DeleteSamplersT,
    delete_shader_p: DeleteShaderT,
    delete_sync_p: DeleteSyncT,
    delete_textures_p: DeleteTexturesT,
    delete_transform_feedbacks_p: DeleteTransformFeedbacksT,
    delete_vertex_arrays_p: DeleteVertexArraysT,
    depth_func_p: DepthFuncT,
    depth_mask_p: DepthMaskT,
    depth_range_p: DepthRangeT,
    depth_rangef_p: DepthRangefT,
    depth_rangex_p: DepthRangexT,
    detach_shader_p: DetachShaderT,
    disable_p: DisableT,
    disable_client_state_p: DisableClientStateT,
    disable_vertex_attrib_array_p: DisableVertexAttribArrayT,
    disablei_p: DisableiT,
    dispatch_compute_p: DispatchComputeT,
    dispatch_compute_indirect_p: DispatchComputeIndirectT,
    draw_arrays_p: DrawArraysT,
    draw_arrays_indirect_p: DrawArraysIndirectT,
    draw_arrays_instanced_p: DrawArraysInstancedT,
    draw_buffer_p: DrawBufferT,
    draw_buffers_p: DrawBuffersT,
    draw_elements_p: DrawElementsT,
    draw_elements_base_vertex_p: DrawElementsBaseVertexT,
    draw_elements_indirect_p: DrawElementsIndirectT,
    draw_elements_instanced_p: DrawElementsInstancedT,
    draw_elements_instanced_base_vertex_p: DrawElementsInstancedBaseVertexT,
    draw_range_elements_p: DrawRangeElementsT,
    draw_range_elements_base_vertex_p: DrawRangeElementsBaseVertexT,
    enable_p: EnableT,
    enable_client_state_p: EnableClientStateT,
    enable_vertex_attrib_array_p: EnableVertexAttribArrayT,
    enablei_p: EnableiT,
    end_conditional_render_p: EndConditionalRenderT,
    end_query_p: EndQueryT,
    end_transform_feedback_p: EndTransformFeedbackT,
    fence_sync_p: FenceSyncT,
    finish_p: FinishT,
    flush_p: FlushT,
    flush_mapped_buffer_range_p: FlushMappedBufferRangeT,
    fogf_p: FogfT,
    fogfv_p: FogfvT,
    fogx_p: FogxT,
    fogxv_p: FogxvT,
    framebuffer_parameteri_p: FramebufferParameteriT,
    framebuffer_renderbuffer_p: FramebufferRenderbufferT,
    framebuffer_texture_p: FramebufferTextureT,
    framebuffer_texture1_d_p: FramebufferTexture1DT,
    framebuffer_texture2_d_p: FramebufferTexture2DT,
    framebuffer_texture3_d_p: FramebufferTexture3DT,
    framebuffer_texture_layer_p: FramebufferTextureLayerT,
    front_face_p: FrontFaceT,
    frustumf_p: FrustumfT,
    frustumx_p: FrustumxT,
    gen_buffers_p: GenBuffersT,
    gen_framebuffers_p: GenFramebuffersT,
    gen_program_pipelines_p: GenProgramPipelinesT,
    gen_queries_p: GenQueriesT,
    gen_renderbuffers_p: GenRenderbuffersT,
    gen_samplers_p: GenSamplersT,
    gen_textures_p: GenTexturesT,
    gen_transform_feedbacks_p: GenTransformFeedbacksT,
    gen_vertex_arrays_p: GenVertexArraysT,
    generate_mipmap_p: GenerateMipmapT,
    get_active_attrib_p: GetActiveAttribT,
    get_active_uniform_p: GetActiveUniformT,
    get_active_uniform_block_name_p: GetActiveUniformBlockNameT,
    get_active_uniform_blockiv_p: GetActiveUniformBlockivT,
    get_active_uniform_name_p: GetActiveUniformNameT,
    get_active_uniformsiv_p: GetActiveUniformsivT,
    get_attached_shaders_p: GetAttachedShadersT,
    get_attrib_location_p: GetAttribLocationT,
    get_booleani_v_p: GetBooleanivT,
    get_booleanv_p: GetBooleanvT,
    get_buffer_parameteri64v_p: GetBufferParameteri64vT,
    get_buffer_parameteriv_p: GetBufferParameterivT,
    get_buffer_pointerv_p: GetBufferPointervT,
    get_buffer_sub_data_p: GetBufferSubDataT,
    get_clip_planef_p: GetClipPlanefT,
    get_clip_planex_p: GetClipPlanexT,
    get_compressed_tex_image_p: GetCompressedTexImageT,
    get_debug_message_log_p: GetDebugMessageLogT,
    get_doublev_p: GetDoublevT,
    get_error_p: GetErrorT,
    get_fixedv_p: GetFixedvT,
    get_floatv_p: GetFloatvT,
    get_frag_data_index_p: GetFragDataIndexT,
    get_frag_data_location_p: GetFragDataLocationT,
    get_framebuffer_attachment_parameteriv_p: GetFramebufferAttachmentParameterivT,
    get_framebuffer_parameteriv_p: GetFramebufferParameterivT,
    get_graphics_reset_status_p: GetGraphicsResetStatusT,
    get_integer64i_v_p: GetInteger64ivT,
    get_integer64v_p: GetInteger64vT,
    get_integeri_v_p: GetIntegerivT,
    get_integerv_p: GetIntegervT,
    get_internalformativ_p: GetInternalformativT,
    get_lightfv_p: GetLightfvT,
    get_lightxv_p: GetLightxvT,
    get_materialfv_p: GetMaterialfvT,
    get_materialxv_p: GetMaterialxvT,
    get_multisamplefv_p: GetMultisamplefvT,
    get_object_label_p: GetObjectLabelT,
    get_object_ptr_label_p: GetObjectPtrLabelT,
    get_pointerv_p: GetPointervT,
    get_program_binary_p: GetProgramBinaryT,
    get_program_info_log_p: GetProgramInfoLogT,
    get_program_interfaceiv_p: GetProgramInterfaceivT,
    get_program_pipeline_info_log_p: GetProgramPipelineInfoLogT,
    get_program_pipelineiv_p: GetProgramPipelineivT,
    get_program_resource_index_p: GetProgramResourceIndexT,
    get_program_resource_location_p: GetProgramResourceLocationT,
    get_program_resource_name_p: GetProgramResourceNameT,
    get_program_resourceiv_p: GetProgramResourceivT,
    get_programiv_p: GetProgramivT,
    get_query_objecti64v_p: GetQueryObjecti64vT,
    get_query_objectiv_p: GetQueryObjectivT,
    get_query_objectui64v_p: GetQueryObjectui64vT,
    get_query_objectuiv_p: GetQueryObjectuivT,
    get_queryiv_p: GetQueryivT,
    get_renderbuffer_parameteriv_p: GetRenderbufferParameterivT,
    get_sampler_parameter_iiv_p: GetSamplerParameterIivT,
    get_sampler_parameter_iuiv_p: GetSamplerParameterIuivT,
    get_sampler_parameterfv_p: GetSamplerParameterfvT,
    get_sampler_parameteriv_p: GetSamplerParameterivT,
    get_shader_info_log_p: GetShaderInfoLogT,
    get_shader_precision_format_p: GetShaderPrecisionFormatT,
    get_shader_source_p: GetShaderSourceT,
    get_shaderiv_p: GetShaderivT,
    get_string_p: GetStringT,
    get_stringi_p: GetStringiT,
    get_synciv_p: GetSyncivT,
    get_tex_envfv_p: GetTexEnvfvT,
    get_tex_enviv_p: GetTexEnvivT,
    get_tex_envxv_p: GetTexEnvxvT,
    get_tex_image_p: GetTexImageT,
    get_tex_level_parameterfv_p: GetTexLevelParameterfvT,
    get_tex_level_parameteriv_p: GetTexLevelParameterivT,
    get_tex_parameter_iiv_p: GetTexParameterIivT,
    get_tex_parameter_iuiv_p: GetTexParameterIuivT,
    get_tex_parameterfv_p: GetTexParameterfvT,
    get_tex_parameteriv_p: GetTexParameterivT,
    get_tex_parameterxv_p: GetTexParameterxvT,
    get_transform_feedback_varying_p: GetTransformFeedbackVaryingT,
    get_uniform_block_index_p: GetUniformBlockIndexT,
    get_uniform_indices_p: GetUniformIndicesT,
    get_uniform_location_p: GetUniformLocationT,
    get_uniformfv_p: GetUniformfvT,
    get_uniformiv_p: GetUniformivT,
    get_uniformuiv_p: GetUniformuivT,
    get_vertex_attrib_iiv_p: GetVertexAttribIivT,
    get_vertex_attrib_iuiv_p: GetVertexAttribIuivT,
    get_vertex_attrib_pointerv_p: GetVertexAttribPointervT,
    get_vertex_attribdv_p: GetVertexAttribdvT,
    get_vertex_attribfv_p: GetVertexAttribfvT,
    get_vertex_attribiv_p: GetVertexAttribivT,
    getn_uniformfv_p: GetnUniformfvT,
    getn_uniformiv_p: GetnUniformivT,
    getn_uniformuiv_p: GetnUniformuivT,
    hint_p: HintT,
    invalidate_framebuffer_p: InvalidateFramebufferT,
    invalidate_sub_framebuffer_p: InvalidateSubFramebufferT,
    is_buffer_p: IsBufferT,
    is_enabled_p: IsEnabledT,
    is_enabledi_p: IsEnablediT,
    is_framebuffer_p: IsFramebufferT,
    is_program_p: IsProgramT,
    is_program_pipeline_p: IsProgramPipelineT,
    is_query_p: IsQueryT,
    is_renderbuffer_p: IsRenderbufferT,
    is_sampler_p: IsSamplerT,
    is_shader_p: IsShaderT,
    is_sync_p: IsSyncT,
    is_texture_p: IsTextureT,
    is_transform_feedback_p: IsTransformFeedbackT,
    is_vertex_array_p: IsVertexArrayT,
    light_modelf_p: LightModelfT,
    light_modelfv_p: LightModelfvT,
    light_modelx_p: LightModelxT,
    light_modelxv_p: LightModelxvT,
    lightf_p: LightfT,
    lightfv_p: LightfvT,
    lightx_p: LightxT,
    lightxv_p: LightxvT,
    line_width_p: LineWidthT,
    line_widthx_p: LineWidthxT,
    link_program_p: LinkProgramT,
    load_identity_p: LoadIdentityT,
    load_matrixf_p: LoadMatrixfT,
    load_matrixx_p: LoadMatrixxT,
    logic_op_p: LogicOpT,
    map_buffer_p: MapBufferT,
    map_buffer_range_p: MapBufferRangeT,
    materialf_p: MaterialfT,
    materialfv_p: MaterialfvT,
    materialx_p: MaterialxT,
    materialxv_p: MaterialxvT,
    matrix_mode_p: MatrixModeT,
    memory_barrier_p: MemoryBarrierT,
    memory_barrier_by_region_p: MemoryBarrierByRegionT,
    min_sample_shading_p: MinSampleShadingT,
    mult_matrixf_p: MultMatrixfT,
    mult_matrixx_p: MultMatrixxT,
    multi_draw_arrays_p: MultiDrawArraysT,
    multi_draw_elements_p: MultiDrawElementsT,
    multi_draw_elements_base_vertex_p: MultiDrawElementsBaseVertexT,
    multi_tex_coord4f_p: MultiTexCoord4fT,
    multi_tex_coord4x_p: MultiTexCoord4xT,
    multi_tex_coord_p1ui_p: MultiTexCoordP1uiT,
    multi_tex_coord_p1uiv_p: MultiTexCoordP1uivT,
    multi_tex_coord_p2ui_p: MultiTexCoordP2uiT,
    multi_tex_coord_p2uiv_p: MultiTexCoordP2uivT,
    multi_tex_coord_p3ui_p: MultiTexCoordP3uiT,
    multi_tex_coord_p3uiv_p: MultiTexCoordP3uivT,
    multi_tex_coord_p4ui_p: MultiTexCoordP4uiT,
    multi_tex_coord_p4uiv_p: MultiTexCoordP4uivT,
    normal3f_p: Normal3fT,
    normal3x_p: Normal3xT,
    normal_p3ui_p: NormalP3uiT,
    normal_p3uiv_p: NormalP3uivT,
    normal_pointer_p: NormalPointerT,
    object_label_p: ObjectLabelT,
    object_ptr_label_p: ObjectPtrLabelT,
    orthof_p: OrthofT,
    orthox_p: OrthoxT,
    patch_parameteri_p: PatchParameteriT,
    pause_transform_feedback_p: PauseTransformFeedbackT,
    pixel_storef_p: PixelStorefT,
    pixel_storei_p: PixelStoreiT,
    point_parameterf_p: PointParameterfT,
    point_parameterfv_p: PointParameterfvT,
    point_parameteri_p: PointParameteriT,
    point_parameteriv_p: PointParameterivT,
    point_parameterx_p: PointParameterxT,
    point_parameterxv_p: PointParameterxvT,
    point_size_p: PointSizeT,
    point_sizex_p: PointSizexT,
    polygon_mode_p: PolygonModeT,
    polygon_offset_p: PolygonOffsetT,
    polygon_offsetx_p: PolygonOffsetxT,
    pop_debug_group_p: PopDebugGroupT,
    pop_matrix_p: PopMatrixT,
    primitive_bounding_box_p: PrimitiveBoundingBoxT,
    primitive_restart_index_p: PrimitiveRestartIndexT,
    program_binary_p: ProgramBinaryT,
    program_parameteri_p: ProgramParameteriT,
    program_uniform1f_p: ProgramUniform1fT,
    program_uniform1fv_p: ProgramUniform1fvT,
    program_uniform1i_p: ProgramUniform1iT,
    program_uniform1iv_p: ProgramUniform1ivT,
    program_uniform1ui_p: ProgramUniform1uiT,
    program_uniform1uiv_p: ProgramUniform1uivT,
    program_uniform2f_p: ProgramUniform2fT,
    program_uniform2fv_p: ProgramUniform2fvT,
    program_uniform2i_p: ProgramUniform2iT,
    program_uniform2iv_p: ProgramUniform2ivT,
    program_uniform2ui_p: ProgramUniform2uiT,
    program_uniform2uiv_p: ProgramUniform2uivT,
    program_uniform3f_p: ProgramUniform3fT,
    program_uniform3fv_p: ProgramUniform3fvT,
    program_uniform3i_p: ProgramUniform3iT,
    program_uniform3iv_p: ProgramUniform3ivT,
    program_uniform3ui_p: ProgramUniform3uiT,
    program_uniform3uiv_p: ProgramUniform3uivT,
    program_uniform4f_p: ProgramUniform4fT,
    program_uniform4fv_p: ProgramUniform4fvT,
    program_uniform4i_p: ProgramUniform4iT,
    program_uniform4iv_p: ProgramUniform4ivT,
    program_uniform4ui_p: ProgramUniform4uiT,
    program_uniform4uiv_p: ProgramUniform4uivT,
    program_uniform_matrix2fv_p: ProgramUniformMatrix2fvT,
    program_uniform_matrix2x3fv_p: ProgramUniformMatrix2x3fvT,
    program_uniform_matrix2x4fv_p: ProgramUniformMatrix2x4fvT,
    program_uniform_matrix3fv_p: ProgramUniformMatrix3fvT,
    program_uniform_matrix3x2fv_p: ProgramUniformMatrix3x2fvT,
    program_uniform_matrix3x4fv_p: ProgramUniformMatrix3x4fvT,
    program_uniform_matrix4fv_p: ProgramUniformMatrix4fvT,
    program_uniform_matrix4x2fv_p: ProgramUniformMatrix4x2fvT,
    program_uniform_matrix4x3fv_p: ProgramUniformMatrix4x3fvT,
    provoking_vertex_p: ProvokingVertexT,
    push_debug_group_p: PushDebugGroupT,
    push_matrix_p: PushMatrixT,
    query_counter_p: QueryCounterT,
    read_buffer_p: ReadBufferT,
    read_pixels_p: ReadPixelsT,
    readn_pixels_p: ReadnPixelsT,
    release_shader_compiler_p: ReleaseShaderCompilerT,
    renderbuffer_storage_p: RenderbufferStorageT,
    renderbuffer_storage_multisample_p: RenderbufferStorageMultisampleT,
    resume_transform_feedback_p: ResumeTransformFeedbackT,
    rotatef_p: RotatefT,
    rotatex_p: RotatexT,
    sample_coverage_p: SampleCoverageT,
    sample_coveragex_p: SampleCoveragexT,
    sample_maski_p: SampleMaskiT,
    sampler_parameter_iiv_p: SamplerParameterIivT,
    sampler_parameter_iuiv_p: SamplerParameterIuivT,
    sampler_parameterf_p: SamplerParameterfT,
    sampler_parameterfv_p: SamplerParameterfvT,
    sampler_parameteri_p: SamplerParameteriT,
    sampler_parameteriv_p: SamplerParameterivT,
    scalef_p: ScalefT,
    scalex_p: ScalexT,
    scissor_p: ScissorT,
    secondary_color_p3ui_p: SecondaryColorP3uiT,
    secondary_color_p3uiv_p: SecondaryColorP3uivT,
    shade_model_p: ShadeModelT,
    shader_binary_p: ShaderBinaryT,
    shader_source_p: ShaderSourceT,
    stencil_func_p: StencilFuncT,
    stencil_func_separate_p: StencilFuncSeparateT,
    stencil_mask_p: StencilMaskT,
    stencil_mask_separate_p: StencilMaskSeparateT,
    stencil_op_p: StencilOpT,
    stencil_op_separate_p: StencilOpSeparateT,
    tex_buffer_p: TexBufferT,
    tex_buffer_range_p: TexBufferRangeT,
    tex_coord_p1ui_p: TexCoordP1uiT,
    tex_coord_p1uiv_p: TexCoordP1uivT,
    tex_coord_p2ui_p: TexCoordP2uiT,
    tex_coord_p2uiv_p: TexCoordP2uivT,
    tex_coord_p3ui_p: TexCoordP3uiT,
    tex_coord_p3uiv_p: TexCoordP3uivT,
    tex_coord_p4ui_p: TexCoordP4uiT,
    tex_coord_p4uiv_p: TexCoordP4uivT,
    tex_coord_pointer_p: TexCoordPointerT,
    tex_envf_p: TexEnvfT,
    tex_envfv_p: TexEnvfvT,
    tex_envi_p: TexEnviT,
    tex_enviv_p: TexEnvivT,
    tex_envx_p: TexEnvxT,
    tex_envxv_p: TexEnvxvT,
    tex_image1_d_p: TexImage1DT,
    tex_image2_d_p: TexImage2DT,
    tex_image2_d_multisample_p: TexImage2DMultisampleT,
    tex_image3_d_p: TexImage3DT,
    tex_image3_d_multisample_p: TexImage3DMultisampleT,
    tex_parameter_iiv_p: TexParameterIivT,
    tex_parameter_iuiv_p: TexParameterIuivT,
    tex_parameterf_p: TexParameterfT,
    tex_parameterfv_p: TexParameterfvT,
    tex_parameteri_p: TexParameteriT,
    tex_parameteriv_p: TexParameterivT,
    tex_parameterx_p: TexParameterxT,
    tex_parameterxv_p: TexParameterxvT,
    tex_storage2_d_p: TexStorage2DT,
    tex_storage2_d_multisample_p: TexStorage2DMultisampleT,
    tex_storage3_d_p: TexStorage3DT,
    tex_storage3_d_multisample_p: TexStorage3DMultisampleT,
    tex_sub_image1_d_p: TexSubImage1DT,
    tex_sub_image2_d_p: TexSubImage2DT,
    tex_sub_image3_d_p: TexSubImage3DT,
    transform_feedback_varyings_p: TransformFeedbackVaryingsT,
    translatef_p: TranslatefT,
    translatex_p: TranslatexT,
    uniform1f_p: Uniform1fT,
    uniform1fv_p: Uniform1fvT,
    uniform1i_p: Uniform1iT,
    uniform1iv_p: Uniform1ivT,
    uniform1ui_p: Uniform1uiT,
    uniform1uiv_p: Uniform1uivT,
    uniform2f_p: Uniform2fT,
    uniform2fv_p: Uniform2fvT,
    uniform2i_p: Uniform2iT,
    uniform2iv_p: Uniform2ivT,
    uniform2ui_p: Uniform2uiT,
    uniform2uiv_p: Uniform2uivT,
    uniform3f_p: Uniform3fT,
    uniform3fv_p: Uniform3fvT,
    uniform3i_p: Uniform3iT,
    uniform3iv_p: Uniform3ivT,
    uniform3ui_p: Uniform3uiT,
    uniform3uiv_p: Uniform3uivT,
    uniform4f_p: Uniform4fT,
    uniform4fv_p: Uniform4fvT,
    uniform4i_p: Uniform4iT,
    uniform4iv_p: Uniform4ivT,
    uniform4ui_p: Uniform4uiT,
    uniform4uiv_p: Uniform4uivT,
    uniform_block_binding_p: UniformBlockBindingT,
    uniform_matrix2fv_p: UniformMatrix2fvT,
    uniform_matrix2x3fv_p: UniformMatrix2x3fvT,
    uniform_matrix2x4fv_p: UniformMatrix2x4fvT,
    uniform_matrix3fv_p: UniformMatrix3fvT,
    uniform_matrix3x2fv_p: UniformMatrix3x2fvT,
    uniform_matrix3x4fv_p: UniformMatrix3x4fvT,
    uniform_matrix4fv_p: UniformMatrix4fvT,
    uniform_matrix4x2fv_p: UniformMatrix4x2fvT,
    uniform_matrix4x3fv_p: UniformMatrix4x3fvT,
    unmap_buffer_p: UnmapBufferT,
    use_program_p: UseProgramT,
    use_program_stages_p: UseProgramStagesT,
    validate_program_p: ValidateProgramT,
    validate_program_pipeline_p: ValidateProgramPipelineT,
    vertex_attrib1d_p: VertexAttrib1dT,
    vertex_attrib1dv_p: VertexAttrib1dvT,
    vertex_attrib1f_p: VertexAttrib1fT,
    vertex_attrib1fv_p: VertexAttrib1fvT,
    vertex_attrib1s_p: VertexAttrib1sT,
    vertex_attrib1sv_p: VertexAttrib1svT,
    vertex_attrib2d_p: VertexAttrib2dT,
    vertex_attrib2dv_p: VertexAttrib2dvT,
    vertex_attrib2f_p: VertexAttrib2fT,
    vertex_attrib2fv_p: VertexAttrib2fvT,
    vertex_attrib2s_p: VertexAttrib2sT,
    vertex_attrib2sv_p: VertexAttrib2svT,
    vertex_attrib3d_p: VertexAttrib3dT,
    vertex_attrib3dv_p: VertexAttrib3dvT,
    vertex_attrib3f_p: VertexAttrib3fT,
    vertex_attrib3fv_p: VertexAttrib3fvT,
    vertex_attrib3s_p: VertexAttrib3sT,
    vertex_attrib3sv_p: VertexAttrib3svT,
    vertex_attrib4_nbv_p: VertexAttrib4NbvT,
    vertex_attrib4_niv_p: VertexAttrib4NivT,
    vertex_attrib4_nsv_p: VertexAttrib4NsvT,
    vertex_attrib4_nub_p: VertexAttrib4NubT,
    vertex_attrib4_nubv_p: VertexAttrib4NubvT,
    vertex_attrib4_nuiv_p: VertexAttrib4NuivT,
    vertex_attrib4_nusv_p: VertexAttrib4NusvT,
    vertex_attrib4bv_p: VertexAttrib4bvT,
    vertex_attrib4d_p: VertexAttrib4dT,
    vertex_attrib4dv_p: VertexAttrib4dvT,
    vertex_attrib4f_p: VertexAttrib4fT,
    vertex_attrib4fv_p: VertexAttrib4fvT,
    vertex_attrib4iv_p: VertexAttrib4ivT,
    vertex_attrib4s_p: VertexAttrib4sT,
    vertex_attrib4sv_p: VertexAttrib4svT,
    vertex_attrib4ubv_p: VertexAttrib4ubvT,
    vertex_attrib4uiv_p: VertexAttrib4uivT,
    vertex_attrib4usv_p: VertexAttrib4usvT,
    vertex_attrib_binding_p: VertexAttribBindingT,
    vertex_attrib_divisor_p: VertexAttribDivisorT,
    vertex_attrib_format_p: VertexAttribFormatT,
    vertex_attrib_i1i_p: VertexAttribI1iT,
    vertex_attrib_i1iv_p: VertexAttribI1ivT,
    vertex_attrib_i1ui_p: VertexAttribI1uiT,
    vertex_attrib_i1uiv_p: VertexAttribI1uivT,
    vertex_attrib_i2i_p: VertexAttribI2iT,
    vertex_attrib_i2iv_p: VertexAttribI2ivT,
    vertex_attrib_i2ui_p: VertexAttribI2uiT,
    vertex_attrib_i2uiv_p: VertexAttribI2uivT,
    vertex_attrib_i3i_p: VertexAttribI3iT,
    vertex_attrib_i3iv_p: VertexAttribI3ivT,
    vertex_attrib_i3ui_p: VertexAttribI3uiT,
    vertex_attrib_i3uiv_p: VertexAttribI3uivT,
    vertex_attrib_i4bv_p: VertexAttribI4bvT,
    vertex_attrib_i4i_p: VertexAttribI4iT,
    vertex_attrib_i4iv_p: VertexAttribI4ivT,
    vertex_attrib_i4sv_p: VertexAttribI4svT,
    vertex_attrib_i4ubv_p: VertexAttribI4ubvT,
    vertex_attrib_i4ui_p: VertexAttribI4uiT,
    vertex_attrib_i4uiv_p: VertexAttribI4uivT,
    vertex_attrib_i4usv_p: VertexAttribI4usvT,
    vertex_attrib_i_format_p: VertexAttribIFormatT,
    vertex_attrib_i_pointer_p: VertexAttribIPointerT,
    vertex_attrib_p1ui_p: VertexAttribP1uiT,
    vertex_attrib_p1uiv_p: VertexAttribP1uivT,
    vertex_attrib_p2ui_p: VertexAttribP2uiT,
    vertex_attrib_p2uiv_p: VertexAttribP2uivT,
    vertex_attrib_p3ui_p: VertexAttribP3uiT,
    vertex_attrib_p3uiv_p: VertexAttribP3uivT,
    vertex_attrib_p4ui_p: VertexAttribP4uiT,
    vertex_attrib_p4uiv_p: VertexAttribP4uivT,
    vertex_attrib_pointer_p: VertexAttribPointerT,
    vertex_binding_divisor_p: VertexBindingDivisorT,
    vertex_p2ui_p: VertexP2uiT,
    vertex_p2uiv_p: VertexP2uivT,
    vertex_p3ui_p: VertexP3uiT,
    vertex_p3uiv_p: VertexP3uivT,
    vertex_p4ui_p: VertexP4uiT,
    vertex_p4uiv_p: VertexP4uivT,
    vertex_pointer_p: VertexPointerT,
    viewport_p: ViewportT,
    wait_sync_p: WaitSyncT,
}

impl GlCmd {
    pub fn load<FnLoad>(&self, loader: &FnLoad) -> Self
    where
        FnLoad: Fn(*const c_char) -> *mut c_void,
    {
        Self {
            active_shader_program_p: load(loader, "glActiveShaderProgram"),
            active_texture_p: load(loader, "glActiveTexture"),
            alpha_func_p: load(loader, "glAlphaFunc"),
            alpha_funcx_p: load(loader, "glAlphaFuncx"),
            attach_shader_p: load(loader, "glAttachShader"),
            begin_conditional_render_p: load(loader, "glBeginConditionalRender"),
            begin_query_p: load(loader, "glBeginQuery"),
            begin_transform_feedback_p: load(loader, "glBeginTransformFeedback"),
            bind_attrib_location_p: load(loader, "glBindAttribLocation"),
            bind_buffer_p: load(loader, "glBindBuffer"),
            bind_buffer_base_p: load(loader, "glBindBufferBase"),
            bind_buffer_range_p: load(loader, "glBindBufferRange"),
            bind_frag_data_location_p: load(loader, "glBindFragDataLocation"),
            bind_frag_data_location_indexed_p: load(loader, "glBindFragDataLocationIndexed"),
            bind_framebuffer_p: load(loader, "glBindFramebuffer"),
            bind_image_texture_p: load(loader, "glBindImageTexture"),
            bind_program_pipeline_p: load(loader, "glBindProgramPipeline"),
            bind_renderbuffer_p: load(loader, "glBindRenderbuffer"),
            bind_sampler_p: load(loader, "glBindSampler"),
            bind_texture_p: load(loader, "glBindTexture"),
            bind_transform_feedback_p: load(loader, "glBindTransformFeedback"),
            bind_vertex_array_p: load(loader, "glBindVertexArray"),
            bind_vertex_buffer_p: load(loader, "glBindVertexBuffer"),
            blend_barrier_p: load(loader, "glBlendBarrier"),
            blend_color_p: load(loader, "glBlendColor"),
            blend_equation_p: load(loader, "glBlendEquation"),
            blend_equation_separate_p: load(loader, "glBlendEquationSeparate"),
            blend_equation_separatei_p: load(loader, "glBlendEquationSeparatei"),
            blend_equationi_p: load(loader, "glBlendEquationi"),
            blend_func_p: load(loader, "glBlendFunc"),
            blend_func_separate_p: load(loader, "glBlendFuncSeparate"),
            blend_func_separatei_p: load(loader, "glBlendFuncSeparatei"),
            blend_funci_p: load(loader, "glBlendFunci"),
            blit_framebuffer_p: load(loader, "glBlitFramebuffer"),
            buffer_data_p: load(loader, "glBufferData"),
            buffer_sub_data_p: load(loader, "glBufferSubData"),
            check_framebuffer_status_p: load(loader, "glCheckFramebufferStatus"),
            clamp_color_p: load(loader, "glClampColor"),
            clear_p: load(loader, "glClear"),
            clear_bufferfi_p: load(loader, "glClearBufferfi"),
            clear_bufferfv_p: load(loader, "glClearBufferfv"),
            clear_bufferiv_p: load(loader, "glClearBufferiv"),
            clear_bufferuiv_p: load(loader, "glClearBufferuiv"),
            clear_color_p: load(loader, "glClearColor"),
            clear_colorx_p: load(loader, "glClearColorx"),
            clear_depth_p: load(loader, "glClearDepth"),
            clear_depthf_p: load(loader, "glClearDepthf"),
            clear_depthx_p: load(loader, "glClearDepthx"),
            clear_stencil_p: load(loader, "glClearStencil"),
            client_active_texture_p: load(loader, "glClientActiveTexture"),
            client_wait_sync_p: load(loader, "glClientWaitSync"),
            clip_planef_p: load(loader, "glClipPlanef"),
            clip_planex_p: load(loader, "glClipPlanex"),
            color4f_p: load(loader, "glColor4f"),
            color4ub_p: load(loader, "glColor4ub"),
            color4x_p: load(loader, "glColor4x"),
            color_mask_p: load(loader, "glColorMask"),
            color_maski_p: load(loader, "glColorMaski"),
            color_p3ui_p: load(loader, "glColorP3ui"),
            color_p3uiv_p: load(loader, "glColorP3uiv"),
            color_p4ui_p: load(loader, "glColorP4ui"),
            color_p4uiv_p: load(loader, "glColorP4uiv"),
            color_pointer_p: load(loader, "glColorPointer"),
            compile_shader_p: load(loader, "glCompileShader"),
            compressed_tex_image1_d_p: load(loader, "glCompressedTexImage1D"),
            compressed_tex_image2_d_p: load(loader, "glCompressedTexImage2D"),
            compressed_tex_image3_d_p: load(loader, "glCompressedTexImage3D"),
            compressed_tex_sub_image1_d_p: load(loader, "glCompressedTexSubImage1D"),
            compressed_tex_sub_image2_d_p: load(loader, "glCompressedTexSubImage2D"),
            compressed_tex_sub_image3_d_p: load(loader, "glCompressedTexSubImage3D"),
            copy_buffer_sub_data_p: load(loader, "glCopyBufferSubData"),
            copy_image_sub_data_p: load(loader, "glCopyImageSubData"),
            copy_tex_image1_d_p: load(loader, "glCopyTexImage1D"),
            copy_tex_image2_d_p: load(loader, "glCopyTexImage2D"),
            copy_tex_sub_image1_d_p: load(loader, "glCopyTexSubImage1D"),
            copy_tex_sub_image2_d_p: load(loader, "glCopyTexSubImage2D"),
            copy_tex_sub_image3_d_p: load(loader, "glCopyTexSubImage3D"),
            create_program_p: load(loader, "glCreateProgram"),
            create_shader_p: load(loader, "glCreateShader"),
            create_shader_programv_p: load(loader, "glCreateShaderProgramv"),
            cull_face_p: load(loader, "glCullFace"),
            debug_message_callback_p: load(loader, "glDebugMessageCallback"),
            debug_message_control_p: load(loader, "glDebugMessageControl"),
            debug_message_insert_p: load(loader, "glDebugMessageInsert"),
            delete_buffers_p: load(loader, "glDeleteBuffers"),
            delete_framebuffers_p: load(loader, "glDeleteFramebuffers"),
            delete_program_p: load(loader, "glDeleteProgram"),
            delete_program_pipelines_p: load(loader, "glDeleteProgramPipelines"),
            delete_queries_p: load(loader, "glDeleteQueries"),
            delete_renderbuffers_p: load(loader, "glDeleteRenderbuffers"),
            delete_samplers_p: load(loader, "glDeleteSamplers"),
            delete_shader_p: load(loader, "glDeleteShader"),
            delete_sync_p: load(loader, "glDeleteSync"),
            delete_textures_p: load(loader, "glDeleteTextures"),
            delete_transform_feedbacks_p: load(loader, "glDeleteTransformFeedbacks"),
            delete_vertex_arrays_p: load(loader, "glDeleteVertexArrays"),
            depth_func_p: load(loader, "glDepthFunc"),
            depth_mask_p: load(loader, "glDepthMask"),
            depth_range_p: load(loader, "glDepthRange"),
            depth_rangef_p: load(loader, "glDepthRangef"),
            depth_rangex_p: load(loader, "glDepthRangex"),
            detach_shader_p: load(loader, "glDetachShader"),
            disable_p: load(loader, "glDisable"),
            disable_client_state_p: load(loader, "glDisableClientState"),
            disable_vertex_attrib_array_p: load(loader, "glDisableVertexAttribArray"),
            disablei_p: load(loader, "glDisablei"),
            dispatch_compute_p: load(loader, "glDispatchCompute"),
            dispatch_compute_indirect_p: load(loader, "glDispatchComputeIndirect"),
            draw_arrays_p: load(loader, "glDrawArrays"),
            draw_arrays_indirect_p: load(loader, "glDrawArraysIndirect"),
            draw_arrays_instanced_p: load(loader, "glDrawArraysInstanced"),
            draw_buffer_p: load(loader, "glDrawBuffer"),
            draw_buffers_p: load(loader, "glDrawBuffers"),
            draw_elements_p: load(loader, "glDrawElements"),
            draw_elements_base_vertex_p: load(loader, "glDrawElementsBaseVertex"),
            draw_elements_indirect_p: load(loader, "glDrawElementsIndirect"),
            draw_elements_instanced_p: load(loader, "glDrawElementsInstanced"),
            draw_elements_instanced_base_vertex_p: load(
                loader,
                "glDrawElementsInstancedBaseVertex",
            ),
            draw_range_elements_p: load(loader, "glDrawRangeElements"),
            draw_range_elements_base_vertex_p: load(loader, "glDrawRangeElementsBaseVertex"),
            enable_p: load(loader, "glEnable"),
            enable_client_state_p: load(loader, "glEnableClientState"),
            enable_vertex_attrib_array_p: load(loader, "glEnableVertexAttribArray"),
            enablei_p: load(loader, "glEnablei"),
            end_conditional_render_p: load(loader, "glEndConditionalRender"),
            end_query_p: load(loader, "glEndQuery"),
            end_transform_feedback_p: load(loader, "glEndTransformFeedback"),
            fence_sync_p: load(loader, "glFenceSync"),
            finish_p: load(loader, "glFinish"),
            flush_p: load(loader, "glFlush"),
            flush_mapped_buffer_range_p: load(loader, "glFlushMappedBufferRange"),
            fogf_p: load(loader, "glFogf"),
            fogfv_p: load(loader, "glFogfv"),
            fogx_p: load(loader, "glFogx"),
            fogxv_p: load(loader, "glFogxv"),
            framebuffer_parameteri_p: load(loader, "glFramebufferParameteri"),
            framebuffer_renderbuffer_p: load(loader, "glFramebufferRenderbuffer"),
            framebuffer_texture_p: load(loader, "glFramebufferTexture"),
            framebuffer_texture1_d_p: load(loader, "glFramebufferTexture1D"),
            framebuffer_texture2_d_p: load(loader, "glFramebufferTexture2D"),
            framebuffer_texture3_d_p: load(loader, "glFramebufferTexture3D"),
            framebuffer_texture_layer_p: load(loader, "glFramebufferTextureLayer"),
            front_face_p: load(loader, "glFrontFace"),
            frustumf_p: load(loader, "glFrustumf"),
            frustumx_p: load(loader, "glFrustumx"),
            gen_buffers_p: load(loader, "glGenBuffers"),
            gen_framebuffers_p: load(loader, "glGenFramebuffers"),
            gen_program_pipelines_p: load(loader, "glGenProgramPipelines"),
            gen_queries_p: load(loader, "glGenQueries"),
            gen_renderbuffers_p: load(loader, "glGenRenderbuffers"),
            gen_samplers_p: load(loader, "glGenSamplers"),
            gen_textures_p: load(loader, "glGenTextures"),
            gen_transform_feedbacks_p: load(loader, "glGenTransformFeedbacks"),
            gen_vertex_arrays_p: load(loader, "glGenVertexArrays"),
            generate_mipmap_p: load(loader, "glGenerateMipmap"),
            get_active_attrib_p: load(loader, "glGetActiveAttrib"),
            get_active_uniform_p: load(loader, "glGetActiveUniform"),
            get_active_uniform_block_name_p: load(loader, "glGetActiveUniformBlockName"),
            get_active_uniform_blockiv_p: load(loader, "glGetActiveUniformBlockiv"),
            get_active_uniform_name_p: load(loader, "glGetActiveUniformName"),
            get_active_uniformsiv_p: load(loader, "glGetActiveUniformsiv"),
            get_attached_shaders_p: load(loader, "glGetAttachedShaders"),
            get_attrib_location_p: load(loader, "glGetAttribLocation"),
            get_booleani_v_p: load(loader, "glGetBooleaniv"),
            get_booleanv_p: load(loader, "glGetBooleanv"),
            get_buffer_parameteri64v_p: load(loader, "glGetBufferParameteri64v"),
            get_buffer_parameteriv_p: load(loader, "glGetBufferParameteriv"),
            get_buffer_pointerv_p: load(loader, "glGetBufferPointerv"),
            get_buffer_sub_data_p: load(loader, "glGetBufferSubData"),
            get_clip_planef_p: load(loader, "glGetClipPlanef"),
            get_clip_planex_p: load(loader, "glGetClipPlanex"),
            get_compressed_tex_image_p: load(loader, "glGetCompressedTexImage"),
            get_debug_message_log_p: load(loader, "glGetDebugMessageLog"),
            get_doublev_p: load(loader, "glGetDoublev"),
            get_error_p: load(loader, "glGetError"),
            get_fixedv_p: load(loader, "glGetFixedv"),
            get_floatv_p: load(loader, "glGetFloatv"),
            get_frag_data_index_p: load(loader, "glGetFragDataIndex"),
            get_frag_data_location_p: load(loader, "glGetFragDataLocation"),
            get_framebuffer_attachment_parameteriv_p: load(
                loader,
                "glGetFramebufferAttachmentParameteriv",
            ),
            get_framebuffer_parameteriv_p: load(loader, "glGetFramebufferParameteriv"),
            get_graphics_reset_status_p: load(loader, "glGetGraphicsResetStatus"),
            get_integer64i_v_p: load(loader, "glGetInteger64iv"),
            get_integer64v_p: load(loader, "glGetInteger64v"),
            get_integeri_v_p: load(loader, "glGetIntegeriv"),
            get_integerv_p: load(loader, "glGetIntegerv"),
            get_internalformativ_p: load(loader, "glGetInternalformativ"),
            get_lightfv_p: load(loader, "glGetLightfv"),
            get_lightxv_p: load(loader, "glGetLightxv"),
            get_materialfv_p: load(loader, "glGetMaterialfv"),
            get_materialxv_p: load(loader, "glGetMaterialxv"),
            get_multisamplefv_p: load(loader, "glGetMultisamplefv"),
            get_object_label_p: load(loader, "glGetObjectLabel"),
            get_object_ptr_label_p: load(loader, "glGetObjectPtrLabel"),
            get_pointerv_p: load(loader, "glGetPointerv"),
            get_program_binary_p: load(loader, "glGetProgramBinary"),
            get_program_info_log_p: load(loader, "glGetProgramInfoLog"),
            get_program_interfaceiv_p: load(loader, "glGetProgramInterfaceiv"),
            get_program_pipeline_info_log_p: load(loader, "glGetProgramPipelineInfoLog"),
            get_program_pipelineiv_p: load(loader, "glGetProgramPipelineiv"),
            get_program_resource_index_p: load(loader, "glGetProgramResourceIndex"),
            get_program_resource_location_p: load(loader, "glGetProgramResourceLocation"),
            get_program_resource_name_p: load(loader, "glGetProgramResourceName"),
            get_program_resourceiv_p: load(loader, "glGetProgramResourceiv"),
            get_programiv_p: load(loader, "glGetProgramiv"),
            get_query_objecti64v_p: load(loader, "glGetQueryObjecti64v"),
            get_query_objectiv_p: load(loader, "glGetQueryObjectiv"),
            get_query_objectui64v_p: load(loader, "glGetQueryObjectui64v"),
            get_query_objectuiv_p: load(loader, "glGetQueryObjectuiv"),
            get_queryiv_p: load(loader, "glGetQueryiv"),
            get_renderbuffer_parameteriv_p: load(loader, "glGetRenderbufferParameteriv"),
            get_sampler_parameter_iiv_p: load(loader, "glGetSamplerParameterIiv"),
            get_sampler_parameter_iuiv_p: load(loader, "glGetSamplerParameterIuiv"),
            get_sampler_parameterfv_p: load(loader, "glGetSamplerParameterfv"),
            get_sampler_parameteriv_p: load(loader, "glGetSamplerParameteriv"),
            get_shader_info_log_p: load(loader, "glGetShaderInfoLog"),
            get_shader_precision_format_p: load(loader, "glGetShaderPrecisionFormat"),
            get_shader_source_p: load(loader, "glGetShaderSource"),
            get_shaderiv_p: load(loader, "glGetShaderiv"),
            get_string_p: load(loader, "glGetString"),
            get_stringi_p: load(loader, "glGetStringi"),
            get_synciv_p: load(loader, "glGetSynciv"),
            get_tex_envfv_p: load(loader, "glGetTexEnvfv"),
            get_tex_enviv_p: load(loader, "glGetTexEnviv"),
            get_tex_envxv_p: load(loader, "glGetTexEnvxv"),
            get_tex_image_p: load(loader, "glGetTexImage"),
            get_tex_level_parameterfv_p: load(loader, "glGetTexLevelParameterfv"),
            get_tex_level_parameteriv_p: load(loader, "glGetTexLevelParameteriv"),
            get_tex_parameter_iiv_p: load(loader, "glGetTexParameterIiv"),
            get_tex_parameter_iuiv_p: load(loader, "glGetTexParameterIuiv"),
            get_tex_parameterfv_p: load(loader, "glGetTexParameterfv"),
            get_tex_parameteriv_p: load(loader, "glGetTexParameteriv"),
            get_tex_parameterxv_p: load(loader, "glGetTexParameterxv"),
            get_transform_feedback_varying_p: load(loader, "glGetTransformFeedbackVarying"),
            get_uniform_block_index_p: load(loader, "glGetUniformBlockIndex"),
            get_uniform_indices_p: load(loader, "glGetUniformIndices"),
            get_uniform_location_p: load(loader, "glGetUniformLocation"),
            get_uniformfv_p: load(loader, "glGetUniformfv"),
            get_uniformiv_p: load(loader, "glGetUniformiv"),
            get_uniformuiv_p: load(loader, "glGetUniformuiv"),
            get_vertex_attrib_iiv_p: load(loader, "glGetVertexAttribIiv"),
            get_vertex_attrib_iuiv_p: load(loader, "glGetVertexAttribIuiv"),
            get_vertex_attrib_pointerv_p: load(loader, "glGetVertexAttribPointerv"),
            get_vertex_attribdv_p: load(loader, "glGetVertexAttribdv"),
            get_vertex_attribfv_p: load(loader, "glGetVertexAttribfv"),
            get_vertex_attribiv_p: load(loader, "glGetVertexAttribiv"),
            getn_uniformfv_p: load(loader, "glGetnUniformfv"),
            getn_uniformiv_p: load(loader, "glGetnUniformiv"),
            getn_uniformuiv_p: load(loader, "glGetnUniformuiv"),
            hint_p: load(loader, "glHint"),
            invalidate_framebuffer_p: load(loader, "glInvalidateFramebuffer"),
            invalidate_sub_framebuffer_p: load(loader, "glInvalidateSubFramebuffer"),
            is_buffer_p: load(loader, "glIsBuffer"),
            is_enabled_p: load(loader, "glIsEnabled"),
            is_enabledi_p: load(loader, "glIsEnabledi"),
            is_framebuffer_p: load(loader, "glIsFramebuffer"),
            is_program_p: load(loader, "glIsProgram"),
            is_program_pipeline_p: load(loader, "glIsProgramPipeline"),
            is_query_p: load(loader, "glIsQuery"),
            is_renderbuffer_p: load(loader, "glIsRenderbuffer"),
            is_sampler_p: load(loader, "glIsSampler"),
            is_shader_p: load(loader, "glIsShader"),
            is_sync_p: load(loader, "glIsSync"),
            is_texture_p: load(loader, "glIsTexture"),
            is_transform_feedback_p: load(loader, "glIsTransformFeedback"),
            is_vertex_array_p: load(loader, "glIsVertexArray"),
            light_modelf_p: load(loader, "glLightModelf"),
            light_modelfv_p: load(loader, "glLightModelfv"),
            light_modelx_p: load(loader, "glLightModelx"),
            light_modelxv_p: load(loader, "glLightModelxv"),
            lightf_p: load(loader, "glLightf"),
            lightfv_p: load(loader, "glLightfv"),
            lightx_p: load(loader, "glLightx"),
            lightxv_p: load(loader, "glLightxv"),
            line_width_p: load(loader, "glLineWidth"),
            line_widthx_p: load(loader, "glLineWidthx"),
            link_program_p: load(loader, "glLinkProgram"),
            load_identity_p: load(loader, "glLoadIdentity"),
            load_matrixf_p: load(loader, "glLoadMatrixf"),
            load_matrixx_p: load(loader, "glLoadMatrixx"),
            logic_op_p: load(loader, "glLogicOp"),
            map_buffer_p: load(loader, "glMapBuffer"),
            map_buffer_range_p: load(loader, "glMapBufferRange"),
            materialf_p: load(loader, "glMaterialf"),
            materialfv_p: load(loader, "glMaterialfv"),
            materialx_p: load(loader, "glMaterialx"),
            materialxv_p: load(loader, "glMaterialxv"),
            matrix_mode_p: load(loader, "glMatrixMode"),
            memory_barrier_p: load(loader, "glMemoryBarrier"),
            memory_barrier_by_region_p: load(loader, "glMemoryBarrierByRegion"),
            min_sample_shading_p: load(loader, "glMinSampleShading"),
            mult_matrixf_p: load(loader, "glMultMatrixf"),
            mult_matrixx_p: load(loader, "glMultMatrixx"),
            multi_draw_arrays_p: load(loader, "glMultiDrawArrays"),
            multi_draw_elements_p: load(loader, "glMultiDrawElements"),
            multi_draw_elements_base_vertex_p: load(loader, "glMultiDrawElementsBaseVertex"),
            multi_tex_coord4f_p: load(loader, "glMultiTexCoord4f"),
            multi_tex_coord4x_p: load(loader, "glMultiTexCoord4x"),
            multi_tex_coord_p1ui_p: load(loader, "glMultiTexCoordP1ui"),
            multi_tex_coord_p1uiv_p: load(loader, "glMultiTexCoordP1uiv"),
            multi_tex_coord_p2ui_p: load(loader, "glMultiTexCoordP2ui"),
            multi_tex_coord_p2uiv_p: load(loader, "glMultiTexCoordP2uiv"),
            multi_tex_coord_p3ui_p: load(loader, "glMultiTexCoordP3ui"),
            multi_tex_coord_p3uiv_p: load(loader, "glMultiTexCoordP3uiv"),
            multi_tex_coord_p4ui_p: load(loader, "glMultiTexCoordP4ui"),
            multi_tex_coord_p4uiv_p: load(loader, "glMultiTexCoordP4uiv"),
            normal3f_p: load(loader, "glNormal3f"),
            normal3x_p: load(loader, "glNormal3x"),
            normal_p3ui_p: load(loader, "glNormalP3ui"),
            normal_p3uiv_p: load(loader, "glNormalP3uiv"),
            normal_pointer_p: load(loader, "glNormalPointer"),
            object_label_p: load(loader, "glObjectLabel"),
            object_ptr_label_p: load(loader, "glObjectPtrLabel"),
            orthof_p: load(loader, "glOrthof"),
            orthox_p: load(loader, "glOrthox"),
            patch_parameteri_p: load(loader, "glPatchParameteri"),
            pause_transform_feedback_p: load(loader, "glPauseTransformFeedback"),
            pixel_storef_p: load(loader, "glPixelStoref"),
            pixel_storei_p: load(loader, "glPixelStorei"),
            point_parameterf_p: load(loader, "glPointParameterf"),
            point_parameterfv_p: load(loader, "glPointParameterfv"),
            point_parameteri_p: load(loader, "glPointParameteri"),
            point_parameteriv_p: load(loader, "glPointParameteriv"),
            point_parameterx_p: load(loader, "glPointParameterx"),
            point_parameterxv_p: load(loader, "glPointParameterxv"),
            point_size_p: load(loader, "glPointSize"),
            point_sizex_p: load(loader, "glPointSizex"),
            polygon_mode_p: load(loader, "glPolygonMode"),
            polygon_offset_p: load(loader, "glPolygonOffset"),
            polygon_offsetx_p: load(loader, "glPolygonOffsetx"),
            pop_debug_group_p: load(loader, "glPopDebugGroup"),
            pop_matrix_p: load(loader, "glPopMatrix"),
            primitive_bounding_box_p: load(loader, "glPrimitiveBoundingBox"),
            primitive_restart_index_p: load(loader, "glPrimitiveRestartIndex"),
            program_binary_p: load(loader, "glProgramBinary"),
            program_parameteri_p: load(loader, "glProgramParameteri"),
            program_uniform1f_p: load(loader, "glProgramUniform1f"),
            program_uniform1fv_p: load(loader, "glProgramUniform1fv"),
            program_uniform1i_p: load(loader, "glProgramUniform1i"),
            program_uniform1iv_p: load(loader, "glProgramUniform1iv"),
            program_uniform1ui_p: load(loader, "glProgramUniform1ui"),
            program_uniform1uiv_p: load(loader, "glProgramUniform1uiv"),
            program_uniform2f_p: load(loader, "glProgramUniform2f"),
            program_uniform2fv_p: load(loader, "glProgramUniform2fv"),
            program_uniform2i_p: load(loader, "glProgramUniform2i"),
            program_uniform2iv_p: load(loader, "glProgramUniform2iv"),
            program_uniform2ui_p: load(loader, "glProgramUniform2ui"),
            program_uniform2uiv_p: load(loader, "glProgramUniform2uiv"),
            program_uniform3f_p: load(loader, "glProgramUniform3f"),
            program_uniform3fv_p: load(loader, "glProgramUniform3fv"),
            program_uniform3i_p: load(loader, "glProgramUniform3i"),
            program_uniform3iv_p: load(loader, "glProgramUniform3iv"),
            program_uniform3ui_p: load(loader, "glProgramUniform3ui"),
            program_uniform3uiv_p: load(loader, "glProgramUniform3uiv"),
            program_uniform4f_p: load(loader, "glProgramUniform4f"),
            program_uniform4fv_p: load(loader, "glProgramUniform4fv"),
            program_uniform4i_p: load(loader, "glProgramUniform4i"),
            program_uniform4iv_p: load(loader, "glProgramUniform4iv"),
            program_uniform4ui_p: load(loader, "glProgramUniform4ui"),
            program_uniform4uiv_p: load(loader, "glProgramUniform4uiv"),
            program_uniform_matrix2fv_p: load(loader, "glProgramUniformMatrix2fv"),
            program_uniform_matrix2x3fv_p: load(loader, "glProgramUniformMatrix2x3fv"),
            program_uniform_matrix2x4fv_p: load(loader, "glProgramUniformMatrix2x4fv"),
            program_uniform_matrix3fv_p: load(loader, "glProgramUniformMatrix3fv"),
            program_uniform_matrix3x2fv_p: load(loader, "glProgramUniformMatrix3x2fv"),
            program_uniform_matrix3x4fv_p: load(loader, "glProgramUniformMatrix3x4fv"),
            program_uniform_matrix4fv_p: load(loader, "glProgramUniformMatrix4fv"),
            program_uniform_matrix4x2fv_p: load(loader, "glProgramUniformMatrix4x2fv"),
            program_uniform_matrix4x3fv_p: load(loader, "glProgramUniformMatrix4x3fv"),
            provoking_vertex_p: load(loader, "glProvokingVertex"),
            push_debug_group_p: load(loader, "glPushDebugGroup"),
            push_matrix_p: load(loader, "glPushMatrix"),
            query_counter_p: load(loader, "glQueryCounter"),
            read_buffer_p: load(loader, "glReadBuffer"),
            read_pixels_p: load(loader, "glReadPixels"),
            readn_pixels_p: load(loader, "glReadnPixels"),
            release_shader_compiler_p: load(loader, "glReleaseShaderCompiler"),
            renderbuffer_storage_p: load(loader, "glRenderbufferStorage"),
            renderbuffer_storage_multisample_p: load(loader, "glRenderbufferStorageMultisample"),
            resume_transform_feedback_p: load(loader, "glResumeTransformFeedback"),
            rotatef_p: load(loader, "glRotatef"),
            rotatex_p: load(loader, "glRotatex"),
            sample_coverage_p: load(loader, "glSampleCoverage"),
            sample_coveragex_p: load(loader, "glSampleCoveragex"),
            sample_maski_p: load(loader, "glSampleMaski"),
            sampler_parameter_iiv_p: load(loader, "glSamplerParameterIiv"),
            sampler_parameter_iuiv_p: load(loader, "glSamplerParameterIuiv"),
            sampler_parameterf_p: load(loader, "glSamplerParameterf"),
            sampler_parameterfv_p: load(loader, "glSamplerParameterfv"),
            sampler_parameteri_p: load(loader, "glSamplerParameteri"),
            sampler_parameteriv_p: load(loader, "glSamplerParameteriv"),
            scalef_p: load(loader, "glScalef"),
            scalex_p: load(loader, "glScalex"),
            scissor_p: load(loader, "glScissor"),
            secondary_color_p3ui_p: load(loader, "glSecondaryColorP3ui"),
            secondary_color_p3uiv_p: load(loader, "glSecondaryColorP3uiv"),
            shade_model_p: load(loader, "glShadeModel"),
            shader_binary_p: load(loader, "glShaderBinary"),
            shader_source_p: load(loader, "glShaderSource"),
            stencil_func_p: load(loader, "glStencilFunc"),
            stencil_func_separate_p: load(loader, "glStencilFuncSeparate"),
            stencil_mask_p: load(loader, "glStencilMask"),
            stencil_mask_separate_p: load(loader, "glStencilMaskSeparate"),
            stencil_op_p: load(loader, "glStencilOp"),
            stencil_op_separate_p: load(loader, "glStencilOpSeparate"),
            tex_buffer_p: load(loader, "glTexBuffer"),
            tex_buffer_range_p: load(loader, "glTexBufferRange"),
            tex_coord_p1ui_p: load(loader, "glTexCoordP1ui"),
            tex_coord_p1uiv_p: load(loader, "glTexCoordP1uiv"),
            tex_coord_p2ui_p: load(loader, "glTexCoordP2ui"),
            tex_coord_p2uiv_p: load(loader, "glTexCoordP2uiv"),
            tex_coord_p3ui_p: load(loader, "glTexCoordP3ui"),
            tex_coord_p3uiv_p: load(loader, "glTexCoordP3uiv"),
            tex_coord_p4ui_p: load(loader, "glTexCoordP4ui"),
            tex_coord_p4uiv_p: load(loader, "glTexCoordP4uiv"),
            tex_coord_pointer_p: load(loader, "glTexCoordPointer"),
            tex_envf_p: load(loader, "glTexEnvf"),
            tex_envfv_p: load(loader, "glTexEnvfv"),
            tex_envi_p: load(loader, "glTexEnvi"),
            tex_enviv_p: load(loader, "glTexEnviv"),
            tex_envx_p: load(loader, "glTexEnvx"),
            tex_envxv_p: load(loader, "glTexEnvxv"),
            tex_image1_d_p: load(loader, "glTexImage1D"),
            tex_image2_d_p: load(loader, "glTexImage2D"),
            tex_image2_d_multisample_p: load(loader, "glTexImage2DMultisample"),
            tex_image3_d_p: load(loader, "glTexImage3D"),
            tex_image3_d_multisample_p: load(loader, "glTexImage3DMultisample"),
            tex_parameter_iiv_p: load(loader, "glTexParameterIiv"),
            tex_parameter_iuiv_p: load(loader, "glTexParameterIuiv"),
            tex_parameterf_p: load(loader, "glTexParameterf"),
            tex_parameterfv_p: load(loader, "glTexParameterfv"),
            tex_parameteri_p: load(loader, "glTexParameteri"),
            tex_parameteriv_p: load(loader, "glTexParameteriv"),
            tex_parameterx_p: load(loader, "glTexParameterx"),
            tex_parameterxv_p: load(loader, "glTexParameterxv"),
            tex_storage2_d_p: load(loader, "glTexStorage2D"),
            tex_storage2_d_multisample_p: load(loader, "glTexStorage2DMultisample"),
            tex_storage3_d_p: load(loader, "glTexStorage3D"),
            tex_storage3_d_multisample_p: load(loader, "glTexStorage3DMultisample"),
            tex_sub_image1_d_p: load(loader, "glTexSubImage1D"),
            tex_sub_image2_d_p: load(loader, "glTexSubImage2D"),
            tex_sub_image3_d_p: load(loader, "glTexSubImage3D"),
            transform_feedback_varyings_p: load(loader, "glTransformFeedbackVaryings"),
            translatef_p: load(loader, "glTranslatef"),
            translatex_p: load(loader, "glTranslatex"),
            uniform1f_p: load(loader, "glUniform1f"),
            uniform1fv_p: load(loader, "glUniform1fv"),
            uniform1i_p: load(loader, "glUniform1i"),
            uniform1iv_p: load(loader, "glUniform1iv"),
            uniform1ui_p: load(loader, "glUniform1ui"),
            uniform1uiv_p: load(loader, "glUniform1uiv"),
            uniform2f_p: load(loader, "glUniform2f"),
            uniform2fv_p: load(loader, "glUniform2fv"),
            uniform2i_p: load(loader, "glUniform2i"),
            uniform2iv_p: load(loader, "glUniform2iv"),
            uniform2ui_p: load(loader, "glUniform2ui"),
            uniform2uiv_p: load(loader, "glUniform2uiv"),
            uniform3f_p: load(loader, "glUniform3f"),
            uniform3fv_p: load(loader, "glUniform3fv"),
            uniform3i_p: load(loader, "glUniform3i"),
            uniform3iv_p: load(loader, "glUniform3iv"),
            uniform3ui_p: load(loader, "glUniform3ui"),
            uniform3uiv_p: load(loader, "glUniform3uiv"),
            uniform4f_p: load(loader, "glUniform4f"),
            uniform4fv_p: load(loader, "glUniform4fv"),
            uniform4i_p: load(loader, "glUniform4i"),
            uniform4iv_p: load(loader, "glUniform4iv"),
            uniform4ui_p: load(loader, "glUniform4ui"),
            uniform4uiv_p: load(loader, "glUniform4uiv"),
            uniform_block_binding_p: load(loader, "glUniformBlockBinding"),
            uniform_matrix2fv_p: load(loader, "glUniformMatrix2fv"),
            uniform_matrix2x3fv_p: load(loader, "glUniformMatrix2x3fv"),
            uniform_matrix2x4fv_p: load(loader, "glUniformMatrix2x4fv"),
            uniform_matrix3fv_p: load(loader, "glUniformMatrix3fv"),
            uniform_matrix3x2fv_p: load(loader, "glUniformMatrix3x2fv"),
            uniform_matrix3x4fv_p: load(loader, "glUniformMatrix3x4fv"),
            uniform_matrix4fv_p: load(loader, "glUniformMatrix4fv"),
            uniform_matrix4x2fv_p: load(loader, "glUniformMatrix4x2fv"),
            uniform_matrix4x3fv_p: load(loader, "glUniformMatrix4x3fv"),
            unmap_buffer_p: load(loader, "glUnmapBuffer"),
            use_program_p: load(loader, "glUseProgram"),
            use_program_stages_p: load(loader, "glUseProgramStages"),
            validate_program_p: load(loader, "glValidateProgram"),
            validate_program_pipeline_p: load(loader, "glValidateProgramPipeline"),
            vertex_attrib1d_p: load(loader, "glVertexAttrib1d"),
            vertex_attrib1dv_p: load(loader, "glVertexAttrib1dv"),
            vertex_attrib1f_p: load(loader, "glVertexAttrib1f"),
            vertex_attrib1fv_p: load(loader, "glVertexAttrib1fv"),
            vertex_attrib1s_p: load(loader, "glVertexAttrib1s"),
            vertex_attrib1sv_p: load(loader, "glVertexAttrib1sv"),
            vertex_attrib2d_p: load(loader, "glVertexAttrib2d"),
            vertex_attrib2dv_p: load(loader, "glVertexAttrib2dv"),
            vertex_attrib2f_p: load(loader, "glVertexAttrib2f"),
            vertex_attrib2fv_p: load(loader, "glVertexAttrib2fv"),
            vertex_attrib2s_p: load(loader, "glVertexAttrib2s"),
            vertex_attrib2sv_p: load(loader, "glVertexAttrib2sv"),
            vertex_attrib3d_p: load(loader, "glVertexAttrib3d"),
            vertex_attrib3dv_p: load(loader, "glVertexAttrib3dv"),
            vertex_attrib3f_p: load(loader, "glVertexAttrib3f"),
            vertex_attrib3fv_p: load(loader, "glVertexAttrib3fv"),
            vertex_attrib3s_p: load(loader, "glVertexAttrib3s"),
            vertex_attrib3sv_p: load(loader, "glVertexAttrib3sv"),
            vertex_attrib4_nbv_p: load(loader, "glVertexAttrib4Nbv"),
            vertex_attrib4_niv_p: load(loader, "glVertexAttrib4Niv"),
            vertex_attrib4_nsv_p: load(loader, "glVertexAttrib4Nsv"),
            vertex_attrib4_nub_p: load(loader, "glVertexAttrib4Nub"),
            vertex_attrib4_nubv_p: load(loader, "glVertexAttrib4Nubv"),
            vertex_attrib4_nuiv_p: load(loader, "glVertexAttrib4Nuiv"),
            vertex_attrib4_nusv_p: load(loader, "glVertexAttrib4Nusv"),
            vertex_attrib4bv_p: load(loader, "glVertexAttrib4bv"),
            vertex_attrib4d_p: load(loader, "glVertexAttrib4d"),
            vertex_attrib4dv_p: load(loader, "glVertexAttrib4dv"),
            vertex_attrib4f_p: load(loader, "glVertexAttrib4f"),
            vertex_attrib4fv_p: load(loader, "glVertexAttrib4fv"),
            vertex_attrib4iv_p: load(loader, "glVertexAttrib4iv"),
            vertex_attrib4s_p: load(loader, "glVertexAttrib4s"),
            vertex_attrib4sv_p: load(loader, "glVertexAttrib4sv"),
            vertex_attrib4ubv_p: load(loader, "glVertexAttrib4ubv"),
            vertex_attrib4uiv_p: load(loader, "glVertexAttrib4uiv"),
            vertex_attrib4usv_p: load(loader, "glVertexAttrib4usv"),
            vertex_attrib_binding_p: load(loader, "glVertexAttribBinding"),
            vertex_attrib_divisor_p: load(loader, "glVertexAttribDivisor"),
            vertex_attrib_format_p: load(loader, "glVertexAttribFormat"),
            vertex_attrib_i1i_p: load(loader, "glVertexAttribI1i"),
            vertex_attrib_i1iv_p: load(loader, "glVertexAttribI1iv"),
            vertex_attrib_i1ui_p: load(loader, "glVertexAttribI1ui"),
            vertex_attrib_i1uiv_p: load(loader, "glVertexAttribI1uiv"),
            vertex_attrib_i2i_p: load(loader, "glVertexAttribI2i"),
            vertex_attrib_i2iv_p: load(loader, "glVertexAttribI2iv"),
            vertex_attrib_i2ui_p: load(loader, "glVertexAttribI2ui"),
            vertex_attrib_i2uiv_p: load(loader, "glVertexAttribI2uiv"),
            vertex_attrib_i3i_p: load(loader, "glVertexAttribI3i"),
            vertex_attrib_i3iv_p: load(loader, "glVertexAttribI3iv"),
            vertex_attrib_i3ui_p: load(loader, "glVertexAttribI3ui"),
            vertex_attrib_i3uiv_p: load(loader, "glVertexAttribI3uiv"),
            vertex_attrib_i4bv_p: load(loader, "glVertexAttribI4bv"),
            vertex_attrib_i4i_p: load(loader, "glVertexAttribI4i"),
            vertex_attrib_i4iv_p: load(loader, "glVertexAttribI4iv"),
            vertex_attrib_i4sv_p: load(loader, "glVertexAttribI4sv"),
            vertex_attrib_i4ubv_p: load(loader, "glVertexAttribI4ubv"),
            vertex_attrib_i4ui_p: load(loader, "glVertexAttribI4ui"),
            vertex_attrib_i4uiv_p: load(loader, "glVertexAttribI4uiv"),
            vertex_attrib_i4usv_p: load(loader, "glVertexAttribI4usv"),
            vertex_attrib_i_format_p: load(loader, "glVertexAttribIFormat"),
            vertex_attrib_i_pointer_p: load(loader, "glVertexAttribIPointer"),
            vertex_attrib_p1ui_p: load(loader, "glVertexAttribP1ui"),
            vertex_attrib_p1uiv_p: load(loader, "glVertexAttribP1uiv"),
            vertex_attrib_p2ui_p: load(loader, "glVertexAttribP2ui"),
            vertex_attrib_p2uiv_p: load(loader, "glVertexAttribP2uiv"),
            vertex_attrib_p3ui_p: load(loader, "glVertexAttribP3ui"),
            vertex_attrib_p3uiv_p: load(loader, "glVertexAttribP3uiv"),
            vertex_attrib_p4ui_p: load(loader, "glVertexAttribP4ui"),
            vertex_attrib_p4uiv_p: load(loader, "glVertexAttribP4uiv"),
            vertex_attrib_pointer_p: load(loader, "glVertexAttribPointer"),
            vertex_binding_divisor_p: load(loader, "glVertexBindingDivisor"),
            vertex_p2ui_p: load(loader, "glVertexP2ui"),
            vertex_p2uiv_p: load(loader, "glVertexP2uiv"),
            vertex_p3ui_p: load(loader, "glVertexP3ui"),
            vertex_p3uiv_p: load(loader, "glVertexP3uiv"),
            vertex_p4ui_p: load(loader, "glVertexP4ui"),
            vertex_p4uiv_p: load(loader, "glVertexP4uiv"),
            vertex_pointer_p: load(loader, "glVertexPointer"),
            viewport_p: load(loader, "glViewport"),
            wait_sync_p: load(loader, "glWaitSync"),
        }
    }
    pub fn active_shader_program(&self, pipeline: c_uint, program: c_uint) {
        (self.active_shader_program_p)(pipeline, program)
    }

    pub fn active_texture(&self, texture: c_uint) {
        (self.active_texture_p)(texture)
    }

    pub fn alpha_func(&self, func: c_uint, ref_: c_float) {
        (self.alpha_func_p)(func, ref_)
    }

    pub fn alpha_funcx(&self, func: c_uint, ref_: c_int) {
        (self.alpha_funcx_p)(func, ref_)
    }

    pub fn attach_shader(&self, program: c_uint, shader: c_uint) {
        (self.attach_shader_p)(program, shader)
    }

    pub fn begin_conditional_render(&self, id: c_uint, mode: c_uint) {
        (self.begin_conditional_render_p)(id, mode)
    }

    pub fn begin_query(&self, target: c_uint, id: c_uint) {
        (self.begin_query_p)(target, id)
    }

    pub fn begin_transform_feedback(&self, primitive_mode: c_uint) {
        (self.begin_transform_feedback_p)(primitive_mode)
    }

    pub fn bind_attrib_location(&self, program: c_uint, index: c_uint, name: *const c_char) {
        (self.bind_attrib_location_p)(program, index, name)
    }

    pub fn bind_buffer(&self, target: c_uint, buffer: c_uint) {
        (self.bind_buffer_p)(target, buffer)
    }

    pub fn bind_buffer_base(&self, target: c_uint, index: c_uint, buffer: c_uint) {
        (self.bind_buffer_base_p)(target, index, buffer)
    }

    pub fn bind_buffer_range(
        &self,
        target: c_uint,
        index: c_uint,
        buffer: c_uint,
        offset: isize,
        size: isize,
    ) {
        (self.bind_buffer_range_p)(target, index, buffer, offset, size)
    }

    pub fn bind_frag_data_location(&self, program: c_uint, color: c_uint, name: *const c_char) {
        (self.bind_frag_data_location_p)(program, color, name)
    }

    pub fn bind_frag_data_location_indexed(
        &self,
        program: c_uint,
        color_number: c_uint,
        index: c_uint,
        name: *const c_char,
    ) {
        (self.bind_frag_data_location_indexed_p)(program, color_number, index, name)
    }

    pub fn bind_framebuffer(&self, target: c_uint, framebuffer: c_uint) {
        (self.bind_framebuffer_p)(target, framebuffer)
    }

    pub fn bind_image_texture(
        &self,
        unit: c_uint,
        texture: c_uint,
        level: c_int,
        layered: c_uchar,
        layer: c_int,
        access: c_uint,
        format: c_uint,
    ) {
        (self.bind_image_texture_p)(unit, texture, level, layered, layer, access, format)
    }

    pub fn bind_program_pipeline(&self, pipeline: c_uint) {
        (self.bind_program_pipeline_p)(pipeline)
    }

    pub fn bind_renderbuffer(&self, target: c_uint, renderbuffer: c_uint) {
        (self.bind_renderbuffer_p)(target, renderbuffer)
    }

    pub fn bind_sampler(&self, unit: c_uint, sampler: c_uint) {
        (self.bind_sampler_p)(unit, sampler)
    }

    pub fn bind_texture(&self, target: c_uint, texture: c_uint) {
        (self.bind_texture_p)(target, texture)
    }

    pub fn bind_transform_feedback(&self, target: c_uint, id: c_uint) {
        (self.bind_transform_feedback_p)(target, id)
    }

    pub fn bind_vertex_array(&self, array: c_uint) {
        (self.bind_vertex_array_p)(array)
    }

    pub fn bind_vertex_buffer(
        &self,
        bindingindex: c_uint,
        buffer: c_uint,
        offset: isize,
        stride: c_int,
    ) {
        (self.bind_vertex_buffer_p)(bindingindex, buffer, offset, stride)
    }

    pub fn blend_barrier(&self) {
        (self.blend_barrier_p)()
    }

    pub fn blend_color(&self, red: c_float, green: c_float, blue: c_float, alpha: c_float) {
        (self.blend_color_p)(red, green, blue, alpha)
    }

    pub fn blend_equation(&self, mode: c_uint) {
        (self.blend_equation_p)(mode)
    }

    pub fn blend_equation_separate(&self, mode_r_g_b: c_uint, mode_alpha: c_uint) {
        (self.blend_equation_separate_p)(mode_r_g_b, mode_alpha)
    }

    pub fn blend_equation_separatei(&self, buf: c_uint, mode_r_g_b: c_uint, mode_alpha: c_uint) {
        (self.blend_equation_separatei_p)(buf, mode_r_g_b, mode_alpha)
    }

    pub fn blend_equationi(&self, buf: c_uint, mode: c_uint) {
        (self.blend_equationi_p)(buf, mode)
    }

    pub fn blend_func(&self, sfactor: c_uint, dfactor: c_uint) {
        (self.blend_func_p)(sfactor, dfactor)
    }

    pub fn blend_func_separate(
        &self,
        sfactor_r_g_b: c_uint,
        dfactor_r_g_b: c_uint,
        sfactor_alpha: c_uint,
        dfactor_alpha: c_uint,
    ) {
        (self.blend_func_separate_p)(sfactor_r_g_b, dfactor_r_g_b, sfactor_alpha, dfactor_alpha)
    }

    pub fn blend_func_separatei(
        &self,
        buf: c_uint,
        src_r_g_b: c_uint,
        dst_r_g_b: c_uint,
        src_alpha: c_uint,
        dst_alpha: c_uint,
    ) {
        (self.blend_func_separatei_p)(buf, src_r_g_b, dst_r_g_b, src_alpha, dst_alpha)
    }

    pub fn blend_funci(&self, buf: c_uint, src: c_uint, dst: c_uint) {
        (self.blend_funci_p)(buf, src, dst)
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: c_int,
        src_y0: c_int,
        src_x1: c_int,
        src_y1: c_int,
        dst_x0: c_int,
        dst_y0: c_int,
        dst_x1: c_int,
        dst_y1: c_int,
        mask: c_uint,
        filter: c_uint,
    ) {
        (self.blit_framebuffer_p)(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        )
    }

    pub fn buffer_data(&self, target: c_uint, size: isize, data: *const c_void, usage: c_uint) {
        (self.buffer_data_p)(target, size, data, usage)
    }

    pub fn buffer_sub_data(&self, target: c_uint, offset: isize, size: isize, data: *const c_void) {
        (self.buffer_sub_data_p)(target, offset, size, data)
    }

    pub fn check_framebuffer_status(&self, target: c_uint) -> c_uint {
        (self.check_framebuffer_status_p)(target)
    }

    pub fn clamp_color(&self, target: c_uint, clamp: c_uint) {
        (self.clamp_color_p)(target, clamp)
    }

    pub fn clear(&self, mask: c_uint) {
        (self.clear_p)(mask)
    }

    pub fn clear_bufferfi(
        &self,
        buffer: c_uint,
        drawbuffer: c_int,
        depth: c_float,
        stencil: c_int,
    ) {
        (self.clear_bufferfi_p)(buffer, drawbuffer, depth, stencil)
    }

    pub fn clear_bufferfv(&self, buffer: c_uint, drawbuffer: c_int, value: *const c_float) {
        (self.clear_bufferfv_p)(buffer, drawbuffer, value)
    }

    pub fn clear_bufferiv(&self, buffer: c_uint, drawbuffer: c_int, value: *const c_int) {
        (self.clear_bufferiv_p)(buffer, drawbuffer, value)
    }

    pub fn clear_bufferuiv(&self, buffer: c_uint, drawbuffer: c_int, value: *const c_uint) {
        (self.clear_bufferuiv_p)(buffer, drawbuffer, value)
    }

    pub fn clear_color(&self, red: c_float, green: c_float, blue: c_float, alpha: c_float) {
        (self.clear_color_p)(red, green, blue, alpha)
    }

    pub fn clear_colorx(&self, red: c_int, green: c_int, blue: c_int, alpha: c_int) {
        (self.clear_colorx_p)(red, green, blue, alpha)
    }

    pub fn clear_depth(&self, depth: c_double) {
        (self.clear_depth_p)(depth)
    }

    pub fn clear_depthf(&self, d: c_float) {
        (self.clear_depthf_p)(d)
    }

    pub fn clear_depthx(&self, depth: c_int) {
        (self.clear_depthx_p)(depth)
    }

    pub fn clear_stencil(&self, s: c_int) {
        (self.clear_stencil_p)(s)
    }

    pub fn client_active_texture(&self, texture: c_uint) {
        (self.client_active_texture_p)(texture)
    }

    pub fn client_wait_sync(&self, sync: *mut c_void, flags: c_uint, timeout: u64) -> c_uint {
        (self.client_wait_sync_p)(sync, flags, timeout)
    }

    pub fn clip_planef(&self, p: c_uint, eqn: *const c_float) {
        (self.clip_planef_p)(p, eqn)
    }

    pub fn clip_planex(&self, plane: c_uint, equation: *const c_int) {
        (self.clip_planex_p)(plane, equation)
    }

    pub fn color4f(&self, red: c_float, green: c_float, blue: c_float, alpha: c_float) {
        (self.color4f_p)(red, green, blue, alpha)
    }

    pub fn color4ub(&self, red: c_uchar, green: c_uchar, blue: c_uchar, alpha: c_uchar) {
        (self.color4ub_p)(red, green, blue, alpha)
    }

    pub fn color4x(&self, red: c_int, green: c_int, blue: c_int, alpha: c_int) {
        (self.color4x_p)(red, green, blue, alpha)
    }

    pub fn color_mask(&self, red: c_uchar, green: c_uchar, blue: c_uchar, alpha: c_uchar) {
        (self.color_mask_p)(red, green, blue, alpha)
    }

    pub fn color_maski(&self, index: c_uint, r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) {
        (self.color_maski_p)(index, r, g, b, a)
    }

    pub fn color_p3ui(&self, type_: c_uint, color: c_uint) {
        (self.color_p3ui_p)(type_, color)
    }

    pub fn color_p3uiv(&self, type_: c_uint, color: *const c_uint) {
        (self.color_p3uiv_p)(type_, color)
    }

    pub fn color_p4ui(&self, type_: c_uint, color: c_uint) {
        (self.color_p4ui_p)(type_, color)
    }

    pub fn color_p4uiv(&self, type_: c_uint, color: *const c_uint) {
        (self.color_p4uiv_p)(type_, color)
    }

    pub fn color_pointer(&self, size: c_int, type_: c_uint, stride: c_int, pointer: *const c_void) {
        (self.color_pointer_p)(size, type_, stride, pointer)
    }

    pub fn compile_shader(&self, shader: c_uint) {
        (self.compile_shader_p)(shader)
    }

    pub fn compressed_tex_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_uint,
        width: c_int,
        border: c_int,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_image1_d_p)(
            target,
            level,
            internalformat,
            width,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        border: c_int,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_image2_d_p)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        depth: c_int,
        border: c_int,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_image3_d_p)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            image_size,
            data,
        )
    }

    pub fn compressed_tex_sub_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        width: c_int,
        format: c_uint,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image1_d_p)(
            target, level, xoffset, width, format, image_size, data,
        )
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        width: c_int,
        height: c_int,
        format: c_uint,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image2_d_p)(
            target, level, xoffset, yoffset, width, height, format, image_size, data,
        )
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        zoffset: c_int,
        width: c_int,
        height: c_int,
        depth: c_int,
        format: c_uint,
        image_size: c_int,
        data: *const c_void,
    ) {
        (self.compressed_tex_sub_image3_d_p)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, image_size,
            data,
        )
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: c_uint,
        write_target: c_uint,
        read_offset: isize,
        write_offset: isize,
        size: isize,
    ) {
        (self.copy_buffer_sub_data_p)(read_target, write_target, read_offset, write_offset, size)
    }

    pub fn copy_image_sub_data(
        &self,
        src_name: c_uint,
        src_target: c_uint,
        src_level: c_int,
        src_x: c_int,
        src_y: c_int,
        src_z: c_int,
        dst_name: c_uint,
        dst_target: c_uint,
        dst_level: c_int,
        dst_x: c_int,
        dst_y: c_int,
        dst_z: c_int,
        src_width: c_int,
        src_height: c_int,
        src_depth: c_int,
    ) {
        (self.copy_image_sub_data_p)(
            src_name, src_target, src_level, src_x, src_y, src_z, dst_name, dst_target, dst_level,
            dst_x, dst_y, dst_z, src_width, src_height, src_depth,
        )
    }

    pub fn copy_tex_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_uint,
        x: c_int,
        y: c_int,
        width: c_int,
        border: c_int,
    ) {
        (self.copy_tex_image1_d_p)(target, level, internalformat, x, y, width, border)
    }

    pub fn copy_tex_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_uint,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        border: c_int,
    ) {
        (self.copy_tex_image2_d_p)(target, level, internalformat, x, y, width, height, border)
    }

    pub fn copy_tex_sub_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        x: c_int,
        y: c_int,
        width: c_int,
    ) {
        (self.copy_tex_sub_image1_d_p)(target, level, xoffset, x, y, width)
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) {
        (self.copy_tex_sub_image2_d_p)(target, level, xoffset, yoffset, x, y, width, height)
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        zoffset: c_int,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) {
        (self.copy_tex_sub_image3_d_p)(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }

    pub fn create_program(&self) -> c_uint {
        (self.create_program_p)()
    }

    pub fn create_shader(&self, type_: c_uint) -> c_uint {
        (self.create_shader_p)(type_)
    }

    pub fn create_shader_programv(
        &self,
        type_: c_uint,
        count: c_int,
        strings: *const *const c_char,
    ) -> c_uint {
        (self.create_shader_programv_p)(type_, count, strings)
    }

    pub fn cull_face(&self, mode: c_uint) {
        (self.cull_face_p)(mode)
    }

    pub fn debug_message_callback(&self, callback: DebugProc, user_param: *const c_void) {
        (self.debug_message_callback_p)(callback, user_param)
    }

    pub fn debug_message_control(
        &self,
        source: c_uint,
        type_: c_uint,
        severity: c_uint,
        count: c_int,
        ids: *const c_uint,
        enabled: c_uchar,
    ) {
        (self.debug_message_control_p)(source, type_, severity, count, ids, enabled)
    }

    pub fn debug_message_insert(
        &self,
        source: c_uint,
        type_: c_uint,
        id: c_uint,
        severity: c_uint,
        length: c_int,
        buf: *const c_char,
    ) {
        (self.debug_message_insert_p)(source, type_, id, severity, length, buf)
    }

    pub fn delete_buffers(&self, n: c_int, buffers: *const c_uint) {
        (self.delete_buffers_p)(n, buffers)
    }

    pub fn delete_framebuffers(&self, n: c_int, framebuffers: *const c_uint) {
        (self.delete_framebuffers_p)(n, framebuffers)
    }

    pub fn delete_program(&self, program: c_uint) {
        (self.delete_program_p)(program)
    }

    pub fn delete_program_pipelines(&self, n: c_int, pipelines: *const c_uint) {
        (self.delete_program_pipelines_p)(n, pipelines)
    }

    pub fn delete_queries(&self, n: c_int, ids: *const c_uint) {
        (self.delete_queries_p)(n, ids)
    }

    pub fn delete_renderbuffers(&self, n: c_int, renderbuffers: *const c_uint) {
        (self.delete_renderbuffers_p)(n, renderbuffers)
    }

    pub fn delete_samplers(&self, count: c_int, samplers: *const c_uint) {
        (self.delete_samplers_p)(count, samplers)
    }

    pub fn delete_shader(&self, shader: c_uint) {
        (self.delete_shader_p)(shader)
    }

    pub fn delete_sync(&self, sync: *mut c_void) {
        (self.delete_sync_p)(sync)
    }

    pub fn delete_textures(&self, n: c_int, textures: *const c_uint) {
        (self.delete_textures_p)(n, textures)
    }

    pub fn delete_transform_feedbacks(&self, n: c_int, ids: *const c_uint) {
        (self.delete_transform_feedbacks_p)(n, ids)
    }

    pub fn delete_vertex_arrays(&self, n: c_int, arrays: *const c_uint) {
        (self.delete_vertex_arrays_p)(n, arrays)
    }

    pub fn depth_func(&self, func: c_uint) {
        (self.depth_func_p)(func)
    }

    pub fn depth_mask(&self, flag: c_uchar) {
        (self.depth_mask_p)(flag)
    }

    pub fn depth_range(&self, n: c_double, f: c_double) {
        (self.depth_range_p)(n, f)
    }

    pub fn depth_rangef(&self, n: c_float, f: c_float) {
        (self.depth_rangef_p)(n, f)
    }

    pub fn depth_rangex(&self, n: c_int, f: c_int) {
        (self.depth_rangex_p)(n, f)
    }

    pub fn detach_shader(&self, program: c_uint, shader: c_uint) {
        (self.detach_shader_p)(program, shader)
    }

    pub fn disable(&self, cap: c_uint) {
        (self.disable_p)(cap)
    }

    pub fn disable_client_state(&self, array: c_uint) {
        (self.disable_client_state_p)(array)
    }

    pub fn disable_vertex_attrib_array(&self, index: c_uint) {
        (self.disable_vertex_attrib_array_p)(index)
    }

    pub fn disablei(&self, target: c_uint, index: c_uint) {
        (self.disablei_p)(target, index)
    }

    pub fn dispatch_compute(
        &self,
        num_groups_x: c_uint,
        num_groups_y: c_uint,
        num_groups_z: c_uint,
    ) {
        (self.dispatch_compute_p)(num_groups_x, num_groups_y, num_groups_z)
    }

    pub fn dispatch_compute_indirect(&self, indirect: isize) {
        (self.dispatch_compute_indirect_p)(indirect)
    }

    pub fn draw_arrays(&self, mode: c_uint, first: c_int, count: c_int) {
        (self.draw_arrays_p)(mode, first, count)
    }

    pub fn draw_arrays_indirect(&self, mode: c_uint, indirect: *const c_void) {
        (self.draw_arrays_indirect_p)(mode, indirect)
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: c_uint,
        first: c_int,
        count: c_int,
        instancecount: c_int,
    ) {
        (self.draw_arrays_instanced_p)(mode, first, count, instancecount)
    }

    pub fn draw_buffer(&self, buf: c_uint) {
        (self.draw_buffer_p)(buf)
    }

    pub fn draw_buffers(&self, n: c_int, bufs: *const c_uint) {
        (self.draw_buffers_p)(n, bufs)
    }

    pub fn draw_elements(&self, mode: c_uint, count: c_int, type_: c_uint, indices: *const c_void) {
        (self.draw_elements_p)(mode, count, type_, indices)
    }

    pub fn draw_elements_base_vertex(
        &self,
        mode: c_uint,
        count: c_int,
        type_: c_uint,
        indices: *const c_void,
        basevertex: c_int,
    ) {
        (self.draw_elements_base_vertex_p)(mode, count, type_, indices, basevertex)
    }

    pub fn draw_elements_indirect(&self, mode: c_uint, type_: c_uint, indirect: *const c_void) {
        (self.draw_elements_indirect_p)(mode, type_, indirect)
    }

    pub fn draw_elements_instanced(
        &self,
        mode: c_uint,
        count: c_int,
        type_: c_uint,
        indices: *const c_void,
        instancecount: c_int,
    ) {
        (self.draw_elements_instanced_p)(mode, count, type_, indices, instancecount)
    }

    pub fn draw_elements_instanced_base_vertex(
        &self,
        mode: c_uint,
        count: c_int,
        type_: c_uint,
        indices: *const c_void,
        instancecount: c_int,
        basevertex: c_int,
    ) {
        (self.draw_elements_instanced_base_vertex_p)(
            mode,
            count,
            type_,
            indices,
            instancecount,
            basevertex,
        )
    }

    pub fn draw_range_elements(
        &self,
        mode: c_uint,
        start: c_uint,
        end: c_uint,
        count: c_int,
        type_: c_uint,
        indices: *const c_void,
    ) {
        (self.draw_range_elements_p)(mode, start, end, count, type_, indices)
    }

    pub fn draw_range_elements_base_vertex(
        &self,
        mode: c_uint,
        start: c_uint,
        end: c_uint,
        count: c_int,
        type_: c_uint,
        indices: *const c_void,
        basevertex: c_int,
    ) {
        (self.draw_range_elements_base_vertex_p)(
            mode, start, end, count, type_, indices, basevertex,
        )
    }

    pub fn enable(&self, cap: c_uint) {
        (self.enable_p)(cap)
    }

    pub fn enable_client_state(&self, array: c_uint) {
        (self.enable_client_state_p)(array)
    }

    pub fn enable_vertex_attrib_array(&self, index: c_uint) {
        (self.enable_vertex_attrib_array_p)(index)
    }

    pub fn enablei(&self, target: c_uint, index: c_uint) {
        (self.enablei_p)(target, index)
    }

    pub fn end_conditional_render(&self) {
        (self.end_conditional_render_p)()
    }

    pub fn end_query(&self, target: c_uint) {
        (self.end_query_p)(target)
    }

    pub fn end_transform_feedback(&self) {
        (self.end_transform_feedback_p)()
    }

    pub fn fence_sync(&self, condition: c_uint, flags: c_uint) -> *mut c_void {
        (self.fence_sync_p)(condition, flags)
    }

    pub fn finish(&self) {
        (self.finish_p)()
    }

    pub fn flush(&self) {
        (self.flush_p)()
    }

    pub fn flush_mapped_buffer_range(&self, target: c_uint, offset: isize, length: isize) {
        (self.flush_mapped_buffer_range_p)(target, offset, length)
    }

    pub fn fogf(&self, pname: c_uint, param: c_float) {
        (self.fogf_p)(pname, param)
    }

    pub fn fogfv(&self, pname: c_uint, params: *const c_float) {
        (self.fogfv_p)(pname, params)
    }

    pub fn fogx(&self, pname: c_uint, param: c_int) {
        (self.fogx_p)(pname, param)
    }

    pub fn fogxv(&self, pname: c_uint, param: *const c_int) {
        (self.fogxv_p)(pname, param)
    }

    pub fn framebuffer_parameteri(&self, target: c_uint, pname: c_uint, param: c_int) {
        (self.framebuffer_parameteri_p)(target, pname, param)
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: c_uint,
        attachment: c_uint,
        renderbuffertarget: c_uint,
        renderbuffer: c_uint,
    ) {
        (self.framebuffer_renderbuffer_p)(target, attachment, renderbuffertarget, renderbuffer)
    }

    pub fn framebuffer_texture(
        &self,
        target: c_uint,
        attachment: c_uint,
        texture: c_uint,
        level: c_int,
    ) {
        (self.framebuffer_texture_p)(target, attachment, texture, level)
    }

    pub fn framebuffer_texture1_d(
        &self,
        target: c_uint,
        attachment: c_uint,
        textarget: c_uint,
        texture: c_uint,
        level: c_int,
    ) {
        (self.framebuffer_texture1_d_p)(target, attachment, textarget, texture, level)
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: c_uint,
        attachment: c_uint,
        textarget: c_uint,
        texture: c_uint,
        level: c_int,
    ) {
        (self.framebuffer_texture2_d_p)(target, attachment, textarget, texture, level)
    }

    pub fn framebuffer_texture3_d(
        &self,
        target: c_uint,
        attachment: c_uint,
        textarget: c_uint,
        texture: c_uint,
        level: c_int,
        zoffset: c_int,
    ) {
        (self.framebuffer_texture3_d_p)(target, attachment, textarget, texture, level, zoffset)
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: c_uint,
        attachment: c_uint,
        texture: c_uint,
        level: c_int,
        layer: c_int,
    ) {
        (self.framebuffer_texture_layer_p)(target, attachment, texture, level, layer)
    }

    pub fn front_face(&self, mode: c_uint) {
        (self.front_face_p)(mode)
    }

    pub fn frustumf(&self, l: c_float, r: c_float, b: c_float, t: c_float, n: c_float, f: c_float) {
        (self.frustumf_p)(l, r, b, t, n, f)
    }

    pub fn frustumx(&self, l: c_int, r: c_int, b: c_int, t: c_int, n: c_int, f: c_int) {
        (self.frustumx_p)(l, r, b, t, n, f)
    }

    pub fn gen_buffers(&self, n: c_int, buffers: *mut c_uint) {
        (self.gen_buffers_p)(n, buffers)
    }

    pub fn gen_framebuffers(&self, n: c_int, framebuffers: *mut c_uint) {
        (self.gen_framebuffers_p)(n, framebuffers)
    }

    pub fn gen_program_pipelines(&self, n: c_int, pipelines: *mut c_uint) {
        (self.gen_program_pipelines_p)(n, pipelines)
    }

    pub fn gen_queries(&self, n: c_int, ids: *mut c_uint) {
        (self.gen_queries_p)(n, ids)
    }

    pub fn gen_renderbuffers(&self, n: c_int, renderbuffers: *mut c_uint) {
        (self.gen_renderbuffers_p)(n, renderbuffers)
    }

    pub fn gen_samplers(&self, count: c_int, samplers: *mut c_uint) {
        (self.gen_samplers_p)(count, samplers)
    }

    pub fn gen_textures(&self, n: c_int, textures: *mut c_uint) {
        (self.gen_textures_p)(n, textures)
    }

    pub fn gen_transform_feedbacks(&self, n: c_int, ids: *mut c_uint) {
        (self.gen_transform_feedbacks_p)(n, ids)
    }

    pub fn gen_vertex_arrays(&self, n: c_int, arrays: *mut c_uint) {
        (self.gen_vertex_arrays_p)(n, arrays)
    }

    pub fn generate_mipmap(&self, target: c_uint) {
        (self.generate_mipmap_p)(target)
    }

    pub fn get_active_attrib(
        &self,
        program: c_uint,
        index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        size: *mut c_int,
        type_: *mut c_uint,
        name: *mut c_char,
    ) {
        (self.get_active_attrib_p)(program, index, buf_size, length, size, type_, name)
    }

    pub fn get_active_uniform(
        &self,
        program: c_uint,
        index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        size: *mut c_int,
        type_: *mut c_uint,
        name: *mut c_char,
    ) {
        (self.get_active_uniform_p)(program, index, buf_size, length, size, type_, name)
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: c_uint,
        uniform_block_index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        uniform_block_name: *mut c_char,
    ) {
        (self.get_active_uniform_block_name_p)(
            program,
            uniform_block_index,
            buf_size,
            length,
            uniform_block_name,
        )
    }

    pub fn get_active_uniform_blockiv(
        &self,
        program: c_uint,
        uniform_block_index: c_uint,
        pname: c_uint,
        params: *mut c_int,
    ) {
        (self.get_active_uniform_blockiv_p)(program, uniform_block_index, pname, params)
    }

    pub fn get_active_uniform_name(
        &self,
        program: c_uint,
        uniform_index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        uniform_name: *mut c_char,
    ) {
        (self.get_active_uniform_name_p)(program, uniform_index, buf_size, length, uniform_name)
    }

    pub fn get_active_uniformsiv(
        &self,
        program: c_uint,
        uniform_count: c_int,
        uniform_indices: *const c_uint,
        pname: c_uint,
        params: *mut c_int,
    ) {
        (self.get_active_uniformsiv_p)(program, uniform_count, uniform_indices, pname, params)
    }

    pub fn get_attached_shaders(
        &self,
        program: c_uint,
        max_count: c_int,
        count: *mut c_int,
        shaders: *mut c_uint,
    ) {
        (self.get_attached_shaders_p)(program, max_count, count, shaders)
    }

    pub fn get_attrib_location(&self, program: c_uint, name: *const c_char) -> c_int {
        (self.get_attrib_location_p)(program, name)
    }

    pub fn get_booleani_v(&self, target: c_uint, index: c_uint, data: *mut c_uchar) {
        (self.get_booleani_v_p)(target, index, data)
    }

    pub fn get_booleanv(&self, pname: c_uint, data: *mut c_uchar) {
        (self.get_booleanv_p)(pname, data)
    }

    pub fn get_buffer_parameteri64v(&self, target: c_uint, pname: c_uint, params: *mut i64) {
        (self.get_buffer_parameteri64v_p)(target, pname, params)
    }

    pub fn get_buffer_parameteriv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_buffer_parameteriv_p)(target, pname, params)
    }

    pub fn get_buffer_pointerv(&self, target: c_uint, pname: c_uint, params: *mut *mut c_void) {
        (self.get_buffer_pointerv_p)(target, pname, params)
    }

    pub fn get_buffer_sub_data(
        &self,
        target: c_uint,
        offset: isize,
        size: isize,
        data: *mut c_void,
    ) {
        (self.get_buffer_sub_data_p)(target, offset, size, data)
    }

    pub fn get_clip_planef(&self, plane: c_uint, equation: *mut c_float) {
        (self.get_clip_planef_p)(plane, equation)
    }

    pub fn get_clip_planex(&self, plane: c_uint, equation: *mut c_int) {
        (self.get_clip_planex_p)(plane, equation)
    }

    pub fn get_compressed_tex_image(&self, target: c_uint, level: c_int, img: *mut c_void) {
        (self.get_compressed_tex_image_p)(target, level, img)
    }

    pub fn get_debug_message_log(
        &self,
        count: c_uint,
        buf_size: c_int,
        sources: *mut c_uint,
        types: *mut c_uint,
        ids: *mut c_uint,
        severities: *mut c_uint,
        lengths: *mut c_int,
        message_log: *mut c_char,
    ) -> c_uint {
        (self.get_debug_message_log_p)(
            count,
            buf_size,
            sources,
            types,
            ids,
            severities,
            lengths,
            message_log,
        )
    }

    pub fn get_doublev(&self, pname: c_uint, data: *mut c_double) {
        (self.get_doublev_p)(pname, data)
    }

    pub fn get_error(&self) -> c_uint {
        (self.get_error_p)()
    }

    pub fn get_fixedv(&self, pname: c_uint, params: *mut c_int) {
        (self.get_fixedv_p)(pname, params)
    }

    pub fn get_floatv(&self, pname: c_uint, data: *mut c_float) {
        (self.get_floatv_p)(pname, data)
    }

    pub fn get_frag_data_index(&self, program: c_uint, name: *const c_char) -> c_int {
        (self.get_frag_data_index_p)(program, name)
    }

    pub fn get_frag_data_location(&self, program: c_uint, name: *const c_char) -> c_int {
        (self.get_frag_data_location_p)(program, name)
    }

    pub fn get_framebuffer_attachment_parameteriv(
        &self,
        target: c_uint,
        attachment: c_uint,
        pname: c_uint,
        params: *mut c_int,
    ) {
        (self.get_framebuffer_attachment_parameteriv_p)(target, attachment, pname, params)
    }

    pub fn get_framebuffer_parameteriv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_framebuffer_parameteriv_p)(target, pname, params)
    }

    pub fn get_graphics_reset_status(&self) -> c_uint {
        (self.get_graphics_reset_status_p)()
    }

    pub fn get_integer64i_v(&self, target: c_uint, index: c_uint, data: *mut i64) {
        (self.get_integer64i_v_p)(target, index, data)
    }

    pub fn get_integer64v(&self, pname: c_uint, data: *mut i64) {
        (self.get_integer64v_p)(pname, data)
    }

    pub fn get_integeri_v(&self, target: c_uint, index: c_uint, data: *mut c_int) {
        (self.get_integeri_v_p)(target, index, data)
    }

    pub fn get_integerv(&self, pname: c_uint, data: *mut c_int) {
        (self.get_integerv_p)(pname, data)
    }

    pub fn get_internalformativ(
        &self,
        target: c_uint,
        internalformat: c_uint,
        pname: c_uint,
        count: c_int,
        params: *mut c_int,
    ) {
        (self.get_internalformativ_p)(target, internalformat, pname, count, params)
    }

    pub fn get_lightfv(&self, light: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_lightfv_p)(light, pname, params)
    }

    pub fn get_lightxv(&self, light: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_lightxv_p)(light, pname, params)
    }

    pub fn get_materialfv(&self, face: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_materialfv_p)(face, pname, params)
    }

    pub fn get_materialxv(&self, face: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_materialxv_p)(face, pname, params)
    }

    pub fn get_multisamplefv(&self, pname: c_uint, index: c_uint, val: *mut c_float) {
        (self.get_multisamplefv_p)(pname, index, val)
    }

    pub fn get_object_label(
        &self,
        identifier: c_uint,
        name: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        label: *mut c_char,
    ) {
        (self.get_object_label_p)(identifier, name, buf_size, length, label)
    }

    pub fn get_object_ptr_label(
        &self,
        ptr: *const c_void,
        buf_size: c_int,
        length: *mut c_int,
        label: *mut c_char,
    ) {
        (self.get_object_ptr_label_p)(ptr, buf_size, length, label)
    }

    pub fn get_pointerv(&self, pname: c_uint, params: *mut *mut c_void) {
        (self.get_pointerv_p)(pname, params)
    }

    pub fn get_program_binary(
        &self,
        program: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        binary_format: *mut c_uint,
        binary: *mut c_void,
    ) {
        (self.get_program_binary_p)(program, buf_size, length, binary_format, binary)
    }

    pub fn get_program_info_log(
        &self,
        program: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        info_log: *mut c_char,
    ) {
        (self.get_program_info_log_p)(program, buf_size, length, info_log)
    }

    pub fn get_program_interfaceiv(
        &self,
        program: c_uint,
        program_interface: c_uint,
        pname: c_uint,
        params: *mut c_int,
    ) {
        (self.get_program_interfaceiv_p)(program, program_interface, pname, params)
    }

    pub fn get_program_pipeline_info_log(
        &self,
        pipeline: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        info_log: *mut c_char,
    ) {
        (self.get_program_pipeline_info_log_p)(pipeline, buf_size, length, info_log)
    }

    pub fn get_program_pipelineiv(&self, pipeline: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_program_pipelineiv_p)(pipeline, pname, params)
    }

    pub fn get_program_resource_index(
        &self,
        program: c_uint,
        program_interface: c_uint,
        name: *const c_char,
    ) -> c_uint {
        (self.get_program_resource_index_p)(program, program_interface, name)
    }

    pub fn get_program_resource_location(
        &self,
        program: c_uint,
        program_interface: c_uint,
        name: *const c_char,
    ) -> c_int {
        (self.get_program_resource_location_p)(program, program_interface, name)
    }

    pub fn get_program_resource_name(
        &self,
        program: c_uint,
        program_interface: c_uint,
        index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        name: *mut c_char,
    ) {
        (self.get_program_resource_name_p)(
            program,
            program_interface,
            index,
            buf_size,
            length,
            name,
        )
    }

    pub fn get_program_resourceiv(
        &self,
        program: c_uint,
        program_interface: c_uint,
        index: c_uint,
        prop_count: c_int,
        props: *const c_uint,
        count: c_int,
        length: *mut c_int,
        params: *mut c_int,
    ) {
        (self.get_program_resourceiv_p)(
            program,
            program_interface,
            index,
            prop_count,
            props,
            count,
            length,
            params,
        )
    }

    pub fn get_programiv(&self, program: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_programiv_p)(program, pname, params)
    }

    pub fn get_query_objecti64v(&self, id: c_uint, pname: c_uint, params: *mut i64) {
        (self.get_query_objecti64v_p)(id, pname, params)
    }

    pub fn get_query_objectiv(&self, id: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_query_objectiv_p)(id, pname, params)
    }

    pub fn get_query_objectui64v(&self, id: c_uint, pname: c_uint, params: *mut u64) {
        (self.get_query_objectui64v_p)(id, pname, params)
    }

    pub fn get_query_objectuiv(&self, id: c_uint, pname: c_uint, params: *mut c_uint) {
        (self.get_query_objectuiv_p)(id, pname, params)
    }

    pub fn get_queryiv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_queryiv_p)(target, pname, params)
    }

    pub fn get_renderbuffer_parameteriv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_renderbuffer_parameteriv_p)(target, pname, params)
    }

    pub fn get_sampler_parameter_iiv(&self, sampler: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_sampler_parameter_iiv_p)(sampler, pname, params)
    }

    pub fn get_sampler_parameter_iuiv(&self, sampler: c_uint, pname: c_uint, params: *mut c_uint) {
        (self.get_sampler_parameter_iuiv_p)(sampler, pname, params)
    }

    pub fn get_sampler_parameterfv(&self, sampler: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_sampler_parameterfv_p)(sampler, pname, params)
    }

    pub fn get_sampler_parameteriv(&self, sampler: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_sampler_parameteriv_p)(sampler, pname, params)
    }

    pub fn get_shader_info_log(
        &self,
        shader: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        info_log: *mut c_char,
    ) {
        (self.get_shader_info_log_p)(shader, buf_size, length, info_log)
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: c_uint,
        precisiontype: c_uint,
        range: *mut c_int,
        precision: *mut c_int,
    ) {
        (self.get_shader_precision_format_p)(shadertype, precisiontype, range, precision)
    }

    pub fn get_shader_source(
        &self,
        shader: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        source: *mut c_char,
    ) {
        (self.get_shader_source_p)(shader, buf_size, length, source)
    }

    pub fn get_shaderiv(&self, shader: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_shaderiv_p)(shader, pname, params)
    }

    pub fn get_string(&self, name: c_uint) -> *const c_uchar {
        (self.get_string_p)(name)
    }

    pub fn get_stringi(&self, name: c_uint, index: c_uint) -> *const c_uchar {
        (self.get_stringi_p)(name, index)
    }

    pub fn get_synciv(
        &self,
        sync: *mut c_void,
        pname: c_uint,
        count: c_int,
        length: *mut c_int,
        values: *mut c_int,
    ) {
        (self.get_synciv_p)(sync, pname, count, length, values)
    }

    pub fn get_tex_envfv(&self, target: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_tex_envfv_p)(target, pname, params)
    }

    pub fn get_tex_enviv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_tex_enviv_p)(target, pname, params)
    }

    pub fn get_tex_envxv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_tex_envxv_p)(target, pname, params)
    }

    pub fn get_tex_image(
        &self,
        target: c_uint,
        level: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *mut c_void,
    ) {
        (self.get_tex_image_p)(target, level, format, type_, pixels)
    }

    pub fn get_tex_level_parameterfv(
        &self,
        target: c_uint,
        level: c_int,
        pname: c_uint,
        params: *mut c_float,
    ) {
        (self.get_tex_level_parameterfv_p)(target, level, pname, params)
    }

    pub fn get_tex_level_parameteriv(
        &self,
        target: c_uint,
        level: c_int,
        pname: c_uint,
        params: *mut c_int,
    ) {
        (self.get_tex_level_parameteriv_p)(target, level, pname, params)
    }

    pub fn get_tex_parameter_iiv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_tex_parameter_iiv_p)(target, pname, params)
    }

    pub fn get_tex_parameter_iuiv(&self, target: c_uint, pname: c_uint, params: *mut c_uint) {
        (self.get_tex_parameter_iuiv_p)(target, pname, params)
    }

    pub fn get_tex_parameterfv(&self, target: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_tex_parameterfv_p)(target, pname, params)
    }

    pub fn get_tex_parameteriv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_tex_parameteriv_p)(target, pname, params)
    }

    pub fn get_tex_parameterxv(&self, target: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_tex_parameterxv_p)(target, pname, params)
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: c_uint,
        index: c_uint,
        buf_size: c_int,
        length: *mut c_int,
        size: *mut c_int,
        type_: *mut c_uint,
        name: *mut c_char,
    ) {
        (self.get_transform_feedback_varying_p)(program, index, buf_size, length, size, type_, name)
    }

    pub fn get_uniform_block_index(
        &self,
        program: c_uint,
        uniform_block_name: *const c_char,
    ) -> c_uint {
        (self.get_uniform_block_index_p)(program, uniform_block_name)
    }

    pub fn get_uniform_indices(
        &self,
        program: c_uint,
        uniform_count: c_int,
        uniform_names: *const *const c_char,
        uniform_indices: *mut c_uint,
    ) {
        (self.get_uniform_indices_p)(program, uniform_count, uniform_names, uniform_indices)
    }

    pub fn get_uniform_location(&self, program: c_uint, name: *const c_char) -> c_int {
        (self.get_uniform_location_p)(program, name)
    }

    pub fn get_uniformfv(&self, program: c_uint, location: c_int, params: *mut c_float) {
        (self.get_uniformfv_p)(program, location, params)
    }

    pub fn get_uniformiv(&self, program: c_uint, location: c_int, params: *mut c_int) {
        (self.get_uniformiv_p)(program, location, params)
    }

    pub fn get_uniformuiv(&self, program: c_uint, location: c_int, params: *mut c_uint) {
        (self.get_uniformuiv_p)(program, location, params)
    }

    pub fn get_vertex_attrib_iiv(&self, index: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_vertex_attrib_iiv_p)(index, pname, params)
    }

    pub fn get_vertex_attrib_iuiv(&self, index: c_uint, pname: c_uint, params: *mut c_uint) {
        (self.get_vertex_attrib_iuiv_p)(index, pname, params)
    }

    pub fn get_vertex_attrib_pointerv(
        &self,
        index: c_uint,
        pname: c_uint,
        pointer: *mut *mut c_void,
    ) {
        (self.get_vertex_attrib_pointerv_p)(index, pname, pointer)
    }

    pub fn get_vertex_attribdv(&self, index: c_uint, pname: c_uint, params: *mut c_double) {
        (self.get_vertex_attribdv_p)(index, pname, params)
    }

    pub fn get_vertex_attribfv(&self, index: c_uint, pname: c_uint, params: *mut c_float) {
        (self.get_vertex_attribfv_p)(index, pname, params)
    }

    pub fn get_vertex_attribiv(&self, index: c_uint, pname: c_uint, params: *mut c_int) {
        (self.get_vertex_attribiv_p)(index, pname, params)
    }

    pub fn getn_uniformfv(
        &self,
        program: c_uint,
        location: c_int,
        buf_size: c_int,
        params: *mut c_float,
    ) {
        (self.getn_uniformfv_p)(program, location, buf_size, params)
    }

    pub fn getn_uniformiv(
        &self,
        program: c_uint,
        location: c_int,
        buf_size: c_int,
        params: *mut c_int,
    ) {
        (self.getn_uniformiv_p)(program, location, buf_size, params)
    }

    pub fn getn_uniformuiv(
        &self,
        program: c_uint,
        location: c_int,
        buf_size: c_int,
        params: *mut c_uint,
    ) {
        (self.getn_uniformuiv_p)(program, location, buf_size, params)
    }

    pub fn hint(&self, target: c_uint, mode: c_uint) {
        (self.hint_p)(target, mode)
    }

    pub fn invalidate_framebuffer(
        &self,
        target: c_uint,
        num_attachments: c_int,
        attachments: *const c_uint,
    ) {
        (self.invalidate_framebuffer_p)(target, num_attachments, attachments)
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: c_uint,
        num_attachments: c_int,
        attachments: *const c_uint,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) {
        (self.invalidate_sub_framebuffer_p)(
            target,
            num_attachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    }

    pub fn is_buffer(&self, buffer: c_uint) -> c_uchar {
        (self.is_buffer_p)(buffer)
    }

    pub fn is_enabled(&self, cap: c_uint) -> c_uchar {
        (self.is_enabled_p)(cap)
    }

    pub fn is_enabledi(&self, target: c_uint, index: c_uint) -> c_uchar {
        (self.is_enabledi_p)(target, index)
    }

    pub fn is_framebuffer(&self, framebuffer: c_uint) -> c_uchar {
        (self.is_framebuffer_p)(framebuffer)
    }

    pub fn is_program(&self, program: c_uint) -> c_uchar {
        (self.is_program_p)(program)
    }

    pub fn is_program_pipeline(&self, pipeline: c_uint) -> c_uchar {
        (self.is_program_pipeline_p)(pipeline)
    }

    pub fn is_query(&self, id: c_uint) -> c_uchar {
        (self.is_query_p)(id)
    }

    pub fn is_renderbuffer(&self, renderbuffer: c_uint) -> c_uchar {
        (self.is_renderbuffer_p)(renderbuffer)
    }

    pub fn is_sampler(&self, sampler: c_uint) -> c_uchar {
        (self.is_sampler_p)(sampler)
    }

    pub fn is_shader(&self, shader: c_uint) -> c_uchar {
        (self.is_shader_p)(shader)
    }

    pub fn is_sync(&self, sync: *mut c_void) -> c_uchar {
        (self.is_sync_p)(sync)
    }

    pub fn is_texture(&self, texture: c_uint) -> c_uchar {
        (self.is_texture_p)(texture)
    }

    pub fn is_transform_feedback(&self, id: c_uint) -> c_uchar {
        (self.is_transform_feedback_p)(id)
    }

    pub fn is_vertex_array(&self, array: c_uint) -> c_uchar {
        (self.is_vertex_array_p)(array)
    }

    pub fn light_modelf(&self, pname: c_uint, param: c_float) {
        (self.light_modelf_p)(pname, param)
    }

    pub fn light_modelfv(&self, pname: c_uint, params: *const c_float) {
        (self.light_modelfv_p)(pname, params)
    }

    pub fn light_modelx(&self, pname: c_uint, param: c_int) {
        (self.light_modelx_p)(pname, param)
    }

    pub fn light_modelxv(&self, pname: c_uint, param: *const c_int) {
        (self.light_modelxv_p)(pname, param)
    }

    pub fn lightf(&self, light: c_uint, pname: c_uint, param: c_float) {
        (self.lightf_p)(light, pname, param)
    }

    pub fn lightfv(&self, light: c_uint, pname: c_uint, params: *const c_float) {
        (self.lightfv_p)(light, pname, params)
    }

    pub fn lightx(&self, light: c_uint, pname: c_uint, param: c_int) {
        (self.lightx_p)(light, pname, param)
    }

    pub fn lightxv(&self, light: c_uint, pname: c_uint, params: *const c_int) {
        (self.lightxv_p)(light, pname, params)
    }

    pub fn line_width(&self, width: c_float) {
        (self.line_width_p)(width)
    }

    pub fn line_widthx(&self, width: c_int) {
        (self.line_widthx_p)(width)
    }

    pub fn link_program(&self, program: c_uint) {
        (self.link_program_p)(program)
    }

    pub fn load_identity(&self) {
        (self.load_identity_p)()
    }

    pub fn load_matrixf(&self, m: *const c_float) {
        (self.load_matrixf_p)(m)
    }

    pub fn load_matrixx(&self, m: *const c_int) {
        (self.load_matrixx_p)(m)
    }

    pub fn logic_op(&self, opcode: c_uint) {
        (self.logic_op_p)(opcode)
    }

    pub fn map_buffer(&self, target: c_uint, access: c_uint) -> *mut c_void {
        (self.map_buffer_p)(target, access)
    }

    pub fn map_buffer_range(
        &self,
        target: c_uint,
        offset: isize,
        length: isize,
        access: c_uint,
    ) -> *mut c_void {
        (self.map_buffer_range_p)(target, offset, length, access)
    }

    pub fn materialf(&self, face: c_uint, pname: c_uint, param: c_float) {
        (self.materialf_p)(face, pname, param)
    }

    pub fn materialfv(&self, face: c_uint, pname: c_uint, params: *const c_float) {
        (self.materialfv_p)(face, pname, params)
    }

    pub fn materialx(&self, face: c_uint, pname: c_uint, param: c_int) {
        (self.materialx_p)(face, pname, param)
    }

    pub fn materialxv(&self, face: c_uint, pname: c_uint, param: *const c_int) {
        (self.materialxv_p)(face, pname, param)
    }

    pub fn matrix_mode(&self, mode: c_uint) {
        (self.matrix_mode_p)(mode)
    }

    pub fn memory_barrier(&self, barriers: c_uint) {
        (self.memory_barrier_p)(barriers)
    }

    pub fn memory_barrier_by_region(&self, barriers: c_uint) {
        (self.memory_barrier_by_region_p)(barriers)
    }

    pub fn min_sample_shading(&self, value: c_float) {
        (self.min_sample_shading_p)(value)
    }

    pub fn mult_matrixf(&self, m: *const c_float) {
        (self.mult_matrixf_p)(m)
    }

    pub fn mult_matrixx(&self, m: *const c_int) {
        (self.mult_matrixx_p)(m)
    }

    pub fn multi_draw_arrays(
        &self,
        mode: c_uint,
        first: *const c_int,
        count: *const c_int,
        drawcount: c_int,
    ) {
        (self.multi_draw_arrays_p)(mode, first, count, drawcount)
    }

    pub fn multi_draw_elements(
        &self,
        mode: c_uint,
        count: *const c_int,
        type_: c_uint,
        indices: *const *const c_void,
        drawcount: c_int,
    ) {
        (self.multi_draw_elements_p)(mode, count, type_, indices, drawcount)
    }

    pub fn multi_draw_elements_base_vertex(
        &self,
        mode: c_uint,
        count: *const c_int,
        type_: c_uint,
        indices: *const *const c_void,
        drawcount: c_int,
        basevertex: *const c_int,
    ) {
        (self.multi_draw_elements_base_vertex_p)(mode, count, type_, indices, drawcount, basevertex)
    }

    pub fn multi_tex_coord4f(
        &self,
        target: c_uint,
        s: c_float,
        t: c_float,
        r: c_float,
        q: c_float,
    ) {
        (self.multi_tex_coord4f_p)(target, s, t, r, q)
    }

    pub fn multi_tex_coord4x(&self, texture: c_uint, s: c_int, t: c_int, r: c_int, q: c_int) {
        (self.multi_tex_coord4x_p)(texture, s, t, r, q)
    }

    pub fn multi_tex_coord_p1ui(&self, texture: c_uint, type_: c_uint, coords: c_uint) {
        (self.multi_tex_coord_p1ui_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p1uiv(&self, texture: c_uint, type_: c_uint, coords: *const c_uint) {
        (self.multi_tex_coord_p1uiv_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p2ui(&self, texture: c_uint, type_: c_uint, coords: c_uint) {
        (self.multi_tex_coord_p2ui_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p2uiv(&self, texture: c_uint, type_: c_uint, coords: *const c_uint) {
        (self.multi_tex_coord_p2uiv_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p3ui(&self, texture: c_uint, type_: c_uint, coords: c_uint) {
        (self.multi_tex_coord_p3ui_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p3uiv(&self, texture: c_uint, type_: c_uint, coords: *const c_uint) {
        (self.multi_tex_coord_p3uiv_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p4ui(&self, texture: c_uint, type_: c_uint, coords: c_uint) {
        (self.multi_tex_coord_p4ui_p)(texture, type_, coords)
    }

    pub fn multi_tex_coord_p4uiv(&self, texture: c_uint, type_: c_uint, coords: *const c_uint) {
        (self.multi_tex_coord_p4uiv_p)(texture, type_, coords)
    }

    pub fn normal3f(&self, nx: c_float, ny: c_float, nz: c_float) {
        (self.normal3f_p)(nx, ny, nz)
    }

    pub fn normal3x(&self, nx: c_int, ny: c_int, nz: c_int) {
        (self.normal3x_p)(nx, ny, nz)
    }

    pub fn normal_p3ui(&self, type_: c_uint, coords: c_uint) {
        (self.normal_p3ui_p)(type_, coords)
    }

    pub fn normal_p3uiv(&self, type_: c_uint, coords: *const c_uint) {
        (self.normal_p3uiv_p)(type_, coords)
    }

    pub fn normal_pointer(&self, type_: c_uint, stride: c_int, pointer: *const c_void) {
        (self.normal_pointer_p)(type_, stride, pointer)
    }

    pub fn object_label(
        &self,
        identifier: c_uint,
        name: c_uint,
        length: c_int,
        label: *const c_char,
    ) {
        (self.object_label_p)(identifier, name, length, label)
    }

    pub fn object_ptr_label(&self, ptr: *const c_void, length: c_int, label: *const c_char) {
        (self.object_ptr_label_p)(ptr, length, label)
    }

    pub fn orthof(&self, l: c_float, r: c_float, b: c_float, t: c_float, n: c_float, f: c_float) {
        (self.orthof_p)(l, r, b, t, n, f)
    }

    pub fn orthox(&self, l: c_int, r: c_int, b: c_int, t: c_int, n: c_int, f: c_int) {
        (self.orthox_p)(l, r, b, t, n, f)
    }

    pub fn patch_parameteri(&self, pname: c_uint, value: c_int) {
        (self.patch_parameteri_p)(pname, value)
    }

    pub fn pause_transform_feedback(&self) {
        (self.pause_transform_feedback_p)()
    }

    pub fn pixel_storef(&self, pname: c_uint, param: c_float) {
        (self.pixel_storef_p)(pname, param)
    }

    pub fn pixel_storei(&self, pname: c_uint, param: c_int) {
        (self.pixel_storei_p)(pname, param)
    }

    pub fn point_parameterf(&self, pname: c_uint, param: c_float) {
        (self.point_parameterf_p)(pname, param)
    }

    pub fn point_parameterfv(&self, pname: c_uint, params: *const c_float) {
        (self.point_parameterfv_p)(pname, params)
    }

    pub fn point_parameteri(&self, pname: c_uint, param: c_int) {
        (self.point_parameteri_p)(pname, param)
    }

    pub fn point_parameteriv(&self, pname: c_uint, params: *const c_int) {
        (self.point_parameteriv_p)(pname, params)
    }

    pub fn point_parameterx(&self, pname: c_uint, param: c_int) {
        (self.point_parameterx_p)(pname, param)
    }

    pub fn point_parameterxv(&self, pname: c_uint, params: *const c_int) {
        (self.point_parameterxv_p)(pname, params)
    }

    pub fn point_size(&self, size: c_float) {
        (self.point_size_p)(size)
    }

    pub fn point_sizex(&self, size: c_int) {
        (self.point_sizex_p)(size)
    }

    pub fn polygon_mode(&self, face: c_uint, mode: c_uint) {
        (self.polygon_mode_p)(face, mode)
    }

    pub fn polygon_offset(&self, factor: c_float, units: c_float) {
        (self.polygon_offset_p)(factor, units)
    }

    pub fn polygon_offsetx(&self, factor: c_int, units: c_int) {
        (self.polygon_offsetx_p)(factor, units)
    }

    pub fn pop_debug_group(&self) {
        (self.pop_debug_group_p)()
    }

    pub fn pop_matrix(&self) {
        (self.pop_matrix_p)()
    }

    pub fn primitive_bounding_box(
        &self,
        min_x: c_float,
        min_y: c_float,
        min_z: c_float,
        min_w: c_float,
        max_x: c_float,
        max_y: c_float,
        max_z: c_float,
        max_w: c_float,
    ) {
        (self.primitive_bounding_box_p)(min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w)
    }

    pub fn primitive_restart_index(&self, index: c_uint) {
        (self.primitive_restart_index_p)(index)
    }

    pub fn program_binary(
        &self,
        program: c_uint,
        binary_format: c_uint,
        binary: *const c_void,
        length: c_int,
    ) {
        (self.program_binary_p)(program, binary_format, binary, length)
    }

    pub fn program_parameteri(&self, program: c_uint, pname: c_uint, value: c_int) {
        (self.program_parameteri_p)(program, pname, value)
    }

    pub fn program_uniform1f(&self, program: c_uint, location: c_int, v0: c_float) {
        (self.program_uniform1f_p)(program, location, v0)
    }

    pub fn program_uniform1fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_float,
    ) {
        (self.program_uniform1fv_p)(program, location, count, value)
    }

    pub fn program_uniform1i(&self, program: c_uint, location: c_int, v0: c_int) {
        (self.program_uniform1i_p)(program, location, v0)
    }

    pub fn program_uniform1iv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_int,
    ) {
        (self.program_uniform1iv_p)(program, location, count, value)
    }

    pub fn program_uniform1ui(&self, program: c_uint, location: c_int, v0: c_uint) {
        (self.program_uniform1ui_p)(program, location, v0)
    }

    pub fn program_uniform1uiv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_uint,
    ) {
        (self.program_uniform1uiv_p)(program, location, count, value)
    }

    pub fn program_uniform2f(&self, program: c_uint, location: c_int, v0: c_float, v1: c_float) {
        (self.program_uniform2f_p)(program, location, v0, v1)
    }

    pub fn program_uniform2fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_float,
    ) {
        (self.program_uniform2fv_p)(program, location, count, value)
    }

    pub fn program_uniform2i(&self, program: c_uint, location: c_int, v0: c_int, v1: c_int) {
        (self.program_uniform2i_p)(program, location, v0, v1)
    }

    pub fn program_uniform2iv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_int,
    ) {
        (self.program_uniform2iv_p)(program, location, count, value)
    }

    pub fn program_uniform2ui(&self, program: c_uint, location: c_int, v0: c_uint, v1: c_uint) {
        (self.program_uniform2ui_p)(program, location, v0, v1)
    }

    pub fn program_uniform2uiv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_uint,
    ) {
        (self.program_uniform2uiv_p)(program, location, count, value)
    }

    pub fn program_uniform3f(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_float,
        v1: c_float,
        v2: c_float,
    ) {
        (self.program_uniform3f_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_float,
    ) {
        (self.program_uniform3fv_p)(program, location, count, value)
    }

    pub fn program_uniform3i(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_int,
        v1: c_int,
        v2: c_int,
    ) {
        (self.program_uniform3i_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3iv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_int,
    ) {
        (self.program_uniform3iv_p)(program, location, count, value)
    }

    pub fn program_uniform3ui(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_uint,
        v1: c_uint,
        v2: c_uint,
    ) {
        (self.program_uniform3ui_p)(program, location, v0, v1, v2)
    }

    pub fn program_uniform3uiv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_uint,
    ) {
        (self.program_uniform3uiv_p)(program, location, count, value)
    }

    pub fn program_uniform4f(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_float,
        v1: c_float,
        v2: c_float,
        v3: c_float,
    ) {
        (self.program_uniform4f_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_float,
    ) {
        (self.program_uniform4fv_p)(program, location, count, value)
    }

    pub fn program_uniform4i(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_int,
        v1: c_int,
        v2: c_int,
        v3: c_int,
    ) {
        (self.program_uniform4i_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4iv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_int,
    ) {
        (self.program_uniform4iv_p)(program, location, count, value)
    }

    pub fn program_uniform4ui(
        &self,
        program: c_uint,
        location: c_int,
        v0: c_uint,
        v1: c_uint,
        v2: c_uint,
        v3: c_uint,
    ) {
        (self.program_uniform4ui_p)(program, location, v0, v1, v2, v3)
    }

    pub fn program_uniform4uiv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        value: *const c_uint,
    ) {
        (self.program_uniform4uiv_p)(program, location, count, value)
    }

    pub fn program_uniform_matrix2fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix2fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix2x3fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix2x3fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix2x4fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix2x4fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix3fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix3fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix3x2fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix3x2fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix3x4fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix3x4fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix4fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix4fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix4x2fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix4x2fv_p)(program, location, count, transpose, value)
    }

    pub fn program_uniform_matrix4x3fv(
        &self,
        program: c_uint,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.program_uniform_matrix4x3fv_p)(program, location, count, transpose, value)
    }

    pub fn provoking_vertex(&self, mode: c_uint) {
        (self.provoking_vertex_p)(mode)
    }

    pub fn push_debug_group(
        &self,
        source: c_uint,
        id: c_uint,
        length: c_int,
        message: *const c_char,
    ) {
        (self.push_debug_group_p)(source, id, length, message)
    }

    pub fn push_matrix(&self) {
        (self.push_matrix_p)()
    }

    pub fn query_counter(&self, id: c_uint, target: c_uint) {
        (self.query_counter_p)(id, target)
    }

    pub fn read_buffer(&self, src: c_uint) {
        (self.read_buffer_p)(src)
    }

    pub fn read_pixels(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *mut c_void,
    ) {
        (self.read_pixels_p)(x, y, width, height, format, type_, pixels)
    }

    pub fn readn_pixels(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        format: c_uint,
        type_: c_uint,
        buf_size: c_int,
        data: *mut c_void,
    ) {
        (self.readn_pixels_p)(x, y, width, height, format, type_, buf_size, data)
    }

    pub fn release_shader_compiler(&self) {
        (self.release_shader_compiler_p)()
    }

    pub fn renderbuffer_storage(
        &self,
        target: c_uint,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
    ) {
        (self.renderbuffer_storage_p)(target, internalformat, width, height)
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: c_uint,
        samples: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
    ) {
        (self.renderbuffer_storage_multisample_p)(target, samples, internalformat, width, height)
    }

    pub fn resume_transform_feedback(&self) {
        (self.resume_transform_feedback_p)()
    }

    pub fn rotatef(&self, angle: c_float, x: c_float, y: c_float, z: c_float) {
        (self.rotatef_p)(angle, x, y, z)
    }

    pub fn rotatex(&self, angle: c_int, x: c_int, y: c_int, z: c_int) {
        (self.rotatex_p)(angle, x, y, z)
    }

    pub fn sample_coverage(&self, value: c_float, invert: c_uchar) {
        (self.sample_coverage_p)(value, invert)
    }

    pub fn sample_coveragex(&self, value: c_int, invert: c_uchar) {
        (self.sample_coveragex_p)(value, invert)
    }

    pub fn sample_maski(&self, mask_number: c_uint, mask: c_uint) {
        (self.sample_maski_p)(mask_number, mask)
    }

    pub fn sampler_parameter_iiv(&self, sampler: c_uint, pname: c_uint, param: *const c_int) {
        (self.sampler_parameter_iiv_p)(sampler, pname, param)
    }

    pub fn sampler_parameter_iuiv(&self, sampler: c_uint, pname: c_uint, param: *const c_uint) {
        (self.sampler_parameter_iuiv_p)(sampler, pname, param)
    }

    pub fn sampler_parameterf(&self, sampler: c_uint, pname: c_uint, param: c_float) {
        (self.sampler_parameterf_p)(sampler, pname, param)
    }

    pub fn sampler_parameterfv(&self, sampler: c_uint, pname: c_uint, param: *const c_float) {
        (self.sampler_parameterfv_p)(sampler, pname, param)
    }

    pub fn sampler_parameteri(&self, sampler: c_uint, pname: c_uint, param: c_int) {
        (self.sampler_parameteri_p)(sampler, pname, param)
    }

    pub fn sampler_parameteriv(&self, sampler: c_uint, pname: c_uint, param: *const c_int) {
        (self.sampler_parameteriv_p)(sampler, pname, param)
    }

    pub fn scalef(&self, x: c_float, y: c_float, z: c_float) {
        (self.scalef_p)(x, y, z)
    }

    pub fn scalex(&self, x: c_int, y: c_int, z: c_int) {
        (self.scalex_p)(x, y, z)
    }

    pub fn scissor(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        (self.scissor_p)(x, y, width, height)
    }

    pub fn secondary_color_p3ui(&self, type_: c_uint, color: c_uint) {
        (self.secondary_color_p3ui_p)(type_, color)
    }

    pub fn secondary_color_p3uiv(&self, type_: c_uint, color: *const c_uint) {
        (self.secondary_color_p3uiv_p)(type_, color)
    }

    pub fn shade_model(&self, mode: c_uint) {
        (self.shade_model_p)(mode)
    }

    pub fn shader_binary(
        &self,
        count: c_int,
        shaders: *const c_uint,
        binary_format: c_uint,
        binary: *const c_void,
        length: c_int,
    ) {
        (self.shader_binary_p)(count, shaders, binary_format, binary, length)
    }

    pub fn shader_source(
        &self,
        shader: c_uint,
        count: c_int,
        string: *const *const c_char,
        length: *const c_int,
    ) {
        (self.shader_source_p)(shader, count, string, length)
    }

    pub fn stencil_func(&self, func: c_uint, ref_: c_int, mask: c_uint) {
        (self.stencil_func_p)(func, ref_, mask)
    }

    pub fn stencil_func_separate(&self, face: c_uint, func: c_uint, ref_: c_int, mask: c_uint) {
        (self.stencil_func_separate_p)(face, func, ref_, mask)
    }

    pub fn stencil_mask(&self, mask: c_uint) {
        (self.stencil_mask_p)(mask)
    }

    pub fn stencil_mask_separate(&self, face: c_uint, mask: c_uint) {
        (self.stencil_mask_separate_p)(face, mask)
    }

    pub fn stencil_op(&self, fail: c_uint, zfail: c_uint, zpass: c_uint) {
        (self.stencil_op_p)(fail, zfail, zpass)
    }

    pub fn stencil_op_separate(&self, face: c_uint, sfail: c_uint, dpfail: c_uint, dppass: c_uint) {
        (self.stencil_op_separate_p)(face, sfail, dpfail, dppass)
    }

    pub fn tex_buffer(&self, target: c_uint, internalformat: c_uint, buffer: c_uint) {
        (self.tex_buffer_p)(target, internalformat, buffer)
    }

    pub fn tex_buffer_range(
        &self,
        target: c_uint,
        internalformat: c_uint,
        buffer: c_uint,
        offset: isize,
        size: isize,
    ) {
        (self.tex_buffer_range_p)(target, internalformat, buffer, offset, size)
    }

    pub fn tex_coord_p1ui(&self, type_: c_uint, coords: c_uint) {
        (self.tex_coord_p1ui_p)(type_, coords)
    }

    pub fn tex_coord_p1uiv(&self, type_: c_uint, coords: *const c_uint) {
        (self.tex_coord_p1uiv_p)(type_, coords)
    }

    pub fn tex_coord_p2ui(&self, type_: c_uint, coords: c_uint) {
        (self.tex_coord_p2ui_p)(type_, coords)
    }

    pub fn tex_coord_p2uiv(&self, type_: c_uint, coords: *const c_uint) {
        (self.tex_coord_p2uiv_p)(type_, coords)
    }

    pub fn tex_coord_p3ui(&self, type_: c_uint, coords: c_uint) {
        (self.tex_coord_p3ui_p)(type_, coords)
    }

    pub fn tex_coord_p3uiv(&self, type_: c_uint, coords: *const c_uint) {
        (self.tex_coord_p3uiv_p)(type_, coords)
    }

    pub fn tex_coord_p4ui(&self, type_: c_uint, coords: c_uint) {
        (self.tex_coord_p4ui_p)(type_, coords)
    }

    pub fn tex_coord_p4uiv(&self, type_: c_uint, coords: *const c_uint) {
        (self.tex_coord_p4uiv_p)(type_, coords)
    }

    pub fn tex_coord_pointer(
        &self,
        size: c_int,
        type_: c_uint,
        stride: c_int,
        pointer: *const c_void,
    ) {
        (self.tex_coord_pointer_p)(size, type_, stride, pointer)
    }

    pub fn tex_envf(&self, target: c_uint, pname: c_uint, param: c_float) {
        (self.tex_envf_p)(target, pname, param)
    }

    pub fn tex_envfv(&self, target: c_uint, pname: c_uint, params: *const c_float) {
        (self.tex_envfv_p)(target, pname, params)
    }

    pub fn tex_envi(&self, target: c_uint, pname: c_uint, param: c_int) {
        (self.tex_envi_p)(target, pname, param)
    }

    pub fn tex_enviv(&self, target: c_uint, pname: c_uint, params: *const c_int) {
        (self.tex_enviv_p)(target, pname, params)
    }

    pub fn tex_envx(&self, target: c_uint, pname: c_uint, param: c_int) {
        (self.tex_envx_p)(target, pname, param)
    }

    pub fn tex_envxv(&self, target: c_uint, pname: c_uint, params: *const c_int) {
        (self.tex_envxv_p)(target, pname, params)
    }

    pub fn tex_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_int,
        width: c_int,
        border: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_image1_d_p)(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            type_,
            pixels,
        )
    }

    pub fn tex_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_int,
        width: c_int,
        height: c_int,
        border: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_image2_d_p)(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        )
    }

    pub fn tex_image2_d_multisample(
        &self,
        target: c_uint,
        samples: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        fixedsamplelocations: c_uchar,
    ) {
        (self.tex_image2_d_multisample_p)(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }

    pub fn tex_image3_d(
        &self,
        target: c_uint,
        level: c_int,
        internalformat: c_int,
        width: c_int,
        height: c_int,
        depth: c_int,
        border: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_image3_d_p)(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels,
        )
    }

    pub fn tex_image3_d_multisample(
        &self,
        target: c_uint,
        samples: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        depth: c_int,
        fixedsamplelocations: c_uchar,
    ) {
        (self.tex_image3_d_multisample_p)(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }

    pub fn tex_parameter_iiv(&self, target: c_uint, pname: c_uint, params: *const c_int) {
        (self.tex_parameter_iiv_p)(target, pname, params)
    }

    pub fn tex_parameter_iuiv(&self, target: c_uint, pname: c_uint, params: *const c_uint) {
        (self.tex_parameter_iuiv_p)(target, pname, params)
    }

    pub fn tex_parameterf(&self, target: c_uint, pname: c_uint, param: c_float) {
        (self.tex_parameterf_p)(target, pname, param)
    }

    pub fn tex_parameterfv(&self, target: c_uint, pname: c_uint, params: *const c_float) {
        (self.tex_parameterfv_p)(target, pname, params)
    }

    pub fn tex_parameteri(&self, target: c_uint, pname: c_uint, param: c_int) {
        (self.tex_parameteri_p)(target, pname, param)
    }

    pub fn tex_parameteriv(&self, target: c_uint, pname: c_uint, params: *const c_int) {
        (self.tex_parameteriv_p)(target, pname, params)
    }

    pub fn tex_parameterx(&self, target: c_uint, pname: c_uint, param: c_int) {
        (self.tex_parameterx_p)(target, pname, param)
    }

    pub fn tex_parameterxv(&self, target: c_uint, pname: c_uint, params: *const c_int) {
        (self.tex_parameterxv_p)(target, pname, params)
    }

    pub fn tex_storage2_d(
        &self,
        target: c_uint,
        levels: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
    ) {
        (self.tex_storage2_d_p)(target, levels, internalformat, width, height)
    }

    pub fn tex_storage2_d_multisample(
        &self,
        target: c_uint,
        samples: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        fixedsamplelocations: c_uchar,
    ) {
        (self.tex_storage2_d_multisample_p)(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }

    pub fn tex_storage3_d(
        &self,
        target: c_uint,
        levels: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        depth: c_int,
    ) {
        (self.tex_storage3_d_p)(target, levels, internalformat, width, height, depth)
    }

    pub fn tex_storage3_d_multisample(
        &self,
        target: c_uint,
        samples: c_int,
        internalformat: c_uint,
        width: c_int,
        height: c_int,
        depth: c_int,
        fixedsamplelocations: c_uchar,
    ) {
        (self.tex_storage3_d_multisample_p)(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }

    pub fn tex_sub_image1_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        width: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image1_d_p)(target, level, xoffset, width, format, type_, pixels)
    }

    pub fn tex_sub_image2_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        width: c_int,
        height: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image2_d_p)(
            target, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }

    pub fn tex_sub_image3_d(
        &self,
        target: c_uint,
        level: c_int,
        xoffset: c_int,
        yoffset: c_int,
        zoffset: c_int,
        width: c_int,
        height: c_int,
        depth: c_int,
        format: c_uint,
        type_: c_uint,
        pixels: *const c_void,
    ) {
        (self.tex_sub_image3_d_p)(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }

    pub fn transform_feedback_varyings(
        &self,
        program: c_uint,
        count: c_int,
        varyings: *const *const c_char,
        buffer_mode: c_uint,
    ) {
        (self.transform_feedback_varyings_p)(program, count, varyings, buffer_mode)
    }

    pub fn translatef(&self, x: c_float, y: c_float, z: c_float) {
        (self.translatef_p)(x, y, z)
    }

    pub fn translatex(&self, x: c_int, y: c_int, z: c_int) {
        (self.translatex_p)(x, y, z)
    }

    pub fn uniform1f(&self, location: c_int, v0: c_float) {
        (self.uniform1f_p)(location, v0)
    }

    pub fn uniform1fv(&self, location: c_int, count: c_int, value: *const c_float) {
        (self.uniform1fv_p)(location, count, value)
    }

    pub fn uniform1i(&self, location: c_int, v0: c_int) {
        (self.uniform1i_p)(location, v0)
    }

    pub fn uniform1iv(&self, location: c_int, count: c_int, value: *const c_int) {
        (self.uniform1iv_p)(location, count, value)
    }

    pub fn uniform1ui(&self, location: c_int, v0: c_uint) {
        (self.uniform1ui_p)(location, v0)
    }

    pub fn uniform1uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        (self.uniform1uiv_p)(location, count, value)
    }

    pub fn uniform2f(&self, location: c_int, v0: c_float, v1: c_float) {
        (self.uniform2f_p)(location, v0, v1)
    }

    pub fn uniform2fv(&self, location: c_int, count: c_int, value: *const c_float) {
        (self.uniform2fv_p)(location, count, value)
    }

    pub fn uniform2i(&self, location: c_int, v0: c_int, v1: c_int) {
        (self.uniform2i_p)(location, v0, v1)
    }

    pub fn uniform2iv(&self, location: c_int, count: c_int, value: *const c_int) {
        (self.uniform2iv_p)(location, count, value)
    }

    pub fn uniform2ui(&self, location: c_int, v0: c_uint, v1: c_uint) {
        (self.uniform2ui_p)(location, v0, v1)
    }

    pub fn uniform2uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        (self.uniform2uiv_p)(location, count, value)
    }

    pub fn uniform3f(&self, location: c_int, v0: c_float, v1: c_float, v2: c_float) {
        (self.uniform3f_p)(location, v0, v1, v2)
    }

    pub fn uniform3fv(&self, location: c_int, count: c_int, value: *const c_float) {
        (self.uniform3fv_p)(location, count, value)
    }

    pub fn uniform3i(&self, location: c_int, v0: c_int, v1: c_int, v2: c_int) {
        (self.uniform3i_p)(location, v0, v1, v2)
    }

    pub fn uniform3iv(&self, location: c_int, count: c_int, value: *const c_int) {
        (self.uniform3iv_p)(location, count, value)
    }

    pub fn uniform3ui(&self, location: c_int, v0: c_uint, v1: c_uint, v2: c_uint) {
        (self.uniform3ui_p)(location, v0, v1, v2)
    }

    pub fn uniform3uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        (self.uniform3uiv_p)(location, count, value)
    }

    pub fn uniform4f(&self, location: c_int, v0: c_float, v1: c_float, v2: c_float, v3: c_float) {
        (self.uniform4f_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4fv(&self, location: c_int, count: c_int, value: *const c_float) {
        (self.uniform4fv_p)(location, count, value)
    }

    pub fn uniform4i(&self, location: c_int, v0: c_int, v1: c_int, v2: c_int, v3: c_int) {
        (self.uniform4i_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4iv(&self, location: c_int, count: c_int, value: *const c_int) {
        (self.uniform4iv_p)(location, count, value)
    }

    pub fn uniform4ui(&self, location: c_int, v0: c_uint, v1: c_uint, v2: c_uint, v3: c_uint) {
        (self.uniform4ui_p)(location, v0, v1, v2, v3)
    }

    pub fn uniform4uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        (self.uniform4uiv_p)(location, count, value)
    }

    pub fn uniform_block_binding(
        &self,
        program: c_uint,
        uniform_block_index: c_uint,
        uniform_block_binding: c_uint,
    ) {
        (self.uniform_block_binding_p)(program, uniform_block_index, uniform_block_binding)
    }

    pub fn uniform_matrix2fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix2fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix2x3fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix2x3fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix2x4fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix2x4fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix3fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix3fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix3x2fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix3x2fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix3x4fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix3x4fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix4fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix4fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix4x2fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix4x2fv_p)(location, count, transpose, value)
    }

    pub fn uniform_matrix4x3fv(
        &self,
        location: c_int,
        count: c_int,
        transpose: c_uchar,
        value: *const c_float,
    ) {
        (self.uniform_matrix4x3fv_p)(location, count, transpose, value)
    }

    pub fn unmap_buffer(&self, target: c_uint) -> c_uchar {
        (self.unmap_buffer_p)(target)
    }

    pub fn use_program(&self, program: c_uint) {
        (self.use_program_p)(program)
    }

    pub fn use_program_stages(&self, pipeline: c_uint, stages: c_uint, program: c_uint) {
        (self.use_program_stages_p)(pipeline, stages, program)
    }

    pub fn validate_program(&self, program: c_uint) {
        (self.validate_program_p)(program)
    }

    pub fn validate_program_pipeline(&self, pipeline: c_uint) {
        (self.validate_program_pipeline_p)(pipeline)
    }

    pub fn vertex_attrib1d(&self, index: c_uint, x: c_double) {
        (self.vertex_attrib1d_p)(index, x)
    }

    pub fn vertex_attrib1dv(&self, index: c_uint, v: *const c_double) {
        (self.vertex_attrib1dv_p)(index, v)
    }

    pub fn vertex_attrib1f(&self, index: c_uint, x: c_float) {
        (self.vertex_attrib1f_p)(index, x)
    }

    pub fn vertex_attrib1fv(&self, index: c_uint, v: *const c_float) {
        (self.vertex_attrib1fv_p)(index, v)
    }

    pub fn vertex_attrib1s(&self, index: c_uint, x: c_short) {
        (self.vertex_attrib1s_p)(index, x)
    }

    pub fn vertex_attrib1sv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib1sv_p)(index, v)
    }

    pub fn vertex_attrib2d(&self, index: c_uint, x: c_double, y: c_double) {
        (self.vertex_attrib2d_p)(index, x, y)
    }

    pub fn vertex_attrib2dv(&self, index: c_uint, v: *const c_double) {
        (self.vertex_attrib2dv_p)(index, v)
    }

    pub fn vertex_attrib2f(&self, index: c_uint, x: c_float, y: c_float) {
        (self.vertex_attrib2f_p)(index, x, y)
    }

    pub fn vertex_attrib2fv(&self, index: c_uint, v: *const c_float) {
        (self.vertex_attrib2fv_p)(index, v)
    }

    pub fn vertex_attrib2s(&self, index: c_uint, x: c_short, y: c_short) {
        (self.vertex_attrib2s_p)(index, x, y)
    }

    pub fn vertex_attrib2sv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib2sv_p)(index, v)
    }

    pub fn vertex_attrib3d(&self, index: c_uint, x: c_double, y: c_double, z: c_double) {
        (self.vertex_attrib3d_p)(index, x, y, z)
    }

    pub fn vertex_attrib3dv(&self, index: c_uint, v: *const c_double) {
        (self.vertex_attrib3dv_p)(index, v)
    }

    pub fn vertex_attrib3f(&self, index: c_uint, x: c_float, y: c_float, z: c_float) {
        (self.vertex_attrib3f_p)(index, x, y, z)
    }

    pub fn vertex_attrib3fv(&self, index: c_uint, v: *const c_float) {
        (self.vertex_attrib3fv_p)(index, v)
    }

    pub fn vertex_attrib3s(&self, index: c_uint, x: c_short, y: c_short, z: c_short) {
        (self.vertex_attrib3s_p)(index, x, y, z)
    }

    pub fn vertex_attrib3sv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib3sv_p)(index, v)
    }

    pub fn vertex_attrib4_nbv(&self, index: c_uint, v: *const c_char) {
        (self.vertex_attrib4_nbv_p)(index, v)
    }

    pub fn vertex_attrib4_niv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib4_niv_p)(index, v)
    }

    pub fn vertex_attrib4_nsv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib4_nsv_p)(index, v)
    }

    pub fn vertex_attrib4_nub(
        &self,
        index: c_uint,
        x: c_uchar,
        y: c_uchar,
        z: c_uchar,
        w: c_uchar,
    ) {
        (self.vertex_attrib4_nub_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4_nubv(&self, index: c_uint, v: *const c_uchar) {
        (self.vertex_attrib4_nubv_p)(index, v)
    }

    pub fn vertex_attrib4_nuiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib4_nuiv_p)(index, v)
    }

    pub fn vertex_attrib4_nusv(&self, index: c_uint, v: *const c_ushort) {
        (self.vertex_attrib4_nusv_p)(index, v)
    }

    pub fn vertex_attrib4bv(&self, index: c_uint, v: *const c_char) {
        (self.vertex_attrib4bv_p)(index, v)
    }

    pub fn vertex_attrib4d(
        &self,
        index: c_uint,
        x: c_double,
        y: c_double,
        z: c_double,
        w: c_double,
    ) {
        (self.vertex_attrib4d_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4dv(&self, index: c_uint, v: *const c_double) {
        (self.vertex_attrib4dv_p)(index, v)
    }

    pub fn vertex_attrib4f(&self, index: c_uint, x: c_float, y: c_float, z: c_float, w: c_float) {
        (self.vertex_attrib4f_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4fv(&self, index: c_uint, v: *const c_float) {
        (self.vertex_attrib4fv_p)(index, v)
    }

    pub fn vertex_attrib4iv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib4iv_p)(index, v)
    }

    pub fn vertex_attrib4s(&self, index: c_uint, x: c_short, y: c_short, z: c_short, w: c_short) {
        (self.vertex_attrib4s_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib4sv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib4sv_p)(index, v)
    }

    pub fn vertex_attrib4ubv(&self, index: c_uint, v: *const c_uchar) {
        (self.vertex_attrib4ubv_p)(index, v)
    }

    pub fn vertex_attrib4uiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib4uiv_p)(index, v)
    }

    pub fn vertex_attrib4usv(&self, index: c_uint, v: *const c_ushort) {
        (self.vertex_attrib4usv_p)(index, v)
    }

    pub fn vertex_attrib_binding(&self, attribindex: c_uint, bindingindex: c_uint) {
        (self.vertex_attrib_binding_p)(attribindex, bindingindex)
    }

    pub fn vertex_attrib_divisor(&self, index: c_uint, divisor: c_uint) {
        (self.vertex_attrib_divisor_p)(index, divisor)
    }

    pub fn vertex_attrib_format(
        &self,
        attribindex: c_uint,
        size: c_int,
        type_: c_uint,
        normalized: c_uchar,
        relativeoffset: c_uint,
    ) {
        (self.vertex_attrib_format_p)(attribindex, size, type_, normalized, relativeoffset)
    }

    pub fn vertex_attrib_i1i(&self, index: c_uint, x: c_int) {
        (self.vertex_attrib_i1i_p)(index, x)
    }

    pub fn vertex_attrib_i1iv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib_i1iv_p)(index, v)
    }

    pub fn vertex_attrib_i1ui(&self, index: c_uint, x: c_uint) {
        (self.vertex_attrib_i1ui_p)(index, x)
    }

    pub fn vertex_attrib_i1uiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib_i1uiv_p)(index, v)
    }

    pub fn vertex_attrib_i2i(&self, index: c_uint, x: c_int, y: c_int) {
        (self.vertex_attrib_i2i_p)(index, x, y)
    }

    pub fn vertex_attrib_i2iv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib_i2iv_p)(index, v)
    }

    pub fn vertex_attrib_i2ui(&self, index: c_uint, x: c_uint, y: c_uint) {
        (self.vertex_attrib_i2ui_p)(index, x, y)
    }

    pub fn vertex_attrib_i2uiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib_i2uiv_p)(index, v)
    }

    pub fn vertex_attrib_i3i(&self, index: c_uint, x: c_int, y: c_int, z: c_int) {
        (self.vertex_attrib_i3i_p)(index, x, y, z)
    }

    pub fn vertex_attrib_i3iv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib_i3iv_p)(index, v)
    }

    pub fn vertex_attrib_i3ui(&self, index: c_uint, x: c_uint, y: c_uint, z: c_uint) {
        (self.vertex_attrib_i3ui_p)(index, x, y, z)
    }

    pub fn vertex_attrib_i3uiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib_i3uiv_p)(index, v)
    }

    pub fn vertex_attrib_i4bv(&self, index: c_uint, v: *const c_char) {
        (self.vertex_attrib_i4bv_p)(index, v)
    }

    pub fn vertex_attrib_i4i(&self, index: c_uint, x: c_int, y: c_int, z: c_int, w: c_int) {
        (self.vertex_attrib_i4i_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib_i4iv(&self, index: c_uint, v: *const c_int) {
        (self.vertex_attrib_i4iv_p)(index, v)
    }

    pub fn vertex_attrib_i4sv(&self, index: c_uint, v: *const c_short) {
        (self.vertex_attrib_i4sv_p)(index, v)
    }

    pub fn vertex_attrib_i4ubv(&self, index: c_uint, v: *const c_uchar) {
        (self.vertex_attrib_i4ubv_p)(index, v)
    }

    pub fn vertex_attrib_i4ui(&self, index: c_uint, x: c_uint, y: c_uint, z: c_uint, w: c_uint) {
        (self.vertex_attrib_i4ui_p)(index, x, y, z, w)
    }

    pub fn vertex_attrib_i4uiv(&self, index: c_uint, v: *const c_uint) {
        (self.vertex_attrib_i4uiv_p)(index, v)
    }

    pub fn vertex_attrib_i4usv(&self, index: c_uint, v: *const c_ushort) {
        (self.vertex_attrib_i4usv_p)(index, v)
    }

    pub fn vertex_attrib_i_format(
        &self,
        attribindex: c_uint,
        size: c_int,
        type_: c_uint,
        relativeoffset: c_uint,
    ) {
        (self.vertex_attrib_i_format_p)(attribindex, size, type_, relativeoffset)
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: c_uint,
        size: c_int,
        type_: c_uint,
        stride: c_int,
        pointer: *const c_void,
    ) {
        (self.vertex_attrib_i_pointer_p)(index, size, type_, stride, pointer)
    }

    pub fn vertex_attrib_p1ui(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: c_uint,
    ) {
        (self.vertex_attrib_p1ui_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p1uiv(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: *const c_uint,
    ) {
        (self.vertex_attrib_p1uiv_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p2ui(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: c_uint,
    ) {
        (self.vertex_attrib_p2ui_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p2uiv(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: *const c_uint,
    ) {
        (self.vertex_attrib_p2uiv_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p3ui(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: c_uint,
    ) {
        (self.vertex_attrib_p3ui_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p3uiv(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: *const c_uint,
    ) {
        (self.vertex_attrib_p3uiv_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p4ui(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: c_uint,
    ) {
        (self.vertex_attrib_p4ui_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_p4uiv(
        &self,
        index: c_uint,
        type_: c_uint,
        normalized: c_uchar,
        value: *const c_uint,
    ) {
        (self.vertex_attrib_p4uiv_p)(index, type_, normalized, value)
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: c_uint,
        size: c_int,
        type_: c_uint,
        normalized: c_uchar,
        stride: c_int,
        pointer: *const c_void,
    ) {
        (self.vertex_attrib_pointer_p)(index, size, type_, normalized, stride, pointer)
    }

    pub fn vertex_binding_divisor(&self, bindingindex: c_uint, divisor: c_uint) {
        (self.vertex_binding_divisor_p)(bindingindex, divisor)
    }

    pub fn vertex_p2ui(&self, type_: c_uint, value: c_uint) {
        (self.vertex_p2ui_p)(type_, value)
    }

    pub fn vertex_p2uiv(&self, type_: c_uint, value: *const c_uint) {
        (self.vertex_p2uiv_p)(type_, value)
    }

    pub fn vertex_p3ui(&self, type_: c_uint, value: c_uint) {
        (self.vertex_p3ui_p)(type_, value)
    }

    pub fn vertex_p3uiv(&self, type_: c_uint, value: *const c_uint) {
        (self.vertex_p3uiv_p)(type_, value)
    }

    pub fn vertex_p4ui(&self, type_: c_uint, value: c_uint) {
        (self.vertex_p4ui_p)(type_, value)
    }

    pub fn vertex_p4uiv(&self, type_: c_uint, value: *const c_uint) {
        (self.vertex_p4uiv_p)(type_, value)
    }

    pub fn vertex_pointer(
        &self,
        size: c_int,
        type_: c_uint,
        stride: c_int,
        pointer: *const c_void,
    ) {
        (self.vertex_pointer_p)(size, type_, stride, pointer)
    }

    pub fn viewport(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        (self.viewport_p)(x, y, width, height)
    }

    pub fn wait_sync(&self, sync: *mut c_void, flags: c_uint, timeout: u64) {
        (self.wait_sync_p)(sync, flags, timeout)
    }
}