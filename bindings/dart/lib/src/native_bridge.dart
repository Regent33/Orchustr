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
typedef _BridgeFreeNative = Void Function(Pointer<Int8>);
typedef _BridgeFreeDart = void Function(Pointer<Int8>);
typedef _MallocNative = Pointer<Void> Function(IntPtr);
typedef _MallocDart = Pointer<Void> Function(int);
typedef _SystemFreeNative = Void Function(Pointer<Void>);
typedef _SystemFreeDart = void Function(Pointer<Void>);

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
        _freeBridgeString =
            library.lookupFunction<_BridgeFreeNative, _BridgeFreeDart>(
          "orchustr_bridge_free_string",
        );

  final _RustStringFn _version;
  final _RenderDart _render;
  final _NormalizeDart _normalize;
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

Pointer<Int8> _allocateCString(String value) {
  final bytes = utf8.encode(value);
  final pointer = _malloc(bytes.length + 1).cast<Uint8>();
  final data = pointer.asTypedList(bytes.length + 1);
  data.setRange(0, bytes.length, bytes);
  data[bytes.length] = 0;
  return pointer.cast<Int8>();
}

String _readCString(Pointer<Uint8> pointer) {
  final bytes = <int>[];
  for (var index = 0; true; index += 1) {
    final value = pointer[index];
    if (value == 0) {
      return utf8.decode(bytes);
    }
    bytes.add(value);
  }
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
