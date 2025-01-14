#[cfg(test)]
mod tests {
    use item_component_macros::item_component;

    item_component! {
        name = Custom for "minecraft:component_id";
        id has String for "minecraft:id";
        number has i32 for "minecraft:number";
    }

    #[test]
    pub fn test() {
        dbg!(ItemCustomComponent {
            id: "hello".to_string(),
            number: 10
        });
    }
}