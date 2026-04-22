try:
    from . import _orchustr as native_bridge
except Exception:
    native_bridge = None


def _native_class(name: str):
    if native_bridge is None:
        return None
    return getattr(native_bridge, name, None)


PyColonyBuilder = _native_class("PyColonyBuilder")
PyConduitProvider = _native_class("PyConduitProvider")
PyDynState = _native_class("PyDynState")
PyExecutionGraph = _native_class("PyExecutionGraph")
PyForgeRegistry = _native_class("PyForgeRegistry")
PyGraphBuilder = _native_class("PyGraphBuilder")
PyNodeResult = _native_class("PyNodeResult")
PyPipeline = _native_class("PyPipeline")
PyPipelineBuilder = _native_class("PyPipelineBuilder")
PyPromptBuilder = _native_class("PyPromptBuilder")
PyPromptTemplate = _native_class("PyPromptTemplate")
PyRelayBuilder = _native_class("PyRelayBuilder")
PyRelayPlan = _native_class("PyRelayPlan")


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
