import 'package:flutter/material.dart';

class AuthScreen extends StatefulWidget {
  static final String ROUTE = "/auth-screen";

  AuthScreen({Key key}) : super(key: key);

  @override
  _AuthScreenState createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Text("Authentication"),
    );
  }
}
