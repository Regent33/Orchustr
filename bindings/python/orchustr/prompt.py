from __future__ import annotations

import json
import re
from dataclasses import dataclass

from ._runtime import render_prompt_native


def _sanitize(value: str) -> str:
    return "".join(character for character in value if character >= " " or character in "\n\t")


@dataclass
class PromptTemplate:
    template: str

    def render(self, context: dict) -> str:
        native = render_prompt_native(self.template, json.dumps(context))
        if native is not None:
            return native
        rendered = self.template
        for variable in re.findall(r"{{([A-Za-z0-9_]+)}}", self.template):
            if variable not in context:
                raise ValueError(f"missing variable: {variable}")
            rendered = rendered.replace(f"{{{{{variable}}}}}", _sanitize(str(context[variable])))
        return rendered


class PromptBuilder:
    def __init__(self) -> None:
        self._template: str | None = None

    def template(self, template: str) -> "PromptBuilder":
        self._template = _sanitize(template)
        return self

    def build(self) -> PromptTemplate:
        if not self._template:
            raise ValueError("template must not be empty")
        return PromptTemplate(self._template)
