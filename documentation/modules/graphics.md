< **[Back to documentation main](../documentation.md)**
___

# *Graphics*

## Description
Graphics is the base level module that handles visual rendering of data. Want to draw a triangle? Done. Want to render a complex 3D scenes? It can do that too. And it can do it in both the web and natively. This module like, all other cosmos modules supports realtime collaboration, as well as disconnected workflows.
Inspiration: figma / blender / unreal / clarisse / inventor

## Color

Color may be sononomous with graphics. In either the lack thereof or implementation of it it's involved. Cosmos will support a wide array of color models and will be able to swicth / convert colors between them. Here are color models Cosmos will support by default:

- RGB
  - RGB+A
- HSV
  - HSV+A
- CMYK
  - CMYK+A
- HEX
  - HEX+A

Notice that each color model can have an associated alpha (transparency) value associated with it.

In implementing colors we'll need to create supports around dealing with them. Things that may help with this could be:

- Libraries / Themes
- Sampling
- Mixing

Colors will have a potential structure in them. In that more complex representation and mixing of colors will reference base colors for example:

- Gradients
  - Radial
  - Linear
- Maps
  - Noise
  - Bitmap
  - Vector

Fills or strokes will reference those base colors or more complex representations of colors. Let's definitely look into color libraries and what they can do. For instance SASS has great functions when it comes to color. Looking into what those can do will give us a good foundation for what we can build out for Cosmos and expose powerful tools both in code and visually.