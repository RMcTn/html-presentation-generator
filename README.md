### What is it
This is a simple HTML presentation generator.  
Run the generator with a valid file, and you'll get a set of html pages that are linked to each other, with an ending slide.  
Left and right arrows navigate between slides.

### Why?
Inspired by [Sean Barrett](https://www.youtube.com/watch?v=eAhWIO1Ra6M), it's an exercise of building simple, quick to use tools that are a bit useful for fun.

### How to use
See the [example file](example/example.txt) for what the generator file will look like.  
See the [example presentation](example/page_1.html) for the goods.  
See [format](#format) for a spec.

Make sure there are the `styles.css` and `navigate.js` files in the directory that the program was run in.  
You can edit the `styles.css` file all you like, but the HTML tags used in the output body are:
- `<li>`
- `<ul>`
- `<a>`
- `<div>`
- `<br>`
- `<h1>`

The presentation files will be created in the output/ folder that the program was run in.
```
cargo run <path-to-presentation-file>
```

### Syntax
- A blank line between text starts a new 'slide'.  
- Text between blank lines will be bullet points in the 'slide'.  
- A newline after text starts a new bullet point in the 'slide'.

