from dataclasses import dataclass
from datetime import datetime
from typing import Optional
import markdown
import yaml
import os


class PostMeta:
    def __init__(
        self,
        title: str,
        description: str,
        date: str,
        tags: list[str],
        image: Optional[str] = None,
    ):
        self.title = title
        self.description = description
        self.date = datetime.strptime(date, "%Y-%m-%d")
        self.tags = tags
        self.image = image


@dataclass
class Post:
    slug: str
    content: str
    meta: PostMeta

    @classmethod
    def load(cls, path: str):
        with open(path, "r", encoding="utf-8") as file:
            content = file.read()

        parts = content.split("---\n")
        if len(parts) < 3:
            raise ValueError("Invalid markdown format: missing frontmatter")

        frontmatter = yaml.safe_load(parts[1])
        meta = PostMeta(
            title=frontmatter["title"],
            description=frontmatter["description"],
            date=frontmatter["date"],
            tags=frontmatter["tags"],
            image=frontmatter.get("image"),
        )

        markdown_content = "---\n".join(parts[2:])
        slug = os.path.splitext(os.path.basename(path))[0]

        return cls(slug, markdown_content, meta)

    def render_content(self) -> str:
        return markdown.markdown(
            self.content, extensions=["fenced_code", "tables", "codehilite"]
        )
