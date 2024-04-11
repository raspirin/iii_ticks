import 'package:iii_ticks/messages/main.pb.dart';
import 'package:path_provider/path_provider.dart';
import './messages/generated.dart';

Future<void> sendPlatformSpecificData() async {
  final path = (await getApplicationSupportDirectory()).path;
  PlatformPathMessage(configPath: path).sendSignalToRust(null);
}
