type _hipblasLtGroupedGemmOpaque_t = ();

/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
pub type hipStream_t = *mut ihipStream_t;
impl hipblasStatus_t {
    #[doc = "< Function succeeds"]
    pub const HIPBLAS_STATUS_SUCCESS: hipblasStatus_t = hipblasStatus_t(0);
}
impl hipblasStatus_t {
    #[doc = "< HIPBLAS library not initialized"]
    pub const HIPBLAS_STATUS_NOT_INITIALIZED: hipblasStatus_t = hipblasStatus_t(1);
}
impl hipblasStatus_t {
    #[doc = "< resource allocation failed"]
    pub const HIPBLAS_STATUS_ALLOC_FAILED: hipblasStatus_t = hipblasStatus_t(2);
}
impl hipblasStatus_t {
    #[doc = "< unsupported numerical value was passed to function"]
    pub const HIPBLAS_STATUS_INVALID_VALUE: hipblasStatus_t = hipblasStatus_t(3);
}
impl hipblasStatus_t {
    #[doc = "< access to GPU memory space failed"]
    pub const HIPBLAS_STATUS_MAPPING_ERROR: hipblasStatus_t = hipblasStatus_t(4);
}
impl hipblasStatus_t {
    #[doc = "< GPU program failed to execute"]
    pub const HIPBLAS_STATUS_EXECUTION_FAILED: hipblasStatus_t = hipblasStatus_t(5);
}
impl hipblasStatus_t {
    #[doc = "< an internal HIPBLAS operation failed"]
    pub const HIPBLAS_STATUS_INTERNAL_ERROR: hipblasStatus_t = hipblasStatus_t(6);
}
impl hipblasStatus_t {
    #[doc = "< function not implemented"]
    pub const HIPBLAS_STATUS_NOT_SUPPORTED: hipblasStatus_t = hipblasStatus_t(7);
}
impl hipblasStatus_t {
    #[doc = "< architecture mismatch"]
    pub const HIPBLAS_STATUS_ARCH_MISMATCH: hipblasStatus_t = hipblasStatus_t(8);
}
impl hipblasStatus_t {
    #[doc = "< hipBLAS handle is null pointer"]
    pub const HIPBLAS_STATUS_HANDLE_IS_NULLPTR: hipblasStatus_t = hipblasStatus_t(9);
}
impl hipblasStatus_t {
    #[doc = "<  unsupported enum value was passed to function"]
    pub const HIPBLAS_STATUS_INVALID_ENUM: hipblasStatus_t = hipblasStatus_t(10);
}
impl hipblasStatus_t {
    #[doc = "<  back-end returned an unsupported status code"]
    pub const HIPBLAS_STATUS_UNKNOWN: hipblasStatus_t = hipblasStatus_t(11);
}
#[repr(transparent)]
#[doc = " \\brief hipblas status codes definition"]
#[must_use]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasStatus_t(pub ::std::os::raw::c_uint);
impl hipblasDatatype_t {
    #[doc = "< 16 bit floating point, real"]
    pub const HIPBLAS_R_16F: hipblasDatatype_t = hipblasDatatype_t(150);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit floating point, real"]
    pub const HIPBLAS_R_32F: hipblasDatatype_t = hipblasDatatype_t(151);
}
impl hipblasDatatype_t {
    #[doc = "< 64 bit floating point, real"]
    pub const HIPBLAS_R_64F: hipblasDatatype_t = hipblasDatatype_t(152);
}
impl hipblasDatatype_t {
    #[doc = "< 16 bit floating point, complex"]
    pub const HIPBLAS_C_16F: hipblasDatatype_t = hipblasDatatype_t(153);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit floating point, complex"]
    pub const HIPBLAS_C_32F: hipblasDatatype_t = hipblasDatatype_t(154);
}
impl hipblasDatatype_t {
    #[doc = "< 64 bit floating point, complex"]
    pub const HIPBLAS_C_64F: hipblasDatatype_t = hipblasDatatype_t(155);
}
impl hipblasDatatype_t {
    #[doc = "<  8 bit signed integer, real"]
    pub const HIPBLAS_R_8I: hipblasDatatype_t = hipblasDatatype_t(160);
}
impl hipblasDatatype_t {
    #[doc = "<  8 bit unsigned integer, real"]
    pub const HIPBLAS_R_8U: hipblasDatatype_t = hipblasDatatype_t(161);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit signed integer, real"]
    pub const HIPBLAS_R_32I: hipblasDatatype_t = hipblasDatatype_t(162);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit unsigned integer, real"]
    pub const HIPBLAS_R_32U: hipblasDatatype_t = hipblasDatatype_t(163);
}
impl hipblasDatatype_t {
    #[doc = "<  8 bit signed integer, complex"]
    pub const HIPBLAS_C_8I: hipblasDatatype_t = hipblasDatatype_t(164);
}
impl hipblasDatatype_t {
    #[doc = "<  8 bit unsigned integer, complex"]
    pub const HIPBLAS_C_8U: hipblasDatatype_t = hipblasDatatype_t(165);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit signed integer, complex"]
    pub const HIPBLAS_C_32I: hipblasDatatype_t = hipblasDatatype_t(166);
}
impl hipblasDatatype_t {
    #[doc = "< 32 bit unsigned integer, complex"]
    pub const HIPBLAS_C_32U: hipblasDatatype_t = hipblasDatatype_t(167);
}
impl hipblasDatatype_t {
    #[doc = "< 16 bit bfloat, real"]
    pub const HIPBLAS_R_16B: hipblasDatatype_t = hipblasDatatype_t(168);
}
impl hipblasDatatype_t {
    #[doc = "< 16 bit bfloat, complex"]
    pub const HIPBLAS_C_16B: hipblasDatatype_t = hipblasDatatype_t(169);
}
impl hipblasDatatype_t {
    #[doc = "< Invalid datatype value, do not use"]
    pub const HIPBLAS_DATATYPE_INVALID: hipblasDatatype_t = hipblasDatatype_t(255);
}
#[repr(transparent)]
#[doc = " \\brief Indicates the precision width of data stored in a blas type."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasDatatype_t(pub ::std::os::raw::c_uint);
#[doc = " \\brief Struct to represent a 16 bit brain floating point number."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hip_bfloat16 {
    pub data: u16,
}
#[doc = " \\brief Single precision floating point type"]
pub type hipblasLtFloat = f32;
#[doc = " \\brief Structure definition for hipblasLtHalf"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _hipblasLtHalf {
    pub data: u16,
}
#[doc = " \\brief Structure definition for hipblasLtHalf"]
pub type hipblasLtHalf = _hipblasLtHalf;
#[doc = " \\brief Struct to represent a 16 bit brain floating point number."]
pub type hipblasLtBfloat16 = hip_bfloat16;
impl hipblasLtEpilogue_t {
    #[doc = "<No special postprocessing, just scale and quantize the results if necessary."]
    pub const HIPBLASLT_EPILOGUE_DEFAULT: hipblasLtEpilogue_t = hipblasLtEpilogue_t(1);
}
impl hipblasLtEpilogue_t {
    #[doc = "<Apply ReLU point-wise transform to the results:(x:=max(x, 0))"]
    pub const HIPBLASLT_EPILOGUE_RELU: hipblasLtEpilogue_t = hipblasLtEpilogue_t(2);
}
impl hipblasLtEpilogue_t {
    #[doc = "<Apply (broadcast) bias from the bias vector. Bias vector length must match matrix D rows, and it must be packed (such as stride between vector elements is 1). Bias vector is broadcast to all columns and added before applying the final postprocessing."]
    pub const HIPBLASLT_EPILOGUE_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(4);
}
impl hipblasLtEpilogue_t {
    #[doc = "<Apply bias and then ReLU transform."]
    pub const HIPBLASLT_EPILOGUE_RELU_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(6);
}
impl hipblasLtEpilogue_t {
    #[doc = "<Apply GELU point-wise transform to the results (x:=GELU(x))."]
    pub const HIPBLASLT_EPILOGUE_GELU: hipblasLtEpilogue_t = hipblasLtEpilogue_t(32);
}
impl hipblasLtEpilogue_t {
    #[doc = "<Apply Bias and then GELU transform."]
    pub const HIPBLASLT_EPILOGUE_GELU_BIAS: hipblasLtEpilogue_t = hipblasLtEpilogue_t(36);
}
#[repr(transparent)]
#[doc = " \\ingroup types_module\n  \\brief Specify the enum type to set the postprocessing options for the epilogue."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtEpilogue_t(pub ::std::os::raw::c_uint);
impl hipblasLtComputeType_t {
    #[doc = "<32-bit floating-point precision."]
    pub const HIPBLASLT_COMPUTE_F32: hipblasLtComputeType_t = hipblasLtComputeType_t(300);
}
#[repr(transparent)]
#[doc = " \\ingroup types_module\n  \\brief Specify the compute precision modes of the matrix"]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtComputeType_t(pub ::std::os::raw::c_uint);
impl hipblasLtMatrixLayoutAttribute_t {
    #[doc = "<Number of batch of this matrix. Default value is 1. Data Type: int32_t"]
    pub const HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT: hipblasLtMatrixLayoutAttribute_t =
        hipblasLtMatrixLayoutAttribute_t(0);
}
impl hipblasLtMatrixLayoutAttribute_t {
    #[doc = "<Stride (in elements) to the next matrix for the strided batch operation. Default value is 0. Data Type: int64_t"]
    pub const HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET: hipblasLtMatrixLayoutAttribute_t =
        hipblasLtMatrixLayoutAttribute_t(1);
}
#[repr(transparent)]
#[doc = " \\ingroup types_module\n  \\brief Specify the attributes that define the details of the matrix."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatrixLayoutAttribute_t(pub ::std::os::raw::c_uint);
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<Specifies the type of transformation operation that should be performed on matrix A. Default value is HIPBLAS_OP_N (for example, non-transpose operation). See hipblasOperation_t. Data Type:int32_t"]
    pub const HIPBLASLT_MATMUL_DESC_TRANSA: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(0);
}
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<Specifies the type of transformation operation that should be performed on matrix B. Default value is HIPBLAS_OP_N (for example, non-transpose operation). See hipblasOperation_t. Data Type:int32_t"]
    pub const HIPBLASLT_MATMUL_DESC_TRANSB: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(1);
}
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<Epilogue function. See hipblasLtEpilogue_t. Default value is: HIPBLASLT_EPILOGUE_DEFAULT. Data Type: uint32_t"]
    pub const HIPBLASLT_MATMUL_DESC_EPILOGUE: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(2);
}
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<Bias or Bias gradient vector pointer in the device memory. Data Type:void* /const void*"]
    pub const HIPBLASLT_MATMUL_DESC_BIAS_POINTER: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(3);
}
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<Type of the bias vector in the device memory. Can be set same as D matrix type or Scale type. Bias case: see HIPBLASLT_EPILOGUE_BIAS. Data Type:int32_t based on hipblasDatatype_t"]
    pub const HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(4);
}
impl hipblasLtMatmulDescAttributes_t {
    #[doc = "<D scale vector length must match matrix D rows. It must match Scale data type. D scale vector is broadcast to all columns and multipied after final postprocssion. Data Type: void* /const void*"]
    pub const HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(5);
}
impl hipblasLtMatmulDescAttributes_t {
    pub const HIPBLASLT_MATMUL_DESC_MAX: hipblasLtMatmulDescAttributes_t =
        hipblasLtMatmulDescAttributes_t(6);
}
#[repr(transparent)]
#[doc = " \\ingroup types_module\n  \\brief Specify the attributes that define the specifics of the matrix multiply operation."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulDescAttributes_t(pub ::std::os::raw::c_uint);
impl hipblasLtMatmulPreferenceAttributes_t {
    #[doc = "<Search mode. Data Type: uint32_t"]
    pub const HIPBLASLT_MATMUL_PREF_SEARCH_MODE: hipblasLtMatmulPreferenceAttributes_t =
        hipblasLtMatmulPreferenceAttributes_t(0);
}
impl hipblasLtMatmulPreferenceAttributes_t {
    #[doc = "<Maximum allowed workspace memory. Default is 0 (no workspace memory allowed). Data Type: uint64_t"]
    pub const HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES: hipblasLtMatmulPreferenceAttributes_t =
        hipblasLtMatmulPreferenceAttributes_t(1);
}
impl hipblasLtMatmulPreferenceAttributes_t {
    pub const HIPBLASLT_MATMUL_PREF_MAX: hipblasLtMatmulPreferenceAttributes_t =
        hipblasLtMatmulPreferenceAttributes_t(2);
}
#[repr(transparent)]
#[doc = " \\ingroup types_module\n  \\brief It is an enumerated type used to apply algorithm search preferences while fine-tuning the heuristic function."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipblasLtMatmulPreferenceAttributes_t(pub ::std::os::raw::c_uint);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipblasLtMatmulDescOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipblasLtMatrixLayoutOpaque_t {
    pub data: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipblasLtMatmulPreferenceOpaque_t {
    pub data: [u64; 5usize],
}
#[doc = " \\ingroup types_module\n  \\brief Handle to the hipBLASLt library context queue\n\n  \\details\n  The hipblasLtHandle_t type is a pointer type to an opaque structure holding the hipBLASLt library context. Use the following functions to manipulate this library context:\n\n  \\ref hipblasLtCreate():\n  To initialize the hipBLASLt library context and return a handle to an opaque structure holding the hipBLASLt library context.\n  \\ref hipblasLtDestroy():\n  To destroy a previously created hipBLASLt library context descriptor and release the resources."]
pub type hipblasLtHandle_t = *mut ::std::os::raw::c_void;
#[doc = " \\ingroup types_module\n  \\brief Descriptor of the matrix multiplication operation\n\n  \\details\n  This is a pointer to an opaque structure holding the description of the matrix multiplication operation \\ref hipblasLtMatmul().\n  Use the following functions to manipulate this descriptor:\n  \\ref hipblasLtMatmulDescCreate(): To create one instance of the descriptor.\n  \\ref hipblasLtMatmulDescDestroy(): To destroy a previously created descriptor and release the resources."]
pub type hipblasLtMatmulDesc_t = *mut hipblasLtMatmulDescOpaque_t;
#[doc = " \\ingroup types_module\n  \\brief Descriptor of the matrix layout\n\n  \\details\n  This is a pointer to an opaque structure holding the description of a matrix layout.\n  Use the following functions to manipulate this descriptor:\n  \\ref hipblasLtMatrixLayoutCreate(): To create one instance of the descriptor.\n  \\ref hipblasLtMatrixLayoutDestroy(): To destroy a previously created descriptor and release the resources."]
pub type hipblasLtMatrixLayout_t = *mut hipblasLtMatrixLayoutOpaque_t;
#[doc = " \\ingroup types_module\n  \\brief Descriptor of the matrix multiplication preference\n\n  \\details\n  This is a pointer to an opaque structure holding the description of the preferences for \\ref hipblasLtMatmulAlgoGetHeuristic() configuration.\n  Use the following functions to manipulate this descriptor:\n  \\ref hipblasLtMatmulPreferenceCreate(): To create one instance of the descriptor.\n  \\ref hipblasLtMatmulPreferenceDestroy(): To destroy a previously created descriptor and release the resources."]
pub type hipblasLtMatmulPreference_t = *mut hipblasLtMatmulPreferenceOpaque_t;
pub type hipblasLtGroupedGemmOpaque_t = _hipblasLtGroupedGemmOpaque_t;
pub type hipblasLtExtGroupedGemm_t = *mut hipblasLtGroupedGemmOpaque_t;
#[doc = " \\ingroup types_module\n  \\brief Description of the matrix multiplication algorithm\n\n  \\details\n  This is an opaque structure holding the description of the matrix multiplication algorithm.\n  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _hipblasLtMatmulAlgo_t {
    pub data: [u8; 104usize],
    pub max_workspace_bytes: usize,
}
#[doc = " \\ingroup types_module\n  \\brief Description of the matrix multiplication algorithm\n\n  \\details\n  This is an opaque structure holding the description of the matrix multiplication algorithm.\n  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again."]
pub type hipblasLtMatmulAlgo_t = _hipblasLtMatmulAlgo_t;
#[doc = " \\ingroup types_module\n  \\brief Description of the matrix multiplication algorithm\n\n  \\details\n  This is a descriptor that holds the configured matrix multiplication algorithm descriptor and its runtime properties.\n  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _hipblasLtMatmulHeuristicResult_t {
    #[doc = "<Algo struct"]
    pub algo: hipblasLtMatmulAlgo_t,
    #[doc = "<Actual size of workspace memory required."]
    pub workspaceSize: usize,
    #[doc = "<Result status. Other fields are valid only if, after call to hipblasLtMatmulAlgoGetHeuristic(), this member is set to HIPBLAS_STATUS_SUCCESS.."]
    pub state: hipblasStatus_t,
    #[doc = "<Waves count is a device utilization metric. A wavesCount value of 1.0f suggests that when the kernel is launched it will fully occupy the GPU."]
    pub wavesCount: f32,
    #[doc = "<Reserved."]
    pub reserved: [::std::os::raw::c_int; 4usize],
}
#[doc = " \\ingroup types_module\n  \\brief Description of the matrix multiplication algorithm\n\n  \\details\n  This is a descriptor that holds the configured matrix multiplication algorithm descriptor and its runtime properties.\n  This structure can be trivially serialized and later restored for use with the same version of hipBLASLt library to save on selecting the right configuration again."]
pub type hipblasLtMatmulHeuristicResult_t = _hipblasLtMatmulHeuristicResult_t;
extern "C" {
    #[must_use]
    pub fn hipblasLtGetVersion(
        handle: hipblasLtHandle_t,
        version: *mut ::std::os::raw::c_int,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtGetGitRevision(
        handle: hipblasLtHandle_t,
        rev: *mut ::std::os::raw::c_char,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtGetArchName(archName: *mut *mut ::std::os::raw::c_char) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Create a hipblaslt handle\n\n  \\details\n  This function initializes the hipBLASLt library and creates a handle to an\n opaque structure holding the hipBLASLt library context. It allocates light\n hardware resources on the host and device, and must be called prior to making\n any other hipBLASLt library calls. The hipBLASLt library context is tied to\n the current CUDA device. To use the library on multiple devices, one\n hipBLASLt handle should be created for each device.\n\n  @param[out]\n  handle  Pointer to the allocated hipBLASLt handle for the created hipBLASLt\n context.\n\n  \\retval HIPBLAS_STATUS_SUCCESS The allocation completed successfully.\n  \\retval HIPBLAS_STATUS_INVALID_VALUE \\p handle == NULL."]
    pub fn hipblasLtCreate(handle: *mut hipblasLtHandle_t) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Destory a hipblaslt handle\n\n  \\details\n  This function releases hardware resources used by the hipBLASLt library.\n  This function is usually the last call with a particular handle to the\n hipBLASLt library. Because hipblasLtCreate() allocates some internal\n resources and the release of those resources by calling hipblasLtDestroy()\n will implicitly call hipDeviceSynchronize(), it is recommended to minimize\n the number of hipblasLtCreate()/hipblasLtDestroy() occurrences.\n\n  @param[in]\n  handle  Pointer to the hipBLASLt handle to be destroyed.\n\n  \\retval HIPBLAS_STATUS_SUCCESS The hipBLASLt context was successfully\n destroyed. \\retval HIPBLAS_STATUS_NOT_INITIALIZED The hipBLASLt library was\n not initialized. \\retval HIPBLAS_STATUS_INVALID_VALUE \\p handle == NULL."]
    pub fn hipblasLtDestroy(handle: hipblasLtHandle_t) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Create a matrix layout descriptor\n\n  \\details\n  This function creates a matrix layout descriptor by allocating the memory\n needed to hold its opaque structure.\n\n  @param[out]\n  matLayout Pointer to the structure holding the matrix layout descriptor\n created by this function. see \\ref hipblasLtMatrixLayout_t .\n  @param[in]\n  type Enumerant that specifies the data precision for the matrix layout\n descriptor this function creates. See hipblasDataType_t.\n  @param[in]\n  rows Number of rows of the matrix.\n  @param[in]\n  cols Number of columns of the matrix.\n  @param[in]\n  ld The leading dimension of the matrix. In column major layout, this is the\n number of elements to jump to reach the next column. Thus ld >= m (number of\n rows).\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the descriptor was created successfully.\n  \\retval HIPBLAS_STATUS_ALLOC_FAILED If the memory could not be allocated."]
    pub fn hipblasLtMatrixLayoutCreate(
        matLayout: *mut hipblasLtMatrixLayout_t,
        type_: hipblasDatatype_t,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Destory a matrix layout descriptor\n\n  \\details\n  This function destroys a previously created matrix layout descriptor object.\n\n  @param[in]\n  matLayout Pointer to the structure holding the matrix layout descriptor that\n should be destroyed by this function. see \\ref hipblasLtMatrixLayout_t .\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the operation was successful."]
    pub fn hipblasLtMatrixLayoutDestroy(matLayout: hipblasLtMatrixLayout_t) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief  Set attribute to a matrix descriptor\n\n  \\details\n  This function sets the value of the specified attribute belonging to a\n previously created matrix descriptor.\n\n  @param[in]\n  matLayout  Pointer to the previously created structure holding the matrix\n mdescriptor queried by this function. See \\ref hipblasLtMatrixLayout_t\n .\n  @param[in]\n  attr  \tThe attribute that will be set by this function. See \\ref\n hipblasLtMatrixLayoutAttribute_t.\n  @param[in]\n  buf  The value to which the specified attribute should be set.\n  @param[in]\n  sizeInBytes Size of buf buffer (in bytes) for verification.\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..\n  \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p buf is NULL or \\p sizeInBytes\n doesn't match the size of the internal storage for the selected attribute."]
    pub fn hipblasLtMatrixLayoutSetAttribute(
        matLayout: hipblasLtMatrixLayout_t,
        attr: hipblasLtMatrixLayoutAttribute_t,
        buf: *const ::std::os::raw::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Query attribute from a matrix descriptor\n\n  \\details\n  This function returns the value of the queried attribute belonging to a\n previously created matrix descriptor.\n\n  @param[in]\n  matLayout  Pointer to the previously created structure holding the matrix\n descriptor queried by this function. See \\ref hipblasLtMatrixLayout_t\n .\n  @param[in]\n  attr  \t    The attribute that will be retrieved by this function. See\n \\ref hipblasLtMatrixLayoutAttribute_t.\n  @param[out]\n  buf         Memory address containing the attribute value retrieved by this\n function.\n  @param[in]\n  sizeInBytes Size of \\p buf buffer (in bytes) for verification.\n  @param[out]\n  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If\n sizeInBytes is non-zero: then sizeWritten is the number of bytes actually\n written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed\n to write full contents.\n\n  \\retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully\n written to user memory. \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p\n sizeInBytes is 0 and \\p sizeWritten is NULL, or if \\p sizeInBytes is non-zero\n and \\p buf is NULL, or \\p sizeInBytes doesn't match size of internal storage\n for the selected attribute."]
    pub fn hipblasLtMatrixLayoutGetAttribute(
        matLayout: hipblasLtMatrixLayout_t,
        attr: hipblasLtMatrixLayoutAttribute_t,
        buf: *mut ::std::os::raw::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Create a matrix multiply descriptor\n\n  \\details\n  This function creates a matrix multiply descriptor by allocating the memory\n needed to hold its opaque structure.\n\n  @param[out]\n  matmulDesc  Pointer to the structure holding the matrix multiply descriptor\n created by this function. See \\ref hipblasLtMatmulDesc_t .\n  @param[in]\n  computeType  Enumerant that specifies the data precision for the matrix\n multiply descriptor this function creates. See \\ref hipblasLtComputeType_t .\n  @param[in]\n  scaleType  Enumerant that specifies the data precision for the matrix\n transform descriptor this function creates. See hipblasDataType_t.\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the descriptor was created successfully.\n  \\retval HIPBLAS_STATUS_ALLOC_FAILED If the memory could not be allocated."]
    pub fn hipblasLtMatmulDescCreate(
        matmulDesc: *mut hipblasLtMatmulDesc_t,
        computeType: hipblasLtComputeType_t,
        scaleType: hipblasDatatype_t,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Destory a matrix multiply descriptor\n\n  \\details\n  This function destroys a previously created matrix multiply descriptor\n object.\n\n  @param[in]\n  matmulDesc  Pointer to the structure holding the matrix multiply descriptor\n that should be destroyed by this function. See \\ref hipblasLtMatmulDesc_t .\n\n  \\retval HIPBLAS_STATUS_SUCCESS If operation was successful."]
    pub fn hipblasLtMatmulDescDestroy(matmulDesc: hipblasLtMatmulDesc_t) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief  Set attribute to a matrix multiply descriptor\n\n  \\details\n  This function sets the value of the specified attribute belonging to a\n previously created matrix multiply descriptor.\n\n  @param[in]\n  matmulDesc  Pointer to the previously created structure holding the matrix\n multiply descriptor queried by this function. See \\ref hipblasLtMatmulDesc_t\n .\n  @param[in]\n  attr  \tThe attribute that will be set by this function. See \\ref\n hipblasLtMatmulDescAttributes_t.\n  @param[in]\n  buf  The value to which the specified attribute should be set.\n  @param[in]\n  sizeInBytes Size of buf buffer (in bytes) for verification.\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..\n  \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p buf is NULL or \\p sizeInBytes\n doesn't match the size of the internal storage for the selected attribute."]
    pub fn hipblasLtMatmulDescSetAttribute(
        matmulDesc: hipblasLtMatmulDesc_t,
        attr: hipblasLtMatmulDescAttributes_t,
        buf: *const ::std::os::raw::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Query attribute from a matrix multiply descriptor\n\n  \\details\n  This function returns the value of the queried attribute belonging to a\n previously created matrix multiply descriptor.\n\n  @param[in]\n  matmulDesc  Pointer to the previously created structure holding the matrix\n multiply descriptor queried by this function. See \\ref hipblasLtMatmulDesc_t\n .\n  @param[in]\n  attr  \t    The attribute that will be retrieved by this function. See\n \\ref hipblasLtMatmulDescAttributes_t.\n  @param[out]\n  buf         Memory address containing the attribute value retrieved by this\n function.\n  @param[in]\n  sizeInBytes Size of \\p buf buffer (in bytes) for verification.\n  @param[out]\n  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If\n sizeInBytes is non-zero: then sizeWritten is the number of bytes actually\n written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed\n to write full contents.\n\n  \\retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully\n written to user memory. \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p\n sizeInBytes is 0 and \\p sizeWritten is NULL, or if \\p sizeInBytes is non-zero\n and \\p buf is NULL, or \\p sizeInBytes doesn't match size of internal storage\n for the selected attribute."]
    pub fn hipblasLtMatmulDescGetAttribute(
        matmulDesc: hipblasLtMatmulDesc_t,
        attr: hipblasLtMatmulDescAttributes_t,
        buf: *mut ::std::os::raw::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Create a preference descriptor\n\n  \\details\n  This function creates a matrix multiply heuristic search preferences\n descriptor by allocating the memory needed to hold its opaque structure.\n\n  @param[out]\n  pref  Pointer to the structure holding the matrix multiply preferences\n descriptor created by this function. see \\ref hipblasLtMatmulPreference_t .\n\n  \\retval HIPBLAS_STATUS_SUCCESS         If the descriptor was created\n successfully. \\retval HIPBLAS_STATUS_ALLOC_FAILED    If memory could not be\n allocated."]
    pub fn hipblasLtMatmulPreferenceCreate(
        pref: *mut hipblasLtMatmulPreference_t,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Destory a preferences descriptor\n\n  \\details\n  This function destroys a previously created matrix multiply preferences\n descriptor object.\n\n  @param[in]\n  pref  Pointer to the structure holding the matrix multiply preferences\n descriptor that should be destroyed by this function. See \\ref\n hipblasLtMatmulPreference_t .\n\n  \\retval HIPBLAS_STATUS_SUCCESS If operation was successful."]
    pub fn hipblasLtMatmulPreferenceDestroy(pref: hipblasLtMatmulPreference_t) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Set attribute to a preference descriptor\n\n  \\details\n  This function sets the value of the specified attribute belonging to a\n previously created matrix multiply preferences descriptor.\n\n  @param[in]\n  pref        Pointer to the previously created structure holding the matrix\n multiply preferences descriptor queried by this function. See \\ref\n hipblasLtMatmulPreference_t\n  @param[in]\n  attr  \t    The attribute that will be set by this function. See \\ref\n hipblasLtMatmulPreferenceAttributes_t.\n  @param[in]\n  buf         The value to which the specified attribute should be set.\n  @param[in]\n  sizeInBytes Size of \\p buf buffer (in bytes) for verification.\n\n  \\retval HIPBLAS_STATUS_SUCCESS If the attribute was set successfully..\n  \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p buf is NULL or \\p sizeInBytes\n doesn't match the size of the internal storage for the selected attribute."]
    pub fn hipblasLtMatmulPreferenceSetAttribute(
        pref: hipblasLtMatmulPreference_t,
        attr: hipblasLtMatmulPreferenceAttributes_t,
        buf: *const ::std::os::raw::c_void,
        sizeInBytes: usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Query attribute from a preference descriptor\n\n  \\details\n  This function returns the value of the queried attribute belonging to a\n previously created matrix multiply heuristic search preferences descriptor.\n\n  @param[in]\n  pref        Pointer to the previously created structure holding the matrix\n multiply heuristic search preferences descriptor queried by this function.\n See \\ref hipblasLtMatmulPreference_t.\n  @param[in]\n  attr  \t    The attribute that will be retrieved by this function. See\n \\ref hipblasLtMatmulPreferenceAttributes_t.\n  @param[out]\n  buf         Memory address containing the attribute value retrieved by this\n function.\n  @param[in]\n  sizeInBytes Size of \\p buf buffer (in bytes) for verification.\n  @param[out]\n  sizeWritten Valid only when the return value is HIPBLAS_STATUS_SUCCESS. If\n sizeInBytes is non-zero: then sizeWritten is the number of bytes actually\n written; if sizeInBytes is 0: then sizeWritten is the number of bytes needed\n to write full contents.\n\n  \\retval HIPBLAS_STATUS_SUCCESS       If attribute's value was successfully\n written to user memory. \\retval HIPBLAS_STATUS_INVALID_VALUE If \\p\n sizeInBytes is 0 and \\p sizeWritten is NULL, or if \\p sizeInBytes is non-zero\n and \\p buf is NULL, or \\p sizeInBytes doesn't match size of internal storage\n for the selected attribute."]
    pub fn hipblasLtMatmulPreferenceGetAttribute(
        pref: hipblasLtMatmulPreference_t,
        attr: hipblasLtMatmulPreferenceAttributes_t,
        buf: *mut ::std::os::raw::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Retrieve the possible algorithms\n\n  \\details\n  This function retrieves the possible algorithms for the matrix multiply\n operation hipblasLtMatmul() function with the given input matrices A, B and\n C, and the output matrix D. The output is placed in heuristicResultsArray[]\n in the order of increasing estimated compute time.\n\n  @param[in]\n  handle                  Pointer to the allocated hipBLASLt handle for the\n hipBLASLt context. See \\ref hipblasLtHandle_t .\n  @param[in]\n  matmulDesc              Handle to a previously created matrix multiplication\n descriptor of type \\ref hipblasLtMatmulDesc_t .\n  @param[in]\n  Adesc,Bdesc,Cdesc,Ddesc Handles to the previously created matrix layout\n descriptors of the type \\ref hipblasLtMatrixLayout_t .\n  @param[in]\n  pref                    Pointer to the structure holding the heuristic\n search preferences descriptor. See \\ref hipblasLtMatmulPreference_t .\n  @param[in]\n  requestedAlgoCount      Size of the \\p heuristicResultsArray (in elements).\n This is the requested maximum number of algorithms to return.\n  @param[out]\n  heuristicResultsArray[] Array containing the algorithm heuristics and\n associated runtime characteristics, returned by this function, in the order\n of increasing estimated compute time.\n  @param[out]\n  returnAlgoCount         Number of algorithms returned by this function. This\n is the number of \\p heuristicResultsArray elements written.\n\n  \\retval HIPBLAS_STATUS_SUCCESS           If query was successful. Inspect\n heuristicResultsArray[0 to (returnAlgoCount -1)].state for the status of the\n results. \\retval HIPBLAS_STATUS_NOT_SUPPORTED     If no heuristic function\n available for current configuration. \\retval HIPBLAS_STATUS_INVALID_VALUE If\n \\p requestedAlgoCount is less or equal to zero."]
    pub fn hipblasLtMatmulAlgoGetHeuristic(
        handle: hipblasLtHandle_t,
        matmulDesc: hipblasLtMatmulDesc_t,
        Adesc: hipblasLtMatrixLayout_t,
        Bdesc: hipblasLtMatrixLayout_t,
        Cdesc: hipblasLtMatrixLayout_t,
        Ddesc: hipblasLtMatrixLayout_t,
        pref: hipblasLtMatmulPreference_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        heuristicResultsArray: *mut hipblasLtMatmulHeuristicResult_t,
        returnAlgoCount: *mut ::std::os::raw::c_int,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtExtGroupedGemmAlgoGetHeuristic(
        groupedgemm: hipblasLtExtGroupedGemm_t,
        pref: hipblasLtMatmulPreference_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        heuristicResultsArray: *mut hipblasLtMatmulHeuristicResult_t,
        returnAlgoCount: *mut ::std::os::raw::c_int,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    #[doc = " \\ingroup library_module\n  \\brief Retrieve the possible algorithms\n\n  \\details\n  This function computes the matrix multiplication of matrices A and B to\n produce the output matrix D, according to the following operation: \\p D = \\p\n alpha*( \\p A *\\p B) + \\p beta*( \\p C ), where \\p A, \\p B, and \\p C are input\n matrices, and \\p alpha and \\p beta are input scalars. Note: This function\n supports both in-place matrix multiplication (C == D and Cdesc == Ddesc) and\n out-of-place matrix multiplication (C != D, both matrices must have the same\n data type, number of rows, number of columns, batch size, and memory order).\n In the out-of-place case, the leading dimension of C can be different from\n the leading dimension of D. Specifically the leading dimension of C can be 0\n to achieve row or column broadcast. If Cdesc is omitted, this function\n assumes it to be equal to Ddesc.\n\n  @param[in]\n  handle                  Pointer to the allocated hipBLASLt handle for the\n hipBLASLt context. See \\ref hipblasLtHandle_t .\n  @param[in]\n  matmulDesc              Handle to a previously created matrix multiplication\n descriptor of type \\ref hipblasLtMatmulDesc_t .\n  @param[in]\n  alpha,beta              Pointers to the scalars used in the multiplication.\n  @param[in]\n  Adesc,Bdesc,Cdesc,Ddesc Handles to the previously created matrix layout\n descriptors of the type \\ref hipblasLtMatrixLayout_t .\n  @param[in]\n  A,B,C                   Pointers to the GPU memory associated with the\n corresponding descriptors \\p Adesc, \\p Bdesc and \\p Cdesc .\n  @param[out]\n  D                       Pointer to the GPU memory associated with the\n descriptor \\p Ddesc .\n  @param[in]\n  algo                    Handle for matrix multiplication algorithm to be\n used. See \\ref hipblasLtMatmulAlgo_t. When NULL, an implicit heuristics query\n with default search preferences will be performed to determine actual\n algorithm to use.\n  @param[in]\n  workspace               Pointer to the workspace buffer allocated in the GPU\n memory. Pointer must be 16B aligned (that is, lowest 4 bits of address must\n be 0).\n  @param[in]\n  workspaceSizeInBytes    Size of the workspace.\n  @param[in]\n  stream                  The HIP stream where all the GPU work will be\n submitted.\n\n  \\retval HIPBLAS_STATUS_SUCCESS           If the operation completed\n successfully. \\retval HIPBLAS_STATUS_EXECUTION_FAILED  If HIP reported an\n execution error from the device. \\retval HIPBLAS_STATUS_ARCH_MISMATCH     If\n the configured operation cannot be run using the selected device. \\retval\n HIPBLAS_STATUS_NOT_SUPPORTED     If the current implementation on the\n selected device doesn't support the configured operation. \\retval\n HIPBLAS_STATUS_INVALID_VALUE     If the parameters are unexpectedly NULL, in\n conflict or in an impossible configuration. For example, when\n workspaceSizeInBytes is less than workspace required by the configured algo.\n  \\retval HIBLAS_STATUS_NOT_INITIALIZED    If hipBLASLt handle has not been\n initialized."]
    pub fn hipblasLtMatmul(
        handle: hipblasLtHandle_t,
        matmulDesc: hipblasLtMatmulDesc_t,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Adesc: hipblasLtMatrixLayout_t,
        B: *const ::std::os::raw::c_void,
        Bdesc: hipblasLtMatrixLayout_t,
        beta: *const ::std::os::raw::c_void,
        C: *const ::std::os::raw::c_void,
        Cdesc: hipblasLtMatrixLayout_t,
        D: *mut ::std::os::raw::c_void,
        Ddesc: hipblasLtMatrixLayout_t,
        algo: *const hipblasLtMatmulAlgo_t,
        workspace: *mut ::std::os::raw::c_void,
        workspaceSizeInBytes: usize,
        stream: hipStream_t,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtExtGroupedGemmDestroy(
        groupedgemm: hipblasLtExtGroupedGemm_t,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtExtGroupedGemmMakeArgument(
        groupedgemm: hipblasLtExtGroupedGemm_t,
        algo: *const hipblasLtMatmulAlgo_t,
        workspace: *mut ::std::os::raw::c_void,
        stream: hipStream_t,
    ) -> hipblasStatus_t;
}
extern "C" {
    #[must_use]
    pub fn hipblasLtExtGroupedGemmRun(
        groupedgemm: hipblasLtExtGroupedGemm_t,
        stream: hipStream_t,
    ) -> hipblasStatus_t;
}
