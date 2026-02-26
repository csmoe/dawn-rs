#include <cstdint>
#include <cstring>
#include <memory>
#include <vector>

#include "dawn/dawn_proc.h"
#include "dawn/native/DawnNative.h"
#include "dawn/wire/WireClient.h"
#include "dawn/wire/WireServer.h"

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
};

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
    if (!client || !data) {
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
    delete server;
}

bool dawn_rs_wire_server_handle_commands(DawnRsWireServer* server,
                                         const uint8_t* data,
                                         size_t size) {
    if (!server || !data) {
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

void dawn_rs_wire_set_client_procs() {
    const DawnProcTable& procs = dawn::wire::client::GetProcs();
    dawnProcSetProcs(&procs);
}

void dawn_rs_wire_set_native_procs() {
    const DawnProcTable& procs = dawn::native::GetProcs();
    dawnProcSetProcs(&procs);
}

void dawn_rs_wire_clear_procs() {
    dawnProcSetProcs(nullptr);
}

}  // extern "C"
