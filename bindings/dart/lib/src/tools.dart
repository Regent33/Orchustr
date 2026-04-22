import "bridge.dart";
import "types.dart";

final class SearchTools {
  static Object? search(String provider, JsonObject query, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-search", "search", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }
}

final class WebTools {
  static Object? fetch(String provider, JsonObject request, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-web", "fetch", <String, Object?>{
      "provider": provider,
      "request": request,
      "config": config,
    });
  }

  static Object? scrape(String provider, String url, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-web", "scrape", <String, Object?>{
      "provider": provider,
      "url": url,
      "config": config,
    });
  }
}

final class VectorTools {
  static Object? ensureCollection(String provider, JsonObject data, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-vector", "ensure_collection", <String, Object?>{
      "provider": provider,
      "data": data,
      "config": config,
    });
  }

  static Object? upsert(String provider, JsonObject data, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-vector", "upsert", <String, Object?>{
      "provider": provider,
      "data": data,
      "config": config,
    });
  }

  static Object? delete(String provider, JsonObject data, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-vector", "delete", <String, Object?>{
      "provider": provider,
      "data": data,
      "config": config,
    });
  }

  static Object? query(String provider, JsonObject data, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-vector", "query", <String, Object?>{
      "provider": provider,
      "data": data,
      "config": config,
    });
  }
}

final class LoaderTools {
  static Object? load(JsonObject request) {
    return RustCrateBridge.invoke("or-tools-loaders", "load", <String, Object?>{
      "request": request,
    });
  }
}

final class ExecTools {
  static Object? execute(
    JsonObject request, {
    List<String> providers = const <String>["python", "shell"],
    JsonObject config = const <String, Object?>{},
  }) {
    return RustCrateBridge.invoke("or-tools-exec", "execute", <String, Object?>{
      "request": request,
      "providers": providers,
      "config": config,
    });
  }
}

final class FileTools {
  static Object? read(String path, {String provider = "local", JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-file", "read", <String, Object?>{
      "provider": provider,
      "path": path,
      "config": config,
    });
  }

  static Object? write(String path, String content, {String provider = "local", JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-file", "write", <String, Object?>{
      "provider": provider,
      "path": path,
      "content": content,
      "config": config,
    });
  }

  static Object? list(String path, {String provider = "local", JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-file", "list", <String, Object?>{
      "provider": provider,
      "path": path,
      "config": config,
    });
  }

  static Object? delete(String path, {String provider = "local", JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-file", "delete", <String, Object?>{
      "provider": provider,
      "path": path,
      "config": config,
    });
  }

  static Object? fetch(String provider, JsonObject query, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-file", "fetch", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }
}

final class CommsTools {
  static Object? send(
    String provider,
    String to,
    String body, {
    String? from,
    JsonObject config = const <String, Object?>{},
  }) {
    return RustCrateBridge.invoke("or-tools-comms", "send", <String, Object?>{
      "provider": provider,
      "to": to,
      "body": body,
      "from": from,
      "config": config,
    });
  }
}

final class ProductivityTools {
  static Object? listEmails(String provider, {JsonObject query = const <String, Object?>{}, JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_emails", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }

  static Object? sendEmail(String provider, JsonObject item, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "send_email", <String, Object?>{
      "provider": provider,
      "item": item,
      "config": config,
    });
  }

  static Object? listEvents(String provider, {JsonObject query = const <String, Object?>{}, JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_events", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }

  static Object? createEvent(String provider, JsonObject item, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_event", <String, Object?>{
      "provider": provider,
      "item": item,
      "config": config,
    });
  }

  static Object? listIssues(String provider, {JsonObject query = const <String, Object?>{}, JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_issues", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }

  static Object? createIssue(String provider, JsonObject item, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_issue", <String, Object?>{
      "provider": provider,
      "item": item,
      "config": config,
    });
  }

  static Object? searchPages(String provider, {JsonObject query = const <String, Object?>{}, JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "search_pages", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }

  static Object? createPage(String provider, JsonObject item, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_page", <String, Object?>{
      "provider": provider,
      "item": item,
      "config": config,
    });
  }

  static Object? postMessage(String provider, String channel, String text, {JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "post_message", <String, Object?>{
      "provider": provider,
      "channel": channel,
      "text": text,
      "config": config,
    });
  }

  static Object? searchMessages(String provider, {JsonObject query = const <String, Object?>{}, JsonObject config = const <String, Object?>{}}) {
    return RustCrateBridge.invoke("or-tools-productivity", "search_messages", <String, Object?>{
      "provider": provider,
      "query": query,
      "config": config,
    });
  }
}
