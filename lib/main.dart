import 'package:apk_parser/src/rust/api.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:apk_parser/src/rust/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('apk_parser test')),
        body: Center(
          child: ApkInfo(),
        ),
      ),
    );
  }
}

class ApkInfo extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _ApkInfo();
}

class _ApkInfo extends State<ApkInfo> {
  AndroidManifestParsed? parsed;
  String? error;

  Future<void> loadManifest() async {
    try {
      setState(() {
        error = null;
        parsed = null;
      });
      final result = await FilePicker.platform.pickFiles();
      if (result != null) {
        final parser = ApkParser(path: result.files.first.path!);
        setState(() {
          parsed = parser.loadManifest();
        });
      }
    } catch (e) {
      setState(() {
        if (e is AnyhowException) {
          error = e.message;
        } else {
          error = e.toString();
        }
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ElevatedButton(
          onPressed: () {
            loadManifest();
          },
          child: Text("Load APK"),
        ),
        if (parsed != null)
          Column(
            children: [
              Text("package=${parsed!.package}"),
              Text("target_sdk_version=${parsed!.targetSdkVersion}"),
              Text("min_sdk_version=${parsed!.minSdkVersion}"),
              Text("compile_sdk_version=${parsed!.compileSdkVersion}"),
              Text(
                  "compile_sdk_version_codename=${parsed!.compileSdkVersionCodename}"),
              ...parsed!.sigs
                  .map((s) => Text("${s.algo}: ${s.signature.length}"))
            ],
          ),
        if (error != null) Text(error!)
      ],
    );
  }
}
