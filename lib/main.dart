import 'package:flutter/material.dart';
import 'package:iii_ticks/messages/main.pb.dart';
import 'package:iii_ticks/simple.dart';
import './messages/generated.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    sendPlatformPathToRust();

    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(
              seedColor: Colors.blue, brightness: Brightness.dark),
          primaryTextTheme: const TextTheme(
              displaySmall:
                  TextStyle(fontFamily: "MonomaniacOne", fontSize: 12))),
      home: const SimpleMain(),
    );
  }

  void sendPlatformPathToRust() {
    final path = getApplicationSupportDirectory();
    path.then((value) =>
        PlatformPathMessage(configPath: value.path).sendSignalToRust(null));
  }
}
