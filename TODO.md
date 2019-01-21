# Todo list

## Misc Work

* Find way to get binaryen wasm-opt into travis build
  - I think it wouldn't be to bad to just build it
  - Then add release build as well as debug build

* Improve structure of web-client artifacts
  - Have a `webresources` directory perhaps,
  - Have `build.sh` copy relevant artifacts into an appropriate target directory

## Interface Work

* Figure out how to do a color picker
  - Hook that up to various debug drawing things
  - Maybe hook up to background too?
  - Thing to think about 

* Try some other layouts
  - I think the plotters should try to take up the whole screen?
  - I think most controls can live in expandable window?
  - Look at better equation entry systems
    - I think something minimal, like a line where your text goes over would be ideal
    - Need to cleanly integrate parsing errors
  - Perhaps look at using KaTeX to render math pretty like

* Explore tradeoff of where interface structure is built:
  - between using index.html to provide structure
  - and web-client building it out
  - On one hand, I think compromise of allowing index.html to define structure, but giving
    web-client ability to add what it needs if not defined is good
  - On the other hand that sounds like a testing nightmare

* Look into gating some features on build type
  - Would be good to keep free camera movement option in dev build for example

## Graphics Work

* Figure out how to render in text
  - add simple gnoman for starters
  - work on graph axis drawing?
  - Ties in with text drawing, but need to figure out good labeling system
  - Seems like we can do it in canvas or on top of with html
  - maybe we want to do it in webGL if possible?

* Work on better line drawing shader system
  - I think we'll need to do a system where we pass in four vertices / two triangles
    for every line. Will need to think a bit about how the vertex shader will work
  - This system should also account for the u16 limit on indices.
  - I think this owuld be a good place to apply the builder pattern
    - That way the constructor could consume desired line segments and internally
      decide how many buffers are needed / etc. 
  - One of the tricks I think, is making each line's "face" orthogonal to the camera
    - I think just passing in the camera's relative position should be sufficient for
      this per-vertex computation?

* Once meshing works, start playing with how to render meshes.
  - I think the simple, grid lines + height color would be a good start
  - Some simple light and shading would go a long way too
    - I do not like graphs with "shiny" meshes though

* How might screenshots work? 
  - Not a priority for now

## implicit-mesh Work

* Work on edge / triangle algorithm
  - We are definitley going to need a more intelligent algorithm here
  - One idea I would like to explore is using a graph-search to find quads,
    then adding triangles from there.
  - However, we also need to consider the 2-d case, where solution might be lines
  - Also needs to support case where there is only one solution
  - How would we support / render 0 = 0, true for all axis variables?

* Work on some autoscaling / bound finding
  - find ideal bounding box, starting at some maximum? 
    - If only inner childer from level 0 -> 2, or only children of some boxes, then we can
      reassign root box and start over?
  - find out how to position camera ideally for bounding box
  - Maybe add in debug only free camera movement?
  - I think the normal build might need to disable zoom in light of this

* Investigate how make work more async friendly. Don't block drawing on meshing
  - threading for webassembly would be a tool, not an answer
  - I think it might look a combination of two things:
    - being able to save / render partial work, i.e. vertices in different solution set scales
    - being to iterate instead of recurse, so that we can iterate while we have time and stop
  - That is to say, how can we work for the time alloted in the desired framerate then stop ourselves?

* I think we can integrate constant sliders
  - When equation is parsed, look at axis variables defined, other variables get auto-populated sliders?
    - hard to know what scale sliders should be?
    - could have user enter bounds... seems gross but I guess it could work
    - I think when we explore here, we need to start thinking about how state could be saved
  - I also think it would be good to explore letting the user define other axis variables for their convenience

* Expand function syntax:
  - Trigonometric functions seem like an obvious next step
    - Would need some extra information about bounding box to make interval arithmetic work
  - Supporting LaTeX syntax would also be worth exploring
  - Can we support mutliple constraints? i.e. x^2 + y^2 = 1, z = 0 ?
  - This would be a earth moving bit of work, but exploring inequalies

* May also be worth exploring a table based parser at some point for better errors

