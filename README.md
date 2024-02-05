# Bevy_Viewcube
* BevyTridentAxis: add a custom axis trident
* BevyViewcube:
  * SimpleViewcube: just 6 faces
  * PowerfulViewcube: 
    * 6 faces
    * 12 edges
    * 8 corners

## dependencies
* bevy_mod_picking
* bevy_panorbit_camera

## Usage
```rs
use bevy_viewcube::prelude::*;
```
### Trident
```rs
commands.spawn((
    MaterialMeshBundle {
        mesh: meshes.add((BevyTridentAxis::default()).into()),
        material: materials.add(StandardMaterial::default()),
        ..Default::default()
    },
));
```
also pre-defined
```rs
// 1 unit size
BevyTridentAxis::default()
// 10
BevyTridentAxis::TRIDENT_10
// 100
BevyTridentAxis::TRIDENT_100
```
customize:
```rs
let trident = BevyTridentAxis {
    axises: [
        BevyTridentArrow {...},
        BevyTridentArrow {...},
        BevyTridentArrow {...},
    ]
}
```
### Viewcube
Currently, occupies 0.3x0.4(wh) in the lower left corner of the window.<br>
Need to add dependency crates
```rs
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin
};
use bevy_mod_picking::prelude::*;
```
* Simple
```rs
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(BevyViewCubePlugin::default())
```
* Powerful
<br>replace to
```rs
.add_plugins(BevyViewCubePlugin{use_powerful_viewcube:true})
```

# Version
|bevy |bevy_viewcube |
|---- |------------- |
|0.12 |0.1.0         |