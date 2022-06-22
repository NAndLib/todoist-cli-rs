# Design for a todoist CLI built for plugins written in Rust

## Introduction

I had a [previous project](https://github.com/NAndLib/todoist-plugable-cli/blob/master/docs/design-doc.md) aiming at doing the same thing but in Python. The
project was unsuccessful due to a few reasons:
- Real life got in the way.
- Python was slow.

The first reason is really the primary reason. This is a rethink and rewrite of
what I intially had in mind.

I chose Rust because I wanted to learn it.

### Todoist

[Todoist](https://todoist.com/) is a task manager and orgnazier created by Doist. It provides an
abundance of features to help manage and organize tasks such as projects,
labels, priority levels, and so on. I use it extensively for every day tassk
from anywhere to chores to work.

### The API

The Doist team does not have a Rust API for their Sync API. Fortunately, a
person by the name of [IanS5](https://docs.rs/releases/IanS5) has done the
hardest part of wrapping their API into a nice crate.

The Rust API provided by IanS5 will be the primary way this project will be
interacting with Todoist.

### Existing Technology
The current biggest todoist CLI tool available right now is [sachaos/todoist](https://github.com/sachaos/todoist). The
app provides implemtation for all basic Todoist functionality. I would highly
recommend giving it a go.

### Goals

This project has several goals:
1. Learn Rust.
2. Learn parallel programming using Rust.
3. Learn a different pattern in programming, namely, **Plugin
   interfaces**
4. Have CLI app that I wrote myself.
5. Have a CLI app that, if needed, can be extended in a modular way.

The 6th point is really the dream.
