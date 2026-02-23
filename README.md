# Project Documenter

## Overview

This project is a small tool to help you find your projects.

## Features

- Automatically find projects in specified directories by their ReadMe files.
- Added ReadMe attribute comments give more context to the project list.
  - `<!-- Title: Title of Project -->`
  - `<!-- Description: Short description of the project -->`
  - `<!-- Github: Link to GitHub repository -->`
  - `<!-- Tags: tag1, tag2, tags3 -->`
  - `<!-- Priority: 1 -->`
- Display ReadMe content directly in the app for quick reference.

## Settings

- Project Directories: List of file paths to find projects in.
- Display ReadMe Content: Whether to show the content of the ReadMe file in the app.

## ToDo:

- [ ] Handle multiple projects on backend
- [ ] Folder shortcut integrations (maybe tabbed)
- [ ] Build out settings
- [ ] Add more project attributes (e.g. tags, last updated, etc.)
- [ ] Screenshots

## Stack

- SvelteKit
- Tauri/Rust

<!-- For Project Documenter -->
<!-- Title: Project Documenter -->
<!-- Description: A tool to document and manage your projects. -->
<!-- Github: https://github.com/bannerblack/project-documenter -->
<!-- Tags: Active, Project, Personal, Public -->
