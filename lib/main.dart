import 'dart:io';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'bridge/ffi.dart';
import 'app.dart';
import 'value.dart';

var viewUpdateStream = api.startAndGetViewUpdateStream();
var viewUpdateBroadcaster = StreamController<ViewUpdateDetail>.broadcast();

void main() async {
  // Debug mode code
  assert(() {
    debugPrint('CWD ${Directory.current.path}');
    return true;
  }());

  // Prepare view update broadcasting
  viewUpdateStream.listen((event) {
    viewUpdateBroadcaster.add(event);
  });

  // Initialization of packages
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

  // Run everything
  runApp(
    EasyLocalization(
      supportedLocales: const [
        Locale('en', 'US'),
        Locale('ko', 'KR'),
      ],
      path: 'assets/translations',
      assetLoader: YamlAssetLoader(),
      fallbackLocale: const Locale('en', 'US'),
      child: const App(),
    ),
  );

  // Set desktop window shape
  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    doWhenWindowReady(() {
      appWindow.minSize = minimumSize;
      appWindow.size = initialSize;
      appWindow.alignment = Alignment.center;
      appWindow.show();
    });
  }
}
