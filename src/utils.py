import os
from models import Post
from flask import Flask


def load_posts(app: Flask) -> list[Post]:
    posts_dir = app.config["POSTS_DIR"]
    posts = []

    for filename in os.listdir(posts_dir):
        if filename.endswith(".md"):
            path = os.path.join(posts_dir, filename)
            post = Post.load(path)
            posts.append(post)

    posts.sort(key=lambda p: p.meta.date, reverse=True)
    return posts
