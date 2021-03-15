import 'package:shared_preferences/shared_preferences.dart';

class AppPreferences {
  static Future<String> accessToken() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString("access_token");
  }
}
