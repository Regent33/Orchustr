try:
    from . import _orchustr as native_bridge
except Exception:
    native_bridge = None


def render_prompt_native(template: str, context_json: str) -> str | None:
    if native_bridge is None:
        return None
    return native_bridge.render_prompt_json(template, context_json)


def normalize_state_native(state_json: str) -> str | None:
    if native_bridge is None:
        return None
    return native_bridge.normalize_state_json(state_json)


def workspace_catalog_native() -> str | None:
    if native_bridge is None:
        return None
    return native_bridge.workspace_catalog_json()


def invoke_crate_native(crate_name: str, operation: str, payload_json: str) -> str | None:
    if native_bridge is None:
        return None
    return native_bridge.invoke_crate_json(crate_name, operation, payload_json)
