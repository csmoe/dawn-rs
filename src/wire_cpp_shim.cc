#include <cstdint>
#include <cstring>
#include <memory>
#include <vector>

#include "dawn/dawn_proc.h"
#include "dawn/native/DawnNative.h"
#include "dawn/wire/client/webgpu.h"
#include "dawn/wire/WireClient.h"
#include "dawn/wire/WireServer.h"

// This shim intentionally does not implement any wire protocol encoding/decoding.
// Dawn's generated wire runtime handles command (de)serialization internally.
// The shim only bridges serializer callbacks, proc-table setup, and HandleCommands routing.

extern "C" {

struct DawnRsWireSerializerCallbacks {
    void* userdata;
    void (*on_flush)(void* userdata, const uint8_t* data, size_t size);
    void (*on_error)(void* userdata, const char* message, size_t message_len);
    size_t max_allocation_size;
};

struct DawnRsWireHandle {
    uint32_t id;
    uint32_t generation;
};

struct DawnRsWireReservedInstance {
    void* instance;
    DawnRsWireHandle handle;
};

struct DawnRsWireReservedSurface {
    void* surface;
    DawnRsWireHandle instance_handle;
    DawnRsWireHandle handle;
};

struct DawnRsWireReservedTexture {
    void* texture;
    DawnRsWireHandle handle;
    DawnRsWireHandle device_handle;
};

}  // extern "C"

namespace {

class SerializerBridge final : public dawn::wire::CommandSerializer {
  public:
    explicit SerializerBridge(const DawnRsWireSerializerCallbacks& callbacks)
        : callbacks_(callbacks) {}
    ~SerializerBridge() override = default;

    size_t GetMaximumAllocationSize() const override {
        if (callbacks_.max_allocation_size == 0) {
            // WireClient/WireServer internally use Dawn's ChunkedCommandSerializer.
            // This value controls when large commands are emitted as chunked commands.
            return 64 * 1024;
        }
        return callbacks_.max_allocation_size;
    }

    void* GetCmdSpace(size_t size) override {
        const size_t old_size = buffer_.size();
        buffer_.resize(old_size + size);
        return buffer_.data() + old_size;
    }

    bool Flush() override {
        if (!callbacks_.on_flush) {
            return false;
        }
        if (buffer_.empty()) {
            return true;
        }
        callbacks_.on_flush(callbacks_.userdata, buffer_.data(), buffer_.size());
        buffer_.clear();
        return true;
    }

    void OnSerializeError() override { EmitError("wire serializer error"); }

  private:
    void EmitError(const char* message) {
        if (!callbacks_.on_error) {
            return;
        }
        callbacks_.on_error(callbacks_.userdata, message, std::strlen(message));
    }

    DawnRsWireSerializerCallbacks callbacks_;
    std::vector<uint8_t> buffer_;
};

struct DawnRsWireClient {
    std::unique_ptr<SerializerBridge> serializer;
    std::unique_ptr<dawn::wire::WireClient> wire;
};

struct DawnRsWireServer {
    std::unique_ptr<SerializerBridge> serializer;
    std::unique_ptr<dawn::wire::WireServer> wire;
    std::vector<WGPUTexture> retained_injected_textures;
};
// The wire server lifecycle (construction + Inject* + HandleCommands) is currently exposed
// through C++ dawn::wire APIs, so we keep this control plane in C++.

// dawnProcSetProcs stores a pointer, so the table must outlive the call site.
static DawnProcTable gWireClientProcs = {};
static bool gWireClientProcsInitialized = false;

#include "wire_client_proc_table_autogen.inc"

}  // namespace

extern "C" {

DawnRsWireClient* dawn_rs_wire_client_create(const DawnRsWireSerializerCallbacks* callbacks) {
    if (!callbacks) {
        return nullptr;
    }
    auto out = std::make_unique<DawnRsWireClient>();
    out->serializer = std::make_unique<SerializerBridge>(*callbacks);
    dawn::wire::WireClientDescriptor desc = {};
    desc.serializer = out->serializer.get();
    out->wire = std::make_unique<dawn::wire::WireClient>(desc);
    return out.release();
}

void dawn_rs_wire_client_destroy(DawnRsWireClient* client) {
    delete client;
}

bool dawn_rs_wire_client_handle_commands(DawnRsWireClient* client,
                                         const uint8_t* data,
                                         size_t size) {
    if (!client || (size > 0 && !data)) {
        return false;
    }
    return client->wire->HandleCommands(reinterpret_cast<const volatile char*>(data), size) !=
           nullptr;
}

bool dawn_rs_wire_client_flush(DawnRsWireClient* client) {
    if (!client) {
        return false;
    }
    return client->serializer->Flush();
}

void dawn_rs_wire_client_disconnect(DawnRsWireClient* client) {
    if (!client) {
        return;
    }
    client->wire->Disconnect();
}

DawnRsWireReservedInstance dawn_rs_wire_client_reserve_instance(DawnRsWireClient* client) {
    DawnRsWireReservedInstance out = {};
    if (!client) {
        return out;
    }
    dawn::wire::ReservedInstance reserved = client->wire->ReserveInstance(nullptr);
    out.instance = reserved.instance;
    out.handle.id = reserved.handle.id;
    out.handle.generation = reserved.handle.generation;
    return out;
}

DawnRsWireReservedSurface dawn_rs_wire_client_reserve_surface(DawnRsWireClient* client,
                                                              void* instance) {
    DawnRsWireReservedSurface out = {};
    if (!client || !instance) {
        return out;
    }
    WGPUSurfaceCapabilities caps = {};
    dawn::wire::ReservedSurface reserved =
        client->wire->ReserveSurface(reinterpret_cast<WGPUInstance>(instance), &caps);
    out.surface = reserved.surface;
    out.instance_handle.id = reserved.instanceHandle.id;
    out.instance_handle.generation = reserved.instanceHandle.generation;
    out.handle.id = reserved.handle.id;
    out.handle.generation = reserved.handle.generation;
    return out;
}

DawnRsWireHandle dawn_rs_wire_client_get_device_handle(DawnRsWireClient* client, void* device) {
    DawnRsWireHandle out = {};
    if (!client || !device) {
        return out;
    }
    dawn::wire::Handle h = client->wire->GetWireHandle(reinterpret_cast<WGPUDevice>(device));
    out.id = h.id;
    out.generation = h.generation;
    return out;
}

DawnRsWireReservedTexture dawn_rs_wire_client_reserve_bgra8_texture_2d(DawnRsWireClient* client,
                                                                        void* device,
                                                                        uint32_t width,
                                                                        uint32_t height) {
    DawnRsWireReservedTexture out = {};
    if (!client || !device || width == 0 || height == 0) {
        return out;
    }
    WGPUExtent3D size = {};
    size.width = width;
    size.height = height;
    size.depthOrArrayLayers = 1;
    WGPUTextureDescriptor desc = {};
    desc.dimension = WGPUTextureDimension_2D;
    desc.format = WGPUTextureFormat_BGRA8Unorm;
    desc.size = size;
    desc.mipLevelCount = 1;
    desc.sampleCount = 1;
    desc.usage = WGPUTextureUsage_TextureBinding | WGPUTextureUsage_CopySrc |
                 WGPUTextureUsage_CopyDst | WGPUTextureUsage_RenderAttachment;
    dawn::wire::ReservedTexture reserved =
        client->wire->ReserveTexture(reinterpret_cast<WGPUDevice>(device), &desc);
    out.texture = reserved.texture;
    out.handle.id = reserved.handle.id;
    out.handle.generation = reserved.handle.generation;
    out.device_handle.id = reserved.deviceHandle.id;
    out.device_handle.generation = reserved.deviceHandle.generation;
    return out;
}

DawnRsWireServer* dawn_rs_wire_server_create_native(const DawnRsWireSerializerCallbacks* callbacks,
                                                    bool use_spontaneous_callbacks) {
    if (!callbacks) {
        return nullptr;
    }
    auto out = std::make_unique<DawnRsWireServer>();
    out->serializer = std::make_unique<SerializerBridge>(*callbacks);

    dawn::wire::WireServerDescriptor desc = {};
    desc.procs = &dawn::native::GetProcs();
    desc.serializer = out->serializer.get();
    desc.useSpontaneousCallbacks = use_spontaneous_callbacks;
    out->wire = std::make_unique<dawn::wire::WireServer>(desc);
    return out.release();
}

void dawn_rs_wire_server_destroy(DawnRsWireServer* server) {
    if (server) {
        const DawnProcTable& procs = dawn::native::GetProcs();
        for (WGPUTexture texture : server->retained_injected_textures) {
            if (texture) {
                procs.textureRelease(texture);
            }
        }
        server->retained_injected_textures.clear();
    }
    delete server;
}

bool dawn_rs_wire_server_handle_commands(DawnRsWireServer* server,
                                         const uint8_t* data,
                                         size_t size) {
    if (!server || (size > 0 && !data)) {
        return false;
    }
    return server->wire->HandleCommands(reinterpret_cast<const volatile char*>(data), size) !=
           nullptr;
}

bool dawn_rs_wire_server_flush(DawnRsWireServer* server) {
    if (!server) {
        return false;
    }
    return server->serializer->Flush();
}

bool dawn_rs_wire_server_inject_instance(DawnRsWireServer* server,
                                         void* instance,
                                         DawnRsWireHandle handle) {
    if (!server || !instance) {
        return false;
    }
    dawn::wire::Handle h = {};
    h.id = handle.id;
    h.generation = handle.generation;
    return server->wire->InjectInstance(reinterpret_cast<WGPUInstance>(instance), h);
}

bool dawn_rs_wire_server_inject_surface(DawnRsWireServer* server,
                                        void* surface,
                                        DawnRsWireHandle handle,
                                        DawnRsWireHandle instance_handle) {
    if (!server || !surface) {
        return false;
    }
    dawn::wire::Handle h = {};
    h.id = handle.id;
    h.generation = handle.generation;
    dawn::wire::Handle instance_h = {};
    instance_h.id = instance_handle.id;
    instance_h.generation = instance_handle.generation;
    return server->wire->InjectSurface(reinterpret_cast<WGPUSurface>(surface), h, instance_h);
}

bool dawn_rs_wire_server_inject_texture(DawnRsWireServer* server,
                                        void* texture,
                                        DawnRsWireHandle handle,
                                        DawnRsWireHandle device_handle) {
    if (!server || !texture) {
        return false;
    }
    dawn::wire::Handle h = {};
    h.id = handle.id;
    h.generation = handle.generation;
    dawn::wire::Handle device_h = {};
    device_h.id = device_handle.id;
    device_h.generation = device_handle.generation;
    return server->wire->InjectTexture(reinterpret_cast<WGPUTexture>(texture), h, device_h);
}

bool dawn_rs_wire_server_inject_buffer(DawnRsWireServer* server,
                                       void* buffer,
                                       DawnRsWireHandle handle,
                                       DawnRsWireHandle device_handle) {
    if (!server || !buffer) {
        return false;
    }
    dawn::wire::Handle h = {};
    h.id = handle.id;
    h.generation = handle.generation;
    dawn::wire::Handle device_h = {};
    device_h.id = device_handle.id;
    device_h.generation = device_handle.generation;
    return server->wire->InjectBuffer(reinterpret_cast<WGPUBuffer>(buffer), h, device_h);
}

void* dawn_rs_wire_server_get_device(DawnRsWireServer* server, DawnRsWireHandle handle) {
    if (!server) {
        return nullptr;
    }
    return reinterpret_cast<void*>(server->wire->GetDevice(handle.id, handle.generation));
}

namespace {
bool DawnRsInjectImportedSharedTexture(DawnRsWireServer* server,
                                       WGPUDevice device,
                                       WGPUSharedTextureMemoryDescriptor* shared_desc,
                                       uint32_t width,
                                       uint32_t height,
                                       DawnRsWireHandle texture_handle,
                                       DawnRsWireHandle device_handle) {
    const DawnProcTable& procs = dawn::native::GetProcs();
    WGPUSharedTextureMemory shared = procs.deviceImportSharedTextureMemory(device, shared_desc);
    if (!shared) {
        return false;
    }
    WGPUExtent3D size = {};
    size.width = width;
    size.height = height;
    size.depthOrArrayLayers = 1;
    WGPUTextureDescriptor tex_desc = {};
    tex_desc.dimension = WGPUTextureDimension_2D;
    tex_desc.format = WGPUTextureFormat_BGRA8Unorm;
    tex_desc.size = size;
    tex_desc.mipLevelCount = 1;
    tex_desc.sampleCount = 1;
    tex_desc.usage = WGPUTextureUsage_TextureBinding | WGPUTextureUsage_CopySrc |
                     WGPUTextureUsage_CopyDst | WGPUTextureUsage_RenderAttachment;
    WGPUTexture shared_texture = procs.sharedTextureMemoryCreateTexture(shared, &tex_desc);
    if (!shared_texture) {
        procs.sharedTextureMemoryRelease(shared);
        return false;
    }
    WGPUSharedTextureMemoryBeginAccessDescriptor begin_access = {};
    begin_access.concurrentRead = false;
    begin_access.initialized = true;
    if (procs.sharedTextureMemoryBeginAccess(shared, shared_texture, &begin_access) !=
        WGPUStatus_Success) {
        procs.textureRelease(shared_texture);
        procs.sharedTextureMemoryRelease(shared);
        return false;
    }

    WGPUTextureDescriptor local_desc = tex_desc;
    local_desc.usage = WGPUTextureUsage_TextureBinding | WGPUTextureUsage_CopyDst |
                       WGPUTextureUsage_RenderAttachment;
    WGPUTexture texture = procs.deviceCreateTexture(device, &local_desc);
    if (!texture) {
        WGPUSharedTextureMemoryEndAccessState end_access = {};
        procs.sharedTextureMemoryEndAccess(shared, shared_texture, &end_access);
        procs.sharedTextureMemoryEndAccessStateFreeMembers(end_access);
        procs.textureRelease(shared_texture);
        procs.sharedTextureMemoryRelease(shared);
        return false;
    }
    WGPUCommandEncoder encoder = procs.deviceCreateCommandEncoder(device, nullptr);
    if (!encoder) {
        WGPUSharedTextureMemoryEndAccessState end_access = {};
        procs.sharedTextureMemoryEndAccess(shared, shared_texture, &end_access);
        procs.sharedTextureMemoryEndAccessStateFreeMembers(end_access);
        procs.textureRelease(shared_texture);
        procs.textureRelease(texture);
        procs.sharedTextureMemoryRelease(shared);
        return false;
    }
    WGPUTexelCopyTextureInfo src = {};
    src.texture = shared_texture;
    src.mipLevel = 0;
    src.origin = {0, 0, 0};
    src.aspect = WGPUTextureAspect_All;
    WGPUTexelCopyTextureInfo dst = {};
    dst.texture = texture;
    dst.mipLevel = 0;
    dst.origin = {0, 0, 0};
    dst.aspect = WGPUTextureAspect_All;
    procs.commandEncoderCopyTextureToTexture(encoder, &src, &dst, &size);
    WGPUCommandBuffer command = procs.commandEncoderFinish(encoder, nullptr);
    procs.commandEncoderRelease(encoder);
    if (!command) {
        WGPUSharedTextureMemoryEndAccessState end_access = {};
        procs.sharedTextureMemoryEndAccess(shared, shared_texture, &end_access);
        procs.sharedTextureMemoryEndAccessStateFreeMembers(end_access);
        procs.textureRelease(shared_texture);
        procs.textureRelease(texture);
        procs.sharedTextureMemoryRelease(shared);
        return false;
    }
    WGPUQueue queue = procs.deviceGetQueue(device);
    procs.queueSubmit(queue, 1, &command);
    procs.commandBufferRelease(command);
    WGPUSharedTextureMemoryEndAccessState end_access = {};
    procs.sharedTextureMemoryEndAccess(shared, shared_texture, &end_access);
    procs.sharedTextureMemoryEndAccessStateFreeMembers(end_access);
    procs.textureRelease(shared_texture);
    procs.sharedTextureMemoryRelease(shared);

    dawn::wire::Handle tex_h = {};
    tex_h.id = texture_handle.id;
    tex_h.generation = texture_handle.generation;
    dawn::wire::Handle dev_h = {};
    dev_h.id = device_handle.id;
    dev_h.generation = device_handle.generation;
    const bool ok = server->wire->InjectTexture(texture, tex_h, dev_h);
    if (ok) {
        server->retained_injected_textures.push_back(texture);
    } else {
        procs.textureRelease(texture);
    }
    return ok;
}
}  // namespace

bool dawn_rs_wire_server_inject_iosurface_texture(DawnRsWireServer* server,
                                                   void* io_surface,
                                                   uint32_t width,
                                                   uint32_t height,
                                                   DawnRsWireHandle texture_handle,
                                                   DawnRsWireHandle device_handle) {
    if (!server || !io_surface || width == 0 || height == 0) {
        return false;
    }
    WGPUDevice device = server->wire->GetDevice(device_handle.id, device_handle.generation);
    if (!device) {
        return false;
    }
    WGPUSharedTextureMemoryDescriptor shared_desc = {};
    WGPUSharedTextureMemoryIOSurfaceDescriptor ios_desc = {};
    ios_desc.chain.sType = WGPUSType_SharedTextureMemoryIOSurfaceDescriptor;
    ios_desc.ioSurface = io_surface;
    ios_desc.allowStorageBinding = false;
    shared_desc.nextInChain = &ios_desc.chain;
    return DawnRsInjectImportedSharedTexture(server, device, &shared_desc, width, height,
                                             texture_handle, device_handle);
}

bool dawn_rs_wire_server_inject_dxgi_texture(DawnRsWireServer* server,
                                             void* shared_handle,
                                             bool use_keyed_mutex,
                                             uint32_t width,
                                             uint32_t height,
                                             DawnRsWireHandle texture_handle,
                                             DawnRsWireHandle device_handle) {
    if (!server || !shared_handle || width == 0 || height == 0) {
        return false;
    }
    WGPUDevice device = server->wire->GetDevice(device_handle.id, device_handle.generation);
    if (!device) {
        return false;
    }
    WGPUSharedTextureMemoryDescriptor shared_desc = {};
    WGPUSharedTextureMemoryDXGISharedHandleDescriptor dxgi_desc = {};
    dxgi_desc.chain.sType = WGPUSType_SharedTextureMemoryDXGISharedHandleDescriptor;
    dxgi_desc.handle = shared_handle;
    dxgi_desc.useKeyedMutex = use_keyed_mutex;
    shared_desc.nextInChain = &dxgi_desc.chain;
    return DawnRsInjectImportedSharedTexture(server, device, &shared_desc, width, height,
                                             texture_handle, device_handle);
}

bool dawn_rs_wire_server_inject_dmabuf_texture(DawnRsWireServer* server,
                                               int fd,
                                               uint32_t drm_format,
                                               uint64_t drm_modifier,
                                               uint32_t stride,
                                               uint64_t offset,
                                               uint32_t width,
                                               uint32_t height,
                                               DawnRsWireHandle texture_handle,
                                               DawnRsWireHandle device_handle) {
    if (!server || fd < 0 || width == 0 || height == 0) {
        return false;
    }
    WGPUDevice device = server->wire->GetDevice(device_handle.id, device_handle.generation);
    if (!device) {
        return false;
    }
    WGPUSharedTextureMemoryDmaBufPlane plane = {};
    plane.fd = fd;
    plane.offset = offset;
    plane.stride = stride;
    WGPUSharedTextureMemoryDescriptor shared_desc = {};
    WGPUSharedTextureMemoryDmaBufDescriptor dma_desc = {};
    dma_desc.chain.sType = WGPUSType_SharedTextureMemoryDmaBufDescriptor;
    dma_desc.size.width = width;
    dma_desc.size.height = height;
    dma_desc.size.depthOrArrayLayers = 1;
    dma_desc.drmFormat = drm_format;
    dma_desc.drmModifier = drm_modifier;
    dma_desc.planeCount = 1;
    dma_desc.planes = &plane;
    shared_desc.nextInChain = &dma_desc.chain;
    return DawnRsInjectImportedSharedTexture(server, device, &shared_desc, width, height,
                                             texture_handle, device_handle);
}

void dawn_rs_wire_set_client_procs() {
    if (!gWireClientProcsInitialized) {
        gWireClientProcs = DawnRsBuildWireClientProcTableFromCAPI();
        gWireClientProcsInitialized = true;
    }
    dawnProcSetProcs(&gWireClientProcs);
}

void dawn_rs_wire_set_native_procs() {
    const DawnProcTable& procs = dawn::native::GetProcs();
    dawnProcSetProcs(&procs);
}

void dawn_rs_wire_clear_procs() {
    dawnProcSetProcs(nullptr);
}

}  // extern "C"
