# VAIC
VAIC, short for **V**ex (**A**nti) **I**nnovation **C**reator (name subject to change) is a Free and Open Source CAD (Computer Aided Design) program designed to succeed [Protobot](https://protobot.web.app), which isn't Open Source and seems to be abandoned (we still love you Dave!)

## Installation Guide
VAIC is built in [Rust](https://www.rust-lang.org), and therefore can be installed via [rustup](https://www.rust-lang.org/tools/install).
After following the guide to install Rust, download the source code from this repository by clicking "Code" => "Download ZIP" and unzip the file wherever you want to install VAIC, or can be downloaded using the [Git CLI](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) by first navigating to the directory where you want to install VAIC and running
```bash
git clone https://github.com/Aztro-dev/VAIC.git
```
After cloning the repository, go into the repository in the terminal and run
```bash
cargo run
```
Which will compile and run VAIC in [debug mode](https://nnethercote.github.io/perf-book/build-configuration.html#:~:text=This%20output%20indicates%20that%20a,checks%2C%20and%20omit%20debug%20info.) 

Please note that upon first loading the program, you might not see anything useful, and this is because VAIC launches in a placeholder "Main Menu" state, which is a WIP. To actually go into the editor, **please press `Escape`**

## Controls
**P** to toggle between movement modes
* "PC-Mode"
	* `Left Shift` + `Middle Mouse Button` to orbit (rotate around the center)
	* `Middle Mouse Button` to pan (move without rotating)
	* `Scroll Wheel Up/Down` to zoom
* "Laptop Mode"
	* `Left Shift` to orbit  (rotate around the center)
	* `Left Control` to pan (move without rotating)
	* `Trackpad Scroll Up/Down` to zoom

**O** to toggle between projections
* `Perspective` - "Normal" projection
* `Orthographic` - This is kinda hard to explain (WIP)

**C** to enable constraints (WIP)
* Constraints are essentially "connections" in CAD
* You constrain an object to another by clicking on a hole on one objects, and clicking on the hole you would like to connect it to on another object
* This is still a WIP, so undesired behavior is common (basically guaranteed)

**T** to toggle between moving states
* `Translation` - Arrows and squares will appear which you can drag on to translate the object.
* `Rotation` - An assortment of circles will appear which you can drag on to rotate the object.

**Left-Shift** or **L**to remove snapping
* Use `Left-Shift` in `PC-Mode` and `L` in `Laptop Mode`
* You must use this keybinding to remove default snapping of 15 degrees for rotations and 0.2 units for translations.

**Ctrl-Z** to undo a move (WIP-kinda)
* If you made a place or constrain that you didn't intend for, press `Ctrl-Z` to undo the action.

**F12** to take a screenshot
* Automatically saved as `screenshot-{x}.png`

## Contributor's guide
If you are requesting a feature or outlining a bug, make an [issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/creating-an-issue), and make a comment requesting the appropriate [label](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels)

If you want to contribute directly to the source code, [fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo) the repository, make your changes in the repository that is created, and then make a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests)
