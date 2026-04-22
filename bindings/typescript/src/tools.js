import { RustCrateBridge } from "./bridge.js";

export class SearchTools {
  static search(provider, query, config = {}) {
    return RustCrateBridge.invoke("or-tools-search", "search", {
      provider,
      query,
      config,
    });
  }
}

export class WebTools {
  static fetch(provider, request, config = {}) {
    return RustCrateBridge.invoke("or-tools-web", "fetch", {
      provider,
      request,
      config,
    });
  }

  static scrape(provider, url, config = {}) {
    return RustCrateBridge.invoke("or-tools-web", "scrape", {
      provider,
      url,
      config,
    });
  }
}

export class VectorTools {
  static ensureCollection(provider, data, config = {}) {
    return RustCrateBridge.invoke("or-tools-vector", "ensure_collection", {
      provider,
      data,
      config,
    });
  }

  static upsert(provider, data, config = {}) {
    return RustCrateBridge.invoke("or-tools-vector", "upsert", {
      provider,
      data,
      config,
    });
  }

  static delete(provider, data, config = {}) {
    return RustCrateBridge.invoke("or-tools-vector", "delete", {
      provider,
      data,
      config,
    });
  }

  static query(provider, data, config = {}) {
    return RustCrateBridge.invoke("or-tools-vector", "query", {
      provider,
      data,
      config,
    });
  }
}

export class LoaderTools {
  static load(request) {
    return RustCrateBridge.invoke("or-tools-loaders", "load", { request });
  }
}

export class ExecTools {
  static execute(request, providers = ["python", "shell"], config = {}) {
    return RustCrateBridge.invoke("or-tools-exec", "execute", {
      request,
      providers,
      config,
    });
  }
}

export class FileTools {
  static read(path, provider = "local", config = {}) {
    return RustCrateBridge.invoke("or-tools-file", "read", {
      provider,
      path,
      config,
    });
  }

  static write(path, content, provider = "local", config = {}) {
    return RustCrateBridge.invoke("or-tools-file", "write", {
      provider,
      path,
      content,
      config,
    });
  }

  static list(path, provider = "local", config = {}) {
    return RustCrateBridge.invoke("or-tools-file", "list", {
      provider,
      path,
      config,
    });
  }

  static delete(path, provider = "local", config = {}) {
    return RustCrateBridge.invoke("or-tools-file", "delete", {
      provider,
      path,
      config,
    });
  }

  static fetch(provider, query, config = {}) {
    return RustCrateBridge.invoke("or-tools-file", "fetch", {
      provider,
      query,
      config,
    });
  }
}

export class CommsTools {
  static send(provider, to, body, from = null, config = {}) {
    return RustCrateBridge.invoke("or-tools-comms", "send", {
      provider,
      to,
      body,
      from,
      config,
    });
  }
}

export class ProductivityTools {
  static listEmails(provider, query = {}, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_emails", {
      provider,
      query,
      config,
    });
  }

  static sendEmail(provider, item, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "send_email", {
      provider,
      item,
      config,
    });
  }

  static listEvents(provider, query = {}, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_events", {
      provider,
      query,
      config,
    });
  }

  static createEvent(provider, item, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_event", {
      provider,
      item,
      config,
    });
  }

  static listIssues(provider, query = {}, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "list_issues", {
      provider,
      query,
      config,
    });
  }

  static createIssue(provider, item, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_issue", {
      provider,
      item,
      config,
    });
  }

  static searchPages(provider, query = {}, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "search_pages", {
      provider,
      query,
      config,
    });
  }

  static createPage(provider, item, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "create_page", {
      provider,
      item,
      config,
    });
  }

  static postMessage(provider, channel, text, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "post_message", {
      provider,
      channel,
      text,
      config,
    });
  }

  static searchMessages(provider, query = {}, config = {}) {
    return RustCrateBridge.invoke("or-tools-productivity", "search_messages", {
      provider,
      query,
      config,
    });
  }
}
