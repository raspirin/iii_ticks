import 'package:flutter/material.dart';
import 'package:iii_ticks/messages/main.pb.dart';
import 'package:iii_ticks/simple.dart';
import './messages/generated.dart';

void main() async {
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
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
}
