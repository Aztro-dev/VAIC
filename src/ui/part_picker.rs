use belly::prelude::*;
use bevy::prelude::*;

pub fn spawn_part_picker(mut commands: Commands) {
    let parts = vec!["Cube", "Sphere", "Cone"];
    commands.add(StyleSheet::load("part_picker.ess"));
    commands.add(eml! {
        <body c:body>
            <div c:picker>
                <span c:title>"Part Picker"</span>
                <div c:list>
                    <for part in=parts>
                        <button c:part>
                        {part}
                        </button>
                    </for>
                </div>
            </div>
        </body>
    });
}
