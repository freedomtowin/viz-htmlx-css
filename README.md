# viz-htmlx-css

### Introduction

I am interested in building a fullstack blog website with Rust, and viz-rs seemed like a good package for me to do this and to start learning Rust.

> Viz - Fast, robust, flexible, lightweight web framework for Rust

I started with the [htmlx example](https://github.com/viz-rs/viz/tree/main/examples/htmlx)
and was able to run it with a few local fixes. This example creates a TODO list that can be edited in the brower (without CSS).

The code was modified to load in a blog post template website with css. I used the [static-files/serve example] (https://github.com/viz-rs/viz/tree/0.4.x/examples/static-files/serve) to get an example for how I might allow the formatted .html will load the static CSS files within the viz-rs Router. Below shows a blog post example.

```
<!DOCTYPE html>
<html>
<head>
    <title>{{post.title}}</title>
    <link rel="stylesheet" type="text/css" href="/static/styles.css">
</head>
<body>
    <header>
        <h1>{{post.title}}</h1>
    </header>
    <div class="container">
        <p>{{post.content}}</p>
        <a href="/">Back to Home</a>
    </div>
</body>
</html>
```

Note that this examples uses `handlebars`, a [template engine](https://viz.rs/en/0.4.x/extra-topics/templates) for formatting HTML.


Surprisingly, I didn't have to pass the data into the `handlebars` render function. The `Router` the GET call seems to automatically add the static files path to the .html files, i.e., 

```
    let app = Router::new()
        .get("/", index)
        .get("/posts/:id", show_post)
        .post("/posts", create)
        .get("/static/styles.css", serve::File::new(dir.join("static/styles.css")))
        .with(State::new(state))
        .with(limits::Config::default());
```

### Run it

To start the web server, you will need to run the following cargo command. Notice the double dash before `--nocapture`.

`cargo run -- --nocapture`


### Fetch data

You can interact with you API/Web App through curl or by opening a browser.

`curl http://127.0.0.1:3000`

### Thoughts

The compile time was very very fast for a Rust application. I noticed that using other template engines like `tera` made the compile time significantly slower though. Overall I'm happy with the package and give it a 7/10 due to lack of example documentation. 

I might create a pull request to add this example to the library's GitHub repo. I also might end up deploying a full-stack blog website soon too!