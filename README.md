![Logo: Violet Crystal](https://raw.githubusercontent.com/NaKeRNarolino/violet_crystal/master/public/violet_crystal_logo_1024.png?token=GHSAT0AAAAAACK6OTIUPZXP6HHH6QW22W42ZPHEN2A)

# Violet Crystal is a Rust framework for rust-powered MCBE addon development (Deprecated)

### Available features:
- Pack generation
- Items (all the components are there!)
- Script support (pairing a folder with .js scripts with the pack)
- Recipes
- Blocks\*

\* - Traits and some components are still not available

### Plans:
- Entities;
- Animation / Render controllers;

## Getting Started
Let's start with configuring a new project by creating a new *Pack*.
```Rust
let scripts = Some(ScriptData {
    mc_server_ui_version: "1.2.0-beta".to_string(), // @minecraft/server_ui version
    mc_server_version: "1.11.0-beta".to_string(), // @minecraft/server version
    paired_scripts_folder: r"./src-scripts", // folder from where to get scripts
}); // Script Data. Can be set to None if project doesn't use scripts
let mut pack = Pack::new( // Pack needs to be mutable
    "Violet Crystal".to_string(), // Pack Name
    "violet_crystal".to_string(), // Pack Identifier
    "NaKeR".to_string(), // Pack Author
    "1, 0, 0", // Pack Version. Separated with commas 
    "Nothing here".to_string(), // Pack Description
    true, // Does the project use scripts
    r"*some path*", // Developer BP Folder
    r"*some path*", // Developer RP Folder
    r"*some path*", // Pack Icon
    &scripts, // Script Data which we defined earlier
);
```
To generate the pack we'll use Pack.generate() and Pack.build_to_dev(). Put this in the end of function.
```Rust
// Your code here

// Your code here
pack.generate(None); // Generates to "violet_crystal_results" folder.
pack.build_to_dev(); // Copies to specified dev folders.
```
This will already create a pack. After first generation change *pack.generate(None)* to *pack.generate(Some(false))*. This is done so Violet Crystal does not change the manifest thus the UUIDs.
#### Let's add a new Item!
Use *Pack.register_item()* to add a new item, which requires an *Item* parameter.
```Rust
pack.register_item(Item {
     type_id: Identifier {
         namespace: "vc",
         value: "tutorial"
     }, // Identifier is used to indicate a namespaced value
     components: vec![
     ], // item components are going here
});
```
This will add new item. But wait... It does not have a texture!
To add a texture we first need to register it with *Pack.register_item_texture()* which requires an ItemAtlasEntry parameter.
```Rust
pack.register_item_texture(ItemAtlasEntry {
    id: "vc_test".to_string(), // Id to use in Item Icon component
    texture_name: "vc_test".to_string(), // Name to use in file system
    path: r"*some path*".to_string(), // Path to texture on your PC
});
```
Then let's add it to the item. Remember the components field?
```Rust
pack.register_item(Item {
     type_id: Identifier {
         namespace: "vc",
         value: "tutorial"
     }, // Identifier is used to indicate a namespaced value
     components: vec![
        &ItemIconComponent {     
            texture: "vc_test", // texture id we've specified earlier
        },                       
     ], // item components are going here
});
```
This will add the texture to item. All item components are there. I won't document all of them. You can see the docs for them on official MC docs.
#### Blocks
Adding blocks is also done in similar way. Currently, in Beta 1 you *don't* need to register texture, just specify the path in *texture_set* field. It might be changed to a way like items are done for consistency.
Example of a block
```Rust
pack.register_block(Block {                                                                                                                                                                                                                                  
    type_id: Identifier {                                                                                                                                                                                                                                    
        namespace: "vc",                                                                                                                                                                                                                                   
        value: "test"                                                                                                                                                                                                                                        
    },                                                                                                                                                                                                                                                       
    components: vec![                                                                                                                                                                                                                                        
        &collision_box,                                                                                                                                                                                                                                      
        &crafting_table_component,                                                                                                                                                                                                                           
        &BlockDestructibleByExplosionComponent {                                                                                                                                                                                                             
            explosion_resistance: Some(1.0)                                                                                                                                                                                                                  
        },                                                                                                                                                                                                                                                   
        &BlockDestructibleByMiningComponent {                                                                                                                                                                                                                
            seconds_to_destroy: Some(10.0)                                                                                                                                                                                                                   
        },                                                                                                                                                                                                                                                   
        &flammable,                                                                                                                                                                                                                                          
        &BlockFrictionComponent {                                                                                                                                                                                                                            
            friction: 0.7,                                                                                                                                                                                                                                   
        },                                                                                                                                                                                                                                                   
    ],                                                                                                                                                                                                                                                       
    texture_set: r"*path here*".to_string(),                                                                                                                                                 
    sound: "stone".to_string(),                                                                                                                                                                                                                              
    permutations: vec![                                                                                                                                                                                                                                      
        BlockPermutation {                                                                                                                                                                                                                                   
            condition: "q.block_state('amex:test') == true",                                                                                                                                                                                                 
            components: vec![                                                                                                                                                                                                                                
                &BlockFrictionComponent {                                                                                                                                                                                                                    
                    friction: 0.1,                                                                                                                                                                                                                           
                }                                                                                                                                                                                                                                            
            ]                                                                                                                                                                                                                                                
        }                                                                                                                                                                                                                                                    
    ],                                                                                                                                                                                                                                                       
    states: vec![                                                                                                                                                                                                                                            
        &BoolBlockState {                                                                                                                                                                                                                                    
            id: Identifier {                                                                                                                                                                                                                                 
                namespace: "amex",                                                                                                                                                                                                                           
                value: "test"                                                                                                                                                                                                                                
            }                                                                                                                                                                                                                                                
        }                                                                                                                                                                                                                                                    
    ]                                                                                                                                                                                                                                                        
});                                                                                                                                                                                                                                                          
```
With this knowledge you're good to go!
