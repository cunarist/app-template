import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'value.dart';
import 'bridge/ffi.dart';
import 'dart:convert';
import 'main.dart';
import 'dart:io';
import 'package:bitsdojo_window/bitsdojo_window.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    context.setLocale(const Locale('en', 'US'));

    // Return the actual app structure
    return MaterialApp(
      onGenerateTitle: (context) {
        if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
          appWindow.title = 'appTitle'.tr(); // For desktop
        }
        return 'appTitle'.tr(); // For mobile and web
      },
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      darkTheme: ThemeData(
        colorScheme: const ColorScheme.dark(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      themeMode: ThemeMode.system,
      home: const HomePage(),
      localizationsDelegates: context.localizationDelegates,
      supportedLocales: context.supportedLocales,
      locale: context.locale,
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            StreamBuilder<String>(
              stream: viewUpdateBroadcaster.stream.where((dataAddress) {
                return dataAddress == 'someDataCategory.count';
              }),
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  Uint8List? bytes = api.readViewmodel(
                    dataAddress: 'someDataCategory.count',
                  );
                  Map jsonObject;
                  if (bytes != null) {
                    String jsonString = utf8.decode(bytes);
                    jsonObject = json.decode(jsonString);
                  } else {
                    jsonObject = {'value': '??'};
                  }
                  return Text('counter.informationText'.tr(
                      namedArgs: {'theValue': jsonObject['value'].toString()}));
                } else {
                  return Text('counter.blankText'.tr());
                }
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Map jsonObject = {'before': 77};
          String jsonString = json.encode(jsonObject);
          api.passUserAction(
            taskAddress: 'someTaskCategory.addOne',
            jsonString: jsonString,
          );
          api.passUserAction(
            taskAddress: 'someTaskCategory.multiplyTwo',
            jsonString: jsonString,
          );
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
