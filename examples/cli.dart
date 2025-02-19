import 'package:apk_parser/src/rust/api.dart';
import 'package:apk_parser/src/rust/frb_generated.dart';

Future<void> main(List<String> args) async {
  await RustLib.init();
  final parser = ApkParser(path: args[0]);
  final manifest = parser.loadManifest();

  print("id=${manifest.package}\nmin_sdk_version=${manifest.minSdkVersion}");
}
