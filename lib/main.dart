import 'dart:io';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'app.dart';
import 'value.dart';
import 'bridge/ffi.dart';

var viewUpdateStream = api.createConnection();
var viewUpdateBroadcaster = StreamController<ViewUpdateDetail>.broadcast();

void main() async {
  // Debug mode code
  assert(() {
    // Assert statement gets removed in release mode
    debugPrint('CWD ${Directory.current.path}');
    dotenv.testLoad(fileInput: File('.env').readAsStringSync());
    dotenv.env.forEach((k, v) => debugPrint('ENV $k $v'));
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
  doWhenWindowReady(() {
    appWindow.title = 'appTitle'.tr();
    appWindow.minSize = minimumSize;
    appWindow.size = initialSize;
    appWindow.alignment = Alignment.center;
    appWindow.show();
  });
}
