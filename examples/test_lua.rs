use std::path::Path;

fn main() -> color_eyre::Result<()> {
    // Initialize the lua component system
    let mut registry = catfoodBar::lua_component::LuaComponentRegistry::new();

    // Load the lua clock component
    if Path::new("examples/lua_clock.lua").exists() {
        registry.load_component("lua_clock", "examples/lua_clock.lua")?;
        println!("Successfully loaded lua_clock component");

        // Test rendering
        if let Some(component) = registry.get_component("lua_clock") {
            let spans = component.render_as_spans_with_colorize(true);
            println!("Rendered with color: {:?}", spans[0].content);

            let spans_no_color = component.render_as_spans_with_colorize(false);
            println!("Rendered without color: {:?}", spans_no_color[0].content);
        }
    } else {
        println!("lua_clock.lua not found in examples/");
    }

    Ok(())
}
