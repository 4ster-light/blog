from flask import Flask, render_template, abort
from utils import load_posts
import os


app = Flask(__name__)
app.config["POSTS_DIR"] = os.path.join(os.path.dirname(__file__), "content/posts")


@app.route("/")
def index():
    posts = load_posts(app)
    return render_template("index.html", posts=posts)


@app.route("/about")
def about():
    return render_template("about.html")


@app.route("/contact")
def contact():
    return render_template("contact.html")


@app.route("/posts/<slug>")
def post(slug):
    posts = load_posts(app)
    post = next((p for p in posts if p.slug == slug), None)
    if not post:
        abort(404)
    return render_template("post.html", post=post)


if __name__ == "__main__":
    app.run(debug=False, port=9112)
