# Generative graphics and art with idmage
[![Cargo](https://api.travis-ci.com/rasviitanen/idmage.svg?branch=master)](https://travis-ci.com/rasviitanen/idmage) 
[![Docs](https://img.shields.io/badge/docs-v0.0.1-blue.svg)](https://rasviitanen.github.io/idmage/idmage/index.html)

The latest generated image:
![Figure 1. Ray tracing, the basics](static/test.svg)


#### Todo
- [ ] NURBS representation
- [ ] Affine transforms
- [ ] Size normalization
- [ ] Add light sources
- [ ] Intersection trait
- [ ] Determine values through path tracing
- [ ] Weight system
- [ ] Polarity system
- [ ] Composition expert
- [ ] Text expert
- [ ] Color expert
#### Done
- [X] Expert system architecture
- [X] SVG templating
- [X] Serve the SVG over http
- [X] Generate content from a profile
- [X] Allow for perspective and add camera

## About 
Idmage is a tool for creating graphics by sending a HTTP request. The tool uses a visual identity as well as a smart expert-system do generate its designs. This way, images (such that follows some visual identities) can easily be created with a different look every time, while still following the visual guidelines.

The API enables:
* Creation of graphics that is difficult to design in e.g. Adobe Illustrator, such as fractals.
* Generate unique avatars (like those found on Github or Slack).
* Generate other documents and graphics with a persistent look (such as banners, posters & invoices).
* Animate SVG-files.
* Visualisation of large data-sets in vector format.
* Serve graphics over http.
* ...
