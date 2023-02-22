import 'dart:typed_data';
import 'dart:convert';
import 'ffi.dart';

dynamic readViewmodelAsJson(String dataAddress) {
  Uint8List? bytes = api.readViewmodel(
    dataAddress: dataAddress,
    takeOwnership: false,
  );
  dynamic jsonValue;
  if (bytes != null) {
    String jsonString = utf8.decode(bytes);
    jsonValue = json.decode(jsonString);
  } else {
    jsonValue = null;
  }
  return jsonValue;
}

Uint8List? readViewmodelAsBytes(String dataAddress,
    [bool takeOwnership = false]) {
  Uint8List? bytes = api.readViewmodel(
    dataAddress: dataAddress,
    takeOwnership: takeOwnership,
  );
  return bytes;
}

void sendUserAction(String taskAddress, dynamic jsonValue) {
  String jsonString = json.encode(jsonValue);
  api.sendUserAction(taskAddress: taskAddress, jsonString: jsonString);
}
