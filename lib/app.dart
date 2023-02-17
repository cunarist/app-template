import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:easy_localization/easy_localization.dart';
import 'value.dart';
import 'bridge/ffi.dart';
import 'dart:convert';
import 'main.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    ThemeMode themeMode = ThemeMode.system;

    // Debug mode code
    assert(() {
      // assert statement gets removed in release mode
      String debugLocale = dotenv.env['DEBUG_LOCALE'] ?? '';
      switch (debugLocale) {
        case '':
          break;
        default:
          List splitted = debugLocale.split('-');
          context.setLocale(Locale(splitted[0], splitted[1]));
      }
      String darkMode = dotenv.env['DARK_MODE'] ?? '';
      switch (darkMode) {
        case 'true':
          themeMode = ThemeMode.dark;
          break;
        case 'false':
          themeMode = ThemeMode.light;
          break;
      }
      return true;
    }());

    // Return the actual app structure
    return MaterialApp(
      onGenerateTitle: (context) {
        return 'appTitle'.tr();
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
      themeMode: themeMode,
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
            StreamBuilder<ViewUpdateDetail>(
              stream: viewUpdateBroadcaster.stream.where((detail) {
                return detail.dataAddress == 'someDataCategory.count';
              }),
              builder: (context, snapshot) {
                if (snapshot.hasData) {
                  String jsonString = utf8.decode(snapshot.data!.bytes);
                  Map jsonObject = json.decode(jsonString);
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
