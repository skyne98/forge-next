import 'package:flutter/material.dart';
import 'package:forge/screens/auth_screen.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Forge',
      theme: ThemeData(
        visualDensity: VisualDensity.adaptivePlatformDensity,
      ),
      initialRoute: AuthScreen.ROUTE,
      routes: {
        AuthScreen.ROUTE: (context) => AuthScreen(),
      },
    );
  }
}
