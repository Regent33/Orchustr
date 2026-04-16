import "dart:io";

Future<void> main(List<String> args) async {
  final release = args.contains("--release");
  final packageRoot = _packageRoot();
  final repoRoot = packageRoot.parent.parent;
  final profile = release ? "release" : "debug";

  final build = await Process.run(
    "cargo",
    <String>[
      "build",
      "-p",
      "or-bridge",
      "--features",
      "dart",
      if (release) "--release",
    ],
    workingDirectory: repoRoot.path,
    runInShell: true,
  );

  stdout.write(build.stdout);
  stderr.write(build.stderr);
  if (build.exitCode != 0) {
    exit(build.exitCode);
  }

  final separator = Platform.pathSeparator;
  final libraryName = _libraryName();
  final builtLibrary = File(
      "${repoRoot.path}${separator}target${separator}$profile${separator}$libraryName");
  if (!builtLibrary.existsSync()) {
    stderr.writeln("native bridge artifact not found: ${builtLibrary.path}");
    exit(1);
  }

  final nativeDirectory = Directory("${packageRoot.path}${separator}native");
  nativeDirectory.createSync(recursive: true);
  final destination = File("${nativeDirectory.path}${separator}$libraryName");
  if (destination.existsSync()) {
    destination.deleteSync();
  }
  final copiedLibrary = builtLibrary.copySync(destination.path);

  stdout.writeln("Copied native bridge to ${copiedLibrary.path}");
  stdout.writeln(
      "Dart bindings will auto-detect this library from the package root.");
}

Directory _packageRoot() {
  final current = Directory.current.absolute;
  if (File("${current.path}${Platform.pathSeparator}pubspec.yaml")
      .existsSync()) {
    return current;
  }

  final nested = Directory(
    "${current.path}${Platform.pathSeparator}bindings${Platform.pathSeparator}dart",
  );
  if (File("${nested.path}${Platform.pathSeparator}pubspec.yaml")
      .existsSync()) {
    return nested;
  }

  throw StateError("run this script from bindings/dart or the repository root");
}

String _libraryName() {
  if (Platform.isWindows) {
    return "or_bridge.dll";
  }
  if (Platform.isMacOS || Platform.isIOS) {
    return "libor_bridge.dylib";
  }
  return "libor_bridge.so";
}
