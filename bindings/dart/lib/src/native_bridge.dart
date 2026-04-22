/// Native FFI bridge to the or-bridge Rust dynamic library.
///
/// ## Allocation contract
///
/// Strings flow across the FFI boundary in two directions:
///
/// * **Dart → Rust**: allocated with the system C allocator (`malloc`) and
///   freed with the system `free` after the Rust call returns.
/// * **Rust → Dart**: allocated by the Rust bridge (via `CString` / the Rust
///   global allocator) and **must** be freed with `orchustr_bridge_free_string`
///   (NOT the system `free`) to avoid heap corruption.
import "dart:convert";
import "dart:ffi";
import "dart:io";

typedef _RustStringFnNative = Pointer<Int8> Function();
typedef _RustStringFn = Pointer<Int8> Function();
typedef _RenderNative = Pointer<Int8> Function(
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Pointer<Int8>>,
);
typedef _RenderDart = Pointer<Int8> Function(
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Pointer<Int8>>,
);
typedef _NormalizeNative = Pointer<Int8> Function(
    Pointer<Int8>, Pointer<Pointer<Int8>>);
typedef _NormalizeDart = Pointer<Int8> Function(
    Pointer<Int8>, Pointer<Pointer<Int8>>);
typedef _CatalogNative = Pointer<Int8> Function(Pointer<Pointer<Int8>>);
typedef _CatalogDart = Pointer<Int8> Function(Pointer<Pointer<Int8>>);
typedef _InvokeNative = Pointer<Int8> Function(
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Pointer<Int8>>,
);
typedef _InvokeDart = Pointer<Int8> Function(
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Int8>,
  Pointer<Pointer<Int8>>,
);
typedef _BridgeFreeNative = Void Function(Pointer<Int8>);
typedef _BridgeFreeDart = void Function(Pointer<Int8>);
typedef _MallocNative = Pointer<Void> Function(IntPtr);
typedef _MallocDart = Pointer<Void> Function(int);
typedef _SystemFreeNative = Void Function(Pointer<Void>);
typedef _SystemFreeDart = void Function(Pointer<Void>);
typedef _StrlenNative = IntPtr Function(Pointer<Uint8>);
typedef _StrlenDart = int Function(Pointer<Uint8>);

String? _configuredLibraryPath;
OrchustrNativeBridge? _cachedBridge;
bool _attemptedBridgeLoad = false;

void configureNativeBridge({String? libraryPath}) {
  _configuredLibraryPath = libraryPath;
  _cachedBridge = null;
  _attemptedBridgeLoad = false;
}

bool get nativeBridgeAvailable => OrchustrNativeBridge.instance != null;

final class OrchustrNativeBridge {
  OrchustrNativeBridge._(DynamicLibrary library)
      : _version = library.lookupFunction<_RustStringFnNative, _RustStringFn>(
          "orchustr_bridge_version",
        ),
        _render = library.lookupFunction<_RenderNative, _RenderDart>(
          "orchustr_render_prompt_json",
        ),
        _normalize = library.lookupFunction<_NormalizeNative, _NormalizeDart>(
          "orchustr_normalize_state_json",
        ),
        _catalog = library.lookupFunction<_CatalogNative, _CatalogDart>(
          "orchustr_workspace_catalog_json",
        ),
        _invoke = library.lookupFunction<_InvokeNative, _InvokeDart>(
          "orchustr_invoke_crate_json",
        ),
        _freeBridgeString =
            library.lookupFunction<_BridgeFreeNative, _BridgeFreeDart>(
          "orchustr_bridge_free_string",
        );

  final _RustStringFn _version;
  final _RenderDart _render;
  final _NormalizeDart _normalize;
  final _CatalogDart _catalog;
  final _InvokeDart _invoke;
  final _BridgeFreeDart _freeBridgeString;

  static OrchustrNativeBridge? get instance {
    if (_attemptedBridgeLoad) {
      return _cachedBridge;
    }
    _attemptedBridgeLoad = true;
    for (final candidate in _bridgeCandidates()) {
      try {
        _cachedBridge = OrchustrNativeBridge._(DynamicLibrary.open(candidate));
        return _cachedBridge;
      } on ArgumentError {
        continue;
      }
    }
    return null;
  }

  String version() => _takeBridgeString(_version());

  String renderPromptJson(String template, String contextJson) {
    final templatePtr = _allocateCString(template);
    final contextPtr = _allocateCString(contextJson);
    final errorSlot = _malloc(sizeOf<IntPtr>()).cast<Pointer<Int8>>();
    try {
      errorSlot.value = Pointer<Int8>.fromAddress(0);
      final result = _render(templatePtr, contextPtr, errorSlot);
      if (result.address != 0) {
        return _takeBridgeString(result);
      }
      final errorPointer = errorSlot.value;
      throw StateError(
        errorPointer.address == 0
            ? "native bridge render failed"
            : _takeBridgeString(errorPointer),
      );
    } finally {
      _free(templatePtr.cast<Void>());
      _free(contextPtr.cast<Void>());
      _free(errorSlot.cast<Void>());
    }
  }

  String normalizeStateJson(String rawState) {
    final rawStatePtr = _allocateCString(rawState);
    final errorSlot = _malloc(sizeOf<IntPtr>()).cast<Pointer<Int8>>();
    try {
      errorSlot.value = Pointer<Int8>.fromAddress(0);
      final result = _normalize(rawStatePtr, errorSlot);
      if (result.address != 0) {
        return _takeBridgeString(result);
      }
      final errorPointer = errorSlot.value;
      throw StateError(
        errorPointer.address == 0
            ? "native bridge normalization failed"
            : _takeBridgeString(errorPointer),
      );
    } finally {
      _free(rawStatePtr.cast<Void>());
      _free(errorSlot.cast<Void>());
    }
  }

  String workspaceCatalogJson() {
    final errorSlot = _malloc(sizeOf<IntPtr>()).cast<Pointer<Int8>>();
    try {
      errorSlot.value = Pointer<Int8>.fromAddress(0);
      final result = _catalog(errorSlot);
      if (result.address != 0) {
        return _takeBridgeString(result);
      }
      final errorPointer = errorSlot.value;
      throw StateError(
        errorPointer.address == 0
            ? "native bridge catalog failed"
            : _takeBridgeString(errorPointer),
      );
    } finally {
      _free(errorSlot.cast<Void>());
    }
  }

  String invokeCrateJson(
    String crateName,
    String operation,
    String payloadJson,
  ) {
    final crateNamePtr = _allocateCString(crateName);
    final operationPtr = _allocateCString(operation);
    final payloadPtr = _allocateCString(payloadJson);
    final errorSlot = _malloc(sizeOf<IntPtr>()).cast<Pointer<Int8>>();
    try {
      errorSlot.value = Pointer<Int8>.fromAddress(0);
      final result = _invoke(crateNamePtr, operationPtr, payloadPtr, errorSlot);
      if (result.address != 0) {
        return _takeBridgeString(result);
      }
      final errorPointer = errorSlot.value;
      throw StateError(
        errorPointer.address == 0
            ? "native bridge invoke failed"
            : _takeBridgeString(errorPointer),
      );
    } finally {
      _free(crateNamePtr.cast<Void>());
      _free(operationPtr.cast<Void>());
      _free(payloadPtr.cast<Void>());
      _free(errorSlot.cast<Void>());
    }
  }

  /// Reads a Rust-allocated C string and frees it using the **Rust** allocator.
  ///
  /// MUST use `_freeBridgeString` (Rust's free) — this string was allocated
  /// by the Rust bridge via `CString::into_raw`, NOT by the system `malloc`.
  String _takeBridgeString(Pointer<Int8> pointer) {
    final text = _readCString(pointer.cast<Uint8>());
    _freeBridgeString(pointer);
    return text;
  }
}

final DynamicLibrary _systemLibrary = DynamicLibrary.open(_systemLibraryName());
final _MallocDart _malloc =
    _systemLibrary.lookupFunction<_MallocNative, _MallocDart>("malloc");
final _SystemFreeDart _free =
    _systemLibrary.lookupFunction<_SystemFreeNative, _SystemFreeDart>("free");
final _StrlenDart _strlen =
    _systemLibrary.lookupFunction<_StrlenNative, _StrlenDart>("strlen");

Iterable<String> _bridgeCandidates() sync* {
  final name = _bridgeFileName();
  final separator = Platform.pathSeparator;
  if (_configuredLibraryPath case final String configured) {
    yield configured;
  }
  if (Platform.environment["ORCHUSTR_DART_LIBRARY"]
      case final String configuredFromEnv) {
    yield configuredFromEnv;
  }
  yield name;
  yield "${Directory.current.path}${separator}$name";
  yield "${Directory.current.path}${separator}native${separator}$name";
  yield "${Directory.current.path}${separator}..${separator}..${separator}target${separator}debug${separator}$name";
  yield "${Directory.current.path}${separator}..${separator}..${separator}target${separator}release${separator}$name";
}

/// Allocates a null-terminated UTF-8 C string using the system `malloc`.
///
/// The caller is responsible for freeing this with the system `free` (NOT
/// the Rust bridge free function).
Pointer<Int8> _allocateCString(String value) {
  final bytes = utf8.encode(value);
  final pointer = _malloc(bytes.length + 1).cast<Uint8>();
  final data = pointer.asTypedList(bytes.length + 1);
  data.setRange(0, bytes.length, bytes);
  data[bytes.length] = 0;
  return pointer.cast<Int8>();
}

/// Reads a null-terminated C string using `strlen` + bulk `asTypedList`.
///
/// This is O(n) via two passes (strlen + copy) but avoids the overhead of
/// per-byte Dart FFI pointer indexing, which is dramatically slower for
/// large strings.
String _readCString(Pointer<Uint8> pointer) {
  final length = _strlen(pointer);
  if (length == 0) {
    return "";
  }
  return utf8.decode(pointer.asTypedList(length));
}

String _bridgeFileName() {
  if (Platform.isWindows) {
    return "or_bridge.dll";
  }
  if (Platform.isMacOS || Platform.isIOS) {
    return "libor_bridge.dylib";
  }
  return "libor_bridge.so";
}

String _systemLibraryName() {
  if (Platform.isWindows) {
    return "msvcrt.dll";
  }
  if (Platform.isMacOS || Platform.isIOS) {
    return "/usr/lib/libSystem.B.dylib";
  }
  return "libc.so.6";
}
