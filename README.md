# Tacit

Implicit Equations to Meshes

[![Build Status](https://travis-ci.org/SallySoul/tacit.svg?branch=master)](https://travis-ci.org/SallySoul/tacit)

## Introduction

This repo contains several interconnected projects based around taking implicit
equations and generating meshes that approximate their solution.

This project was initially developed as several indipendent repos, but I
the overhead was high, and I'm migrated to a new github account, so I took the
opportunity to move all the projects into one repo.

For now there are two primary ways to use the software. The `expr_to_geom` cli tool
can interactivley generate geometry files that are viewable via the `asap` plotter.

I am also working on a web application in the `web-client` crate.
