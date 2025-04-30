#include <cuda_runtime.h>
#include <cuda.h>

extern void **CUDARTAPI __cudaRegisterFatBinary(void *fatCubin);

extern void CUDARTAPI __cudaRegisterFatBinaryEnd(void **fatCubinHandle);

extern void CUDARTAPI __cudaUnregisterFatBinary(void **fatCubinHandle);

extern void CUDARTAPI __cudaRegisterVar(void **fatCubinHandle, char *hostVar,
                                        char *deviceAddress,
                                        const char *deviceName, int ext,
                                        size_t size, int constant, int global);

extern void CUDARTAPI __cudaRegisterManagedVar(
    void **fatCubinHandle, void **hostVarPtrAddress, char *deviceAddress,
    const char *deviceName, int ext, size_t size, int constant, int global);

extern char CUDARTAPI __cudaInitModule(void **fatCubinHandle);

extern void CUDARTAPI __cudaRegisterTexture(
    void **fatCubinHandle, const struct textureReference *hostVar,
    const void **deviceAddress, const char *deviceName, int dim, int norm,
    int ext);

extern void CUDARTAPI __cudaRegisterSurface(
    void **fatCubinHandle, const struct surfaceReference *hostVar,
    const void **deviceAddress, const char *deviceName, int dim, int ext);

extern void CUDARTAPI __cudaRegisterFunction(
        void   **fatCubinHandle,
  const char    *hostFun,
        char    *deviceFun,
  const char    *deviceName,
        int      thread_limit,
        uint3   *tid,
        uint3   *bid,
        dim3    *bDim,
        dim3    *gDim,
        int     *wSize
);
