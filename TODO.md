# TODO

Load constraints from meshes (don't link everything as one mesh)

- Leave the constraints separate from the main mesh, and give them a numbered name in blender
- This should allow for future reads of the file to find the constraints and generate constraint data

Handle constraints, and make it customizable with movable window

- Constraints should come with data (orientation, relative position, etc.)
- Movable window should have:
  - A reverse constraint function (like Inventor)
  - The ability to change the first and second constraints (also like Inventor)

Ctrl-Y (redo) functionality

- Ctrl-Y to redo something that you undid

Undo translations/rotations

- Ctrl-Z to undo these thingy mabobs

Load to/from files (save functionality)

- Should contain full (up to like 50 actions) object/action tracking, so you can Ctrl-Z

UI Overhaul

- Could use a crate such as [bevy_blur_regions](https://github.com/atbentley/bevy_blur_regions) to make a cool effect
